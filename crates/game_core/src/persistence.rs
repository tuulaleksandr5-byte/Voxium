//! Versioned, checked world saves independent of the renderer and application.

use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeSet, HashSet},
    fs::{self, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

/// Increase when the save schema changes, with an explicit migration before loading it.
pub const CURRENT_FORMAT_VERSION: u32 = 2;
/// Generation must remain deterministic for every supported generator version.
pub const CURRENT_GENERATOR_VERSION: u32 = 2;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SaveData {
    pub format_version: u32,
    pub generator_version: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ore_config: Option<crate::ores::OreConfig>,
    pub name: String,
    pub seed: u64,
    pub player: [f32; 3],
    pub yaw: f32,
    pub pitch: f32,
    pub time: f64,
    pub edits: Vec<([i32; 3], u8)>,
}

impl Default for SaveData {
    fn default() -> Self {
        Self {
            format_version: CURRENT_FORMAT_VERSION,
            generator_version: CURRENT_GENERATOR_VERSION,
            ore_config: Some(crate::ores::OreConfig::default()),
            name: "New World".to_owned(),
            seed: 0,
            player: [0.5, 100.0, 0.5],
            yaw: 0.0,
            pitch: 0.0,
            time: 0.0,
            edits: Vec::new(),
        }
    }
}

/// Saves a world beneath an explicitly chosen directory.
///
/// A synced temporary file replaces the primary save. A checked previous primary
/// is staged and synced as a backup first. Corrupt primary data never replaces a
/// valid backup, and wholly corrupt saves must be recovered manually or saved
/// under another ID. Callers should serialize saves to the same directory.
pub fn save_world(root: &Path, id: &str, data: &SaveData) -> Result<(), String> {
    validate_id(id)?;
    validate_data(data)?;
    let bytes = encode_save(data)?;
    let primary_path = root.join(format!("{id}.json"));
    let backup_path = root.join(format!("{id}.json.bak"));
    let primary = read_candidate(&primary_path);
    let backup = read_candidate(&backup_path);
    require_replaceable(&primary)?;
    require_replaceable(&backup)?;
    let primary_valid = matches!(&primary, Ok(Some(_)));
    let backup_valid = matches!(&backup, Ok(Some(_)));
    let new_world = matches!(&primary, Ok(None)) && matches!(&backup, Ok(None));
    if !new_world && !primary_valid && !backup_valid {
        return Err(format!(
            "Refusing to overwrite unrecoverable world {id:?}; primary: {}; backup: {}",
            candidate_status(&primary),
            candidate_status(&backup)
        ));
    }

    fs::create_dir_all(root)
        .map_err(|error| format!("Cannot create save directory {}: {error}", root.display()))?;
    let staged = stage_file(root, id, "save", &bytes)?;
    if let Ok(Some(previous)) = &primary {
        let staged_backup = stage_file(root, id, "backup", &previous.bytes)?;
        replace_file(&staged_backup.path, &backup_path, true)?;
        sync_directory(root)?;
    }
    // A Windows replacement fallback may temporarily remove the primary only
    // after a verified backup is available. The loader can recover that backup.
    replace_file(&staged.path, &primary_path, primary_valid || backup_valid)?;
    sync_directory(root)
}

/// Loads the checked primary or a checked backup without changing either file.
/// Missing or invalid data always returns an error; no new world is substituted.
pub fn load_world(root: &Path, id: &str) -> Result<SaveData, String> {
    validate_id(id)?;
    let primary = read_candidate(&root.join(format!("{id}.json")));
    require_replaceable(&primary)?;
    if let Ok(Some(save)) = &primary {
        return Ok(save.data.clone());
    }
    let backup = read_candidate(&root.join(format!("{id}.json.bak")));
    require_replaceable(&backup)?;
    if let Ok(Some(save)) = &backup {
        return Ok(save.data.clone());
    }
    Err(format!(
        "Cannot load world {id:?}; primary: {}; backup: {}",
        candidate_status(&primary),
        candidate_status(&backup)
    ))
}

/// Returns IDs and display names sorted by ID, including recoverable backups.
/// An invalid recognized world is reported rather than silently hidden.
pub fn list_worlds(root: &Path) -> Result<Vec<(String, String)>, String> {
    let entries = match fs::read_dir(root) {
        Ok(entries) => entries,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => {
            return Err(format!(
                "Cannot list save directory {}: {error}",
                root.display()
            ));
        }
    };
    let mut ids = BTreeSet::new();
    for entry in entries {
        let entry = entry.map_err(|error| format!("Cannot read save entry: {error}"))?;
        let filename = entry.file_name();
        let Some(filename) = filename.to_str() else {
            return Err("Save directory contains a non-Unicode filename".to_owned());
        };
        if let Some(id) = filename
            .strip_suffix(".json.bak")
            .or_else(|| filename.strip_suffix(".json"))
        {
            validate_id(id)
                .map_err(|error| format!("Invalid save filename {filename:?}: {error}"))?;
            ids.insert(id.to_owned());
        }
    }
    ids.into_iter()
        .map(|id| load_world(root, &id).map(|data| (id, data.name)))
        .collect()
}

#[derive(Serialize, Deserialize)]
struct SaveEnvelope {
    /// FNV-1a checks exact payload bytes for accidental corruption, not authenticity.
    checksum: String,
    payload: String,
}

struct CheckedSave {
    data: SaveData,
    bytes: Vec<u8>,
}

enum ReadFailure {
    Corrupt(String),
    // An unreadable or newer save could contain constructions absent in the
    // backup. It must never be interpreted as a reason to roll back that save.
    Protected(String),
}

type SaveCandidate = Result<Option<CheckedSave>, ReadFailure>;

fn require_replaceable(candidate: &SaveCandidate) -> Result<(), String> {
    match candidate {
        Err(ReadFailure::Protected(error)) => Err(error.clone()),
        _ => Ok(()),
    }
}

fn validate_id(id: &str) -> Result<(), String> {
    if id.is_empty()
        || id.len() > 64
        || !id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_')
    {
        return Err("World ID must contain 1–64 ASCII letters, digits, '-' or '_'".to_owned());
    }
    let upper = id.to_ascii_uppercase();
    if matches!(upper.as_str(), "CON" | "PRN" | "AUX" | "NUL")
        || upper
            .strip_prefix("COM")
            .or_else(|| upper.strip_prefix("LPT"))
            .is_some_and(|suffix| suffix.len() == 1 && matches!(suffix.as_bytes()[0], b'1'..=b'9'))
    {
        return Err("World ID is a reserved Windows filename".to_owned());
    }
    Ok(())
}

fn validate_data(data: &SaveData) -> Result<(), String> {
    if data.format_version != CURRENT_FORMAT_VERSION {
        return Err(format!(
            "Unsupported save format version {}",
            data.format_version
        ));
    }
    crate::ores::WorldGenerator::from_snapshot(data.generator_version, data.ore_config.clone())?;
    if data.name.trim().is_empty()
        || data.name.chars().count() > 128
        || data.name.chars().any(char::is_control)
    {
        return Err(
            "World name must contain 1–128 characters without control characters".to_owned(),
        );
    }
    if data.player.iter().any(|coordinate| !coordinate.is_finite())
        || !data.yaw.is_finite()
        || !data.pitch.is_finite()
    {
        return Err("Player coordinates and orientation must be finite".to_owned());
    }
    if !data.time.is_finite() || data.time < 0.0 {
        return Err("World time must be finite and nonnegative".to_owned());
    }
    let mut positions = HashSet::with_capacity(data.edits.len());
    for &(position, block) in &data.edits {
        if !crate::valid_world_height(position[1]) {
            return Err(format!(
                "Edit at {position:?} is outside world height bounds"
            ));
        }
        // Runtime-сундук не сохраняется до реализации состояния контейнеров.
        if usize::from(block) >= crate::BLOCKS.len() || block == crate::CHEST {
            return Err(format!(
                "Edit at {position:?} contains unknown block ID {block}"
            ));
        }
        if !positions.insert(position) {
            return Err(format!("Duplicate edit at {position:?}"));
        }
    }
    Ok(())
}

fn checksum(bytes: &[u8]) -> String {
    let value = bytes.iter().fold(0xcbf2_9ce4_8422_2325_u64, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(0x0000_0100_0000_01b3)
    });
    format!("{value:016x}")
}

fn encode_save(data: &SaveData) -> Result<Vec<u8>, String> {
    let payload = serde_json::to_string(data)
        .map_err(|error| format!("Cannot encode world data: {error}"))?;
    let envelope = SaveEnvelope {
        checksum: checksum(payload.as_bytes()),
        payload,
    };
    serde_json::to_vec_pretty(&envelope)
        .map_err(|error| format!("Cannot encode save envelope: {error}"))
}

fn read_candidate(path: &Path) -> SaveCandidate {
    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            return Err(ReadFailure::Protected(format!(
                "Cannot inspect {}: {error}",
                path.display()
            )));
        }
    };
    if !metadata.file_type().is_file() {
        return Err(ReadFailure::Protected(format!(
            "{} is not a regular save file",
            path.display()
        )));
    }
    let bytes = fs::read(path).map_err(|error| {
        ReadFailure::Protected(format!("Cannot read {}: {error}", path.display()))
    })?;
    let envelope: SaveEnvelope = serde_json::from_slice(&bytes).map_err(|error| {
        ReadFailure::Corrupt(format!("Invalid save envelope {}: {error}", path.display()))
    })?;
    if envelope.checksum != checksum(envelope.payload.as_bytes()) {
        return Err(ReadFailure::Corrupt(format!(
            "Save checksum mismatch in {}",
            path.display()
        )));
    }
    // Inspect versions before strict schema parsing: future versions may add
    // fields, which must block rollback rather than look like corrupt content.
    let header: serde_json::Value = serde_json::from_str(&envelope.payload).map_err(|error| {
        ReadFailure::Corrupt(format!("Invalid world data {}: {error}", path.display()))
    })?;
    for (field, supported) in [
        ("format_version", CURRENT_FORMAT_VERSION),
        ("generator_version", CURRENT_GENERATOR_VERSION),
    ] {
        if let Some(version) = header.get(field).and_then(serde_json::Value::as_u64)
            && !(1..=u64::from(supported)).contains(&version)
        {
            return Err(ReadFailure::Protected(format!(
                "Unsupported {field} {version} in {}; save was left unchanged",
                path.display()
            )));
        }
    }
    let data = if header
        .get("format_version")
        .and_then(serde_json::Value::as_u64)
        == Some(1)
    {
        migrate_legacy(&envelope.payload)
    } else {
        serde_json::from_str(&envelope.payload).map_err(|error| error.to_string())
    }
    .map_err(|error| {
        ReadFailure::Corrupt(format!("Invalid world data {}: {error}", path.display()))
    })?;
    validate_data(&data)
        .map_err(|error| ReadFailure::Corrupt(format!("{}: {error}", path.display())))?;
    Ok(Some(CheckedSave { data, bytes }))
}

// Строгая старая схема проверяется до миграции, без добавления снимка руд.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct LegacySaveData {
    format_version: u32,
    generator_version: u32,
    name: String,
    seed: u64,
    player: [f32; 3],
    yaw: f32,
    pitch: f32,
    time: f64,
    edits: Vec<([i32; 3], u8)>,
}
fn migrate_legacy(payload: &str) -> Result<SaveData, String> {
    let old: LegacySaveData = serde_json::from_str(payload).map_err(|error| error.to_string())?;
    if old.format_version != 1
        || old.generator_version != 1
        || old.edits.iter().any(|(_, block)| *block >= crate::CHEST)
    {
        return Err("Некорректное сохранение старого формата 1".into());
    }
    let data = SaveData {
        format_version: CURRENT_FORMAT_VERSION,
        generator_version: 1,
        ore_config: None,
        name: old.name,
        seed: old.seed,
        player: old.player,
        yaw: old.yaw,
        pitch: old.pitch,
        time: old.time,
        edits: old.edits,
    };
    validate_data(&data)?;
    Ok(data)
}

fn candidate_status(candidate: &SaveCandidate) -> &str {
    match candidate {
        Ok(Some(_)) => "valid",
        Ok(None) => "missing",
        Err(ReadFailure::Corrupt(error) | ReadFailure::Protected(error)) => error,
    }
}

static NEXT_TEMP_FILE: AtomicU64 = AtomicU64::new(0);

struct StagedFile {
    path: PathBuf,
}

impl Drop for StagedFile {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

fn stage_file(root: &Path, id: &str, purpose: &str, bytes: &[u8]) -> Result<StagedFile, String> {
    loop {
        let sequence = NEXT_TEMP_FILE.fetch_add(1, Ordering::Relaxed);
        let path = root.join(format!(
            ".{id}.{purpose}.{}.{sequence}.tmp",
            std::process::id()
        ));
        let mut file = match OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(file) => file,
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(format!("Cannot stage save {}: {error}", path.display())),
        };
        let staged = StagedFile { path };
        let result = file.write_all(bytes).and_then(|()| file.sync_all());
        // Windows cannot rename an open file unless it was opened for sharing.
        drop(file);
        result.map_err(|error| format!("Cannot sync save {}: {error}", staged.path.display()))?;
        return Ok(staged);
    }
}

fn replace_file(staged: &Path, destination: &Path, _backup_available: bool) -> Result<(), String> {
    match fs::rename(staged, destination) {
        Ok(()) => Ok(()),
        Err(error) => {
            #[cfg(windows)]
            if _backup_available
                && matches!(
                    error.kind(),
                    io::ErrorKind::AlreadyExists | io::ErrorKind::PermissionDenied
                )
                && destination.is_file()
            {
                fs::remove_file(destination).map_err(|remove_error| {
                    format!(
                        "Cannot replace {}: {remove_error}; previous save remains backed up",
                        destination.display()
                    )
                })?;
                return fs::rename(staged, destination).map_err(|rename_error| {
                    format!(
                        "Cannot install {}: {rename_error}; load its backup to recover",
                        destination.display()
                    )
                });
            }
            Err(format!("Cannot replace {}: {error}", destination.display()))
        }
    }
}

fn sync_directory(root: &Path) -> Result<(), String> {
    #[cfg(unix)]
    fs::File::open(root)
        .and_then(|directory| directory.sync_all())
        .map_err(|error| format!("Cannot sync save directory {}: {error}", root.display()))?;
    #[cfg(not(unix))]
    let _ = root;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs,
        path::PathBuf,
        sync::atomic::{AtomicU64, Ordering},
    };

    static NEXT_TEST_ROOT: AtomicU64 = AtomicU64::new(0);

    struct TestRoot(PathBuf);

    impl TestRoot {
        fn new() -> Self {
            let path = std::env::temp_dir().join(format!(
                "voxel-sandbox-save-test-{}-{}",
                std::process::id(),
                NEXT_TEST_ROOT.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&path).unwrap();
            Self(path)
        }
    }

    impl Drop for TestRoot {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn example() -> SaveData {
        SaveData {
            name: "A world 世界".to_owned(),
            seed: u64::MAX,
            player: [-16.5, 70.25, 31.5],
            yaw: 2.345,
            pitch: -0.75,
            time: 123_456.123_456_789,
            edits: vec![([-17, -64, 16], 0), ([16, 320, -1], 15)],
            ..SaveData::default()
        }
    }

    #[test]
    fn legacy_checksum_migrates_without_rewriting_and_backs_up_exact_bytes() {
        let root = TestRoot::new();
        let original = include_bytes!("../tests/fixtures/legacy-v1.json");
        let primary = root.0.join("legacy.json");
        fs::write(&primary, original).unwrap();
        let mut data = load_world(&root.0, "legacy").unwrap();
        assert_eq!(data.format_version, 2);
        assert_eq!(data.generator_version, 1);
        assert_eq!(data.seed, 1_794_214_108_883_539_500);
        assert_eq!(fs::read(&primary).unwrap(), original);
        assert!(!root.0.join("legacy.json.bak").exists());
        data.edits.push(([8, 200, 8], crate::DIAMOND_BLOCK));
        save_world(&root.0, "legacy", &data).unwrap();
        assert_eq!(fs::read(root.0.join("legacy.json.bak")).unwrap(), original);
        let loaded = load_world(&root.0, "legacy").unwrap();
        assert_eq!(loaded, data);
        let generator =
            crate::ores::WorldGenerator::from_snapshot(loaded.generator_version, loaded.ore_config)
                .unwrap();
        let mut world = crate::World::new(loaded.seed);
        world.edits = loaded.edits.into_iter().collect();
        world.insert(generator.generate(loaded.seed, [0, 0]));
        assert_eq!(world.block([8, 200, 8]), crate::DIAMOND_BLOCK);
        let envelope: SaveEnvelope = serde_json::from_slice(&fs::read(primary).unwrap()).unwrap();
        let payload: serde_json::Value = serde_json::from_str(&envelope.payload).unwrap();
        assert_eq!(payload["format_version"], 2);
        assert!(
            payload
                .get("ore_config")
                .is_none_or(serde_json::Value::is_null)
        );
    }

    #[test]
    fn legacy_schema_rejects_snapshot_new_ids_and_mismatched_generator() {
        let root = TestRoot::new();
        let original: SaveEnvelope =
            serde_json::from_slice(include_bytes!("../tests/fixtures/legacy-v1.json")).unwrap();
        let value: serde_json::Value = serde_json::from_str(&original.payload).unwrap();
        let mut with_snapshot = value.clone();
        with_snapshot["ore_config"] =
            serde_json::to_value(crate::ores::OreConfig::default()).unwrap();
        let mut with_new_id = value.clone();
        with_new_id["edits"] = serde_json::json!([[[8, 200, 8], crate::DIAMOND_BLOCK]]);
        let mut with_generator_two = value;
        with_generator_two["generator_version"] = 2.into();
        for invalid in [with_snapshot, with_new_id, with_generator_two] {
            let payload = serde_json::to_string(&invalid).unwrap();
            let bytes = serde_json::to_vec(&SaveEnvelope {
                checksum: checksum(payload.as_bytes()),
                payload,
            })
            .unwrap();
            let path = root.0.join("invalid.json");
            fs::write(&path, &bytes).unwrap();
            assert!(load_world(&root.0, "invalid").is_err());
            assert_eq!(fs::read(path).unwrap(), bytes);
            assert!(!root.0.join("invalid.json.bak").exists());
        }
    }

    #[test]
    fn generator_two_requires_snapshot_and_round_trips_changed_world_defaults() {
        let root = TestRoot::new();
        let mut value = serde_json::to_value(example()).unwrap();
        assert_eq!(value["format_version"], 2);
        assert_eq!(value["generator_version"], 2);
        value["ore_config"]["ores"][0]["veins"] = 1.into();
        let data: SaveData = serde_json::from_value(value.clone()).unwrap();
        save_world(&root.0, "snapshot", &data).unwrap();
        let loaded = load_world(&root.0, "snapshot").unwrap();
        let generator = crate::ores::WorldGenerator::from_snapshot(
            loaded.generator_version,
            loaded.ore_config.clone(),
        )
        .unwrap();
        let current_defaults =
            crate::ores::WorldGenerator::from_snapshot(2, Some(crate::ores::OreConfig::default()))
                .unwrap();
        assert_ne!(
            generator.generate(data.seed, [-1, 0]),
            current_defaults.generate(data.seed, [-1, 0])
        );
        assert_eq!(serde_json::to_value(loaded).unwrap(), value);
        for invalid in [serde_json::Value::Null, serde_json::json!({"ores": []})] {
            value["ore_config"] = invalid;
            let payload = serde_json::to_string(&value).unwrap();
            let bytes = serde_json::to_vec(&SaveEnvelope {
                checksum: checksum(payload.as_bytes()),
                payload,
            })
            .unwrap();
            fs::write(root.0.join("invalid.json"), bytes).unwrap();
            assert!(load_world(&root.0, "invalid").is_err());
        }
    }

    #[test]
    fn checksum_is_checked_before_legacy_migration_or_future_version_guard() {
        let root = TestRoot::new();
        let original = include_bytes!("../tests/fixtures/legacy-v1.json");
        let mut envelope: SaveEnvelope = serde_json::from_slice(original).unwrap();
        envelope.payload = envelope
            .payload
            .replace("\"format_version\":1", "\"format_version\":100");
        fs::write(
            root.0.join("world.json"),
            serde_json::to_vec(&envelope).unwrap(),
        )
        .unwrap();
        let error = load_world(&root.0, "world").unwrap_err();
        assert!(error.contains("checksum"), "{error}");
        assert!(!error.contains("Unsupported"), "{error}");
    }

    #[test]
    fn world_round_trips_player_seed_time_and_boundary_edits() {
        let root = TestRoot::new();
        let data = example();
        save_world(&root.0, "my-world_1", &data).unwrap();
        assert_eq!(load_world(&root.0, "my-world_1").unwrap(), data);
    }

    #[test]
    fn lists_worlds_by_id_with_display_names_and_handles_missing_root() {
        let root = TestRoot::new();
        assert_eq!(list_worlds(&root.0.join("missing")).unwrap(), Vec::new());
        let mut data = example();
        data.name = "Zebra".to_owned();
        save_world(&root.0, "z", &data).unwrap();
        data.name = "Alpha".to_owned();
        save_world(&root.0, "a", &data).unwrap();
        fs::write(root.0.join("unrelated.tmp"), b"ignored").unwrap();
        assert_eq!(
            list_worlds(&root.0).unwrap(),
            vec![
                ("a".to_owned(), "Alpha".to_owned()),
                ("z".to_owned(), "Zebra".to_owned())
            ]
        );
    }

    #[test]
    fn recovers_previous_save_from_backup_after_primary_is_corrupted() {
        let root = TestRoot::new();
        let first = example();
        save_world(&root.0, "world", &first).unwrap();
        let mut second = first.clone();
        second.edits.push(([1, 80, 1], 7));
        save_world(&root.0, "world", &second).unwrap();
        assert_eq!(load_world(&root.0, "world").unwrap(), second);
        fs::write(root.0.join("world.json"), b"interrupted write").unwrap();
        assert_eq!(load_world(&root.0, "world").unwrap(), first);
        let backup = fs::read(root.0.join("world.json.bak")).unwrap();
        save_world(&root.0, "world", &second).unwrap();
        assert_eq!(fs::read(root.0.join("world.json.bak")).unwrap(), backup);
        assert_eq!(load_world(&root.0, "world").unwrap(), second);
    }

    #[test]
    fn loads_backup_when_primary_is_missing() {
        let root = TestRoot::new();
        let data = example();
        save_world(&root.0, "world", &data).unwrap();
        save_world(&root.0, "world", &data).unwrap();
        fs::remove_file(root.0.join("world.json")).unwrap();
        assert_eq!(load_world(&root.0, "world").unwrap(), data);
        assert_eq!(
            list_worlds(&root.0).unwrap(),
            vec![("world".to_owned(), data.name)]
        );
    }

    #[test]
    fn does_not_overwrite_unrecoverable_save_or_hide_it_from_listing() {
        let root = TestRoot::new();
        let primary = root.0.join("world.json");
        let backup = root.0.join("world.json.bak");
        fs::write(&primary, b"corrupt primary").unwrap();
        fs::write(&backup, b"corrupt backup").unwrap();
        assert!(load_world(&root.0, "world").is_err());
        assert!(list_worlds(&root.0).is_err());
        assert!(save_world(&root.0, "world", &example()).is_err());
        assert_eq!(fs::read(primary).unwrap(), b"corrupt primary");
        assert_eq!(fs::read(backup).unwrap(), b"corrupt backup");
    }

    #[test]
    fn rejects_parseable_payload_with_wrong_integrity_checksum() {
        let root = TestRoot::new();
        save_world(&root.0, "world", &example()).unwrap();
        let path = root.0.join("world.json");
        let mut envelope: serde_json::Value =
            serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        let payload = envelope["payload"]
            .as_str()
            .unwrap()
            .replace("A world 世界", "Altered world");
        envelope["payload"] = payload.into();
        let corrupted = serde_json::to_vec(&envelope).unwrap();
        fs::write(&path, &corrupted).unwrap();
        assert!(
            load_world(&root.0, "world")
                .unwrap_err()
                .contains("checksum")
        );
        assert!(save_world(&root.0, "world", &example()).is_err());
        assert_eq!(fs::read(path).unwrap(), corrupted);
    }

    #[test]
    fn never_rolls_back_or_overwrites_incompatible_primary_from_older_backup() {
        let root = TestRoot::new();
        for field in ["format_version", "generator_version"] {
            let id = field;
            let data = example();
            save_world(&root.0, id, &data).unwrap();
            save_world(&root.0, id, &data).unwrap();
            let primary_path = root.0.join(format!("{id}.json"));
            let backup_path = root.0.join(format!("{id}.json.bak"));
            let backup = fs::read(&backup_path).unwrap();
            let mut payload = serde_json::to_value(&data).unwrap();
            payload[field] = 100_u32.into();
            payload["future_field"] = true.into();
            let payload = serde_json::to_string(&payload).unwrap();
            let envelope = SaveEnvelope {
                checksum: checksum(payload.as_bytes()),
                payload,
            };
            let mut envelope = serde_json::to_value(&envelope).unwrap();
            envelope["future_envelope_field"] = true.into();
            let future = serde_json::to_vec(&envelope).unwrap();
            fs::write(&primary_path, &future).unwrap();
            assert!(load_world(&root.0, id).is_err());
            assert!(save_world(&root.0, id, &data).is_err());
            assert_eq!(fs::read(primary_path).unwrap(), future);
            assert_eq!(fs::read(backup_path).unwrap(), backup);
        }
    }

    #[test]
    fn rejects_unsafe_ids_without_creating_a_directory() {
        let root = TestRoot::new();
        let missing = root.0.join("missing");
        for id in [
            "",
            "..",
            "../escape",
            "a/b",
            "a\\b",
            "a.b",
            "with space",
            "CON",
            "lpt1",
        ] {
            assert!(
                save_world(&missing, id, &example()).is_err(),
                "accepted {id:?}"
            );
            assert!(load_world(&missing, id).is_err(), "accepted {id:?}");
        }
        assert!(!missing.exists());
    }

    #[test]
    fn rejects_invalid_save_values_before_writing() {
        let root = TestRoot::new();
        let mut cases = Vec::new();
        let mut data = example();
        data.format_version += 1;
        cases.push(data);
        let mut data = example();
        data.generator_version += 1;
        cases.push(data);
        let mut data = example();
        data.name = " \t".to_owned();
        cases.push(data);
        let mut data = example();
        data.name = "bad\nname".to_owned();
        cases.push(data);
        let mut data = example();
        data.player[0] = f32::INFINITY;
        cases.push(data);
        let mut data = example();
        data.yaw = f32::NAN;
        cases.push(data);
        let mut data = example();
        data.pitch = f32::INFINITY;
        cases.push(data);
        let mut data = example();
        data.time = f64::INFINITY;
        cases.push(data);
        let mut data = example();
        data.time = -1.0;
        cases.push(data);
        for (position, block) in [([0, -65, 0], 1), ([0, 321, 0], 1), ([0, 70, 0], 16)] {
            let mut data = example();
            data.edits.push((position, block));
            cases.push(data);
        }
        let mut data = example();
        data.edits.push(data.edits[0]);
        cases.push(data);
        for data in cases {
            assert!(
                save_world(&root.0, "world", &data).is_err(),
                "accepted {data:?}"
            );
        }
        assert_eq!(fs::read_dir(&root.0).unwrap().count(), 0);
    }

    #[cfg(unix)]
    #[test]
    fn refuses_to_read_or_overwrite_symlink_save() {
        let root = TestRoot::new();
        let external = root.0.join("external");
        fs::write(&external, b"do not touch").unwrap();
        std::os::unix::fs::symlink(&external, root.0.join("world.json")).unwrap();
        assert!(load_world(&root.0, "world").is_err());
        assert!(save_world(&root.0, "world", &example()).is_err());
        assert_eq!(fs::read(external).unwrap(), b"do not touch");
    }

    #[test]
    fn does_not_replace_non_file_primary_even_with_valid_backup() {
        let root = TestRoot::new();
        let data = example();
        save_world(&root.0, "world", &data).unwrap();
        save_world(&root.0, "world", &data).unwrap();
        let primary = root.0.join("world.json");
        let backup = root.0.join("world.json.bak");
        let old_backup = fs::read(&backup).unwrap();
        fs::remove_file(&primary).unwrap();
        fs::create_dir(&primary).unwrap();
        assert!(load_world(&root.0, "world").is_err());
        assert!(save_world(&root.0, "world", &data).is_err());
        assert!(primary.is_dir());
        assert_eq!(fs::read(backup).unwrap(), old_backup);
    }
}
