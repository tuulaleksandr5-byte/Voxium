//! Embedded, material-specific Foley with optional, validated disk overrides.
use std::{
    io::Cursor,
    path::{Path, PathBuf},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use bevy::{
    audio::{AddAudioSource, Decodable, PlaybackSettings, SpatialScale, Volume},
    prelude::*,
};
use rodio::{Source, buffer::SamplesBuffer};

/// The three independently recorded/synthesized interactions for each material.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SoundKind {
    Break,
    Step,
    Place,
}

impl SoundKind {
    fn index(self) -> usize {
        match self {
            Self::Break => 0,
            Self::Step => 1,
            Self::Place => 2,
        }
    }
    fn name(self) -> &'static str {
        match self {
            Self::Break => "break",
            Self::Step => "step",
            Self::Place => "place",
        }
    }
}

/// Emit after a successful interaction. `position` is the world-space block center.
#[derive(Message, Clone, Copy, Debug)]
pub struct BlockSound {
    pub block: u8,
    pub kind: SoundKind,
    pub position: Vec3,
}

pub struct BlockAudioPlugin;

impl Plugin for BlockAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_audio_source::<BlockClip>()
            .add_message::<BlockSound>()
            .init_resource::<SoundRandom>()
            .add_systems(Startup, load_library)
            .add_systems(Update, (expire_emitters, play_block_sounds).chain());
    }
}

// Decoded once, using fallible rodio APIs. Playback only clones this ready-to-use
// buffer, avoiding AudioSource::decoder's unwrap on malformed external files.
#[derive(Asset, TypePath, Clone)]
struct BlockClip(SamplesBuffer);
impl Decodable for BlockClip {
    type Decoder = SamplesBuffer;
    fn decoder(&self) -> Self::Decoder {
        self.0.clone()
    }
}

#[derive(Resource)]
struct SoundLibrary(Vec<[[Option<Handle<BlockClip>>; 3]; 3]>);

const BLOCK_NAMES: [&str; 15] = [
    "grass",
    "dirt",
    "stone",
    "deepslate",
    "sand",
    "gravel",
    "oak_log",
    "leaves",
    "oak_planks",
    "cobblestone",
    "glass",
    "water",
    "bedrock",
    "snow",
    "sandstone",
];
const KINDS: [SoundKind; 3] = [SoundKind::Break, SoundKind::Step, SoundKind::Place];

macro_rules! embedded {
    ($block:literal, $kind:literal, $variant:literal) => {
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../assets/sounds/",
            $block,
            "/",
            $kind,
            $variant,
            ".wav"
        )) as &'static [u8]
    };
}
macro_rules! material {
    ($block:literal) => {
        [
            [
                embedded!($block, "break", "1"),
                embedded!($block, "break", "2"),
                embedded!($block, "break", "3"),
            ],
            [
                embedded!($block, "step", "1"),
                embedded!($block, "step", "2"),
                embedded!($block, "step", "3"),
            ],
            [
                embedded!($block, "place", "1"),
                embedded!($block, "place", "2"),
                embedded!($block, "place", "3"),
            ],
        ]
    };
}
const EMBEDDED: [[[&[u8]; 3]; 3]; 15] = [
    material!("grass"),
    material!("dirt"),
    material!("stone"),
    material!("deepslate"),
    material!("sand"),
    material!("gravel"),
    material!("oak_log"),
    material!("leaves"),
    material!("oak_planks"),
    material!("cobblestone"),
    material!("glass"),
    material!("water"),
    material!("bedrock"),
    material!("snow"),
    material!("sandstone"),
];

fn decode(bytes: Vec<u8>) -> Result<BlockClip, String> {
    validate_container(&bytes)?;
    let mut decoder =
        rodio::Decoder::try_from(Cursor::new(bytes)).map_err(|error| error.to_string())?;
    let channels = decoder.channels();
    let rate = decoder.sample_rate();
    if channels.get() > 2 || rate.get() > 192_000 {
        return Err("expected mono/stereo audio at no more than 192 kHz".into());
    }
    let limit = rate.get() as usize * channels.get() as usize * 4;
    let samples: Vec<f32> = decoder.by_ref().take(limit + 1).collect();
    if samples.is_empty()
        || samples.len() > limit
        || samples.iter().any(|sample| !sample.is_finite())
    {
        return Err("empty, invalid, or longer than four seconds".into());
    }
    Ok(BlockClip(SamplesBuffer::new(channels, rate, samples)))
}

// Decoders may stop quietly at a truncated final packet. Check the container
// boundaries too, so a damaged override produces a warning and the full fallback.
fn validate_container(bytes: &[u8]) -> Result<(), String> {
    if bytes.starts_with(b"RIFF") {
        let size = bytes.get(4..8).ok_or("truncated WAV header")?;
        let size = u32::from_le_bytes([size[0], size[1], size[2], size[3]]) as usize;
        if size.checked_add(8) != Some(bytes.len()) {
            return Err("truncated or inconsistent WAV container".into());
        }
    } else if bytes.starts_with(b"OggS") {
        let mut offset = 0;
        let mut ended = false;
        while offset < bytes.len() {
            let header = bytes.get(offset..offset + 27).ok_or("truncated Ogg page")?;
            if &header[..4] != b"OggS" || header[4] != 0 {
                return Err("invalid Ogg page".into());
            }
            let segments = header[26] as usize;
            let lacing = bytes
                .get(offset + 27..offset + 27 + segments)
                .ok_or("truncated Ogg lacing")?;
            let body: usize = lacing.iter().map(|&length| length as usize).sum();
            let end = offset + 27 + segments + body;
            let page = bytes.get(offset..end).ok_or("truncated Ogg body")?;
            let expected = u32::from_le_bytes([header[22], header[23], header[24], header[25]]);
            let mut checksum = 0_u32;
            for (index, &value) in page.iter().enumerate() {
                checksum ^= u32::from(if (22..26).contains(&index) { 0 } else { value }) << 24;
                for _ in 0..8 {
                    checksum = (checksum << 1)
                        ^ if checksum & 0x8000_0000 != 0 {
                            0x04c1_1db7
                        } else {
                            0
                        };
                }
            }
            if checksum != expected {
                return Err("damaged Ogg page checksum".into());
            }
            ended = header[5] & 4 != 0;
            offset = end;
        }
        if !ended {
            return Err("Ogg stream has no final page".into());
        }
    }
    Ok(())
}

/// A provided root is an explicit request to override sounds. No root means the
/// embedded library is expected, so a standalone executable logs no missing files.
fn override_root() -> Option<PathBuf> {
    if let Some(root) = std::env::var_os("VOXEL_SOUND_DIR") {
        return Some(root.into());
    }
    if let Ok(exe) = std::env::current_exe()
        && let Some(parent) = exe.parent()
    {
        let root = parent.join("assets/sounds");
        if root.is_dir() {
            return Some(root);
        }
    }
    let root = PathBuf::from("assets/sounds");
    root.is_dir().then_some(root)
}

fn load_clip(
    root: Option<&Path>,
    block: &str,
    kind: SoundKind,
    variant: usize,
    fallback: &[u8],
    mut warning: impl FnMut(String),
) -> Option<BlockClip> {
    if let Some(root) = root {
        let stem = root.join(block).join(format!("{}{variant}", kind.name()));
        let mut found = false;
        for extension in ["ogg", "wav"] {
            let path = stem.with_extension(extension);
            match std::fs::metadata(&path) {
                Ok(metadata) => {
                    found = true;
                    if metadata.len() > 16 * 1024 * 1024 {
                        warning(format!(
                            "Sound {} exceeds 16 MiB; using fallback",
                            path.display()
                        ));
                        continue;
                    }
                    let result = std::fs::read(&path)
                        .map_err(|error| error.to_string())
                        .and_then(decode);
                    match result {
                        Ok(clip) => return Some(clip),
                        Err(error) => warning(format!(
                            "Cannot decode sound {}: {error}; using fallback",
                            path.display()
                        )),
                    }
                }
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(error) => warning(format!(
                    "Cannot read sound {}: {error}; using fallback",
                    path.display()
                )),
            }
        }
        if !found {
            warning(format!(
                "Missing sound {}.ogg/.wav; using embedded sound",
                stem.display()
            ));
        }
    }
    match decode(fallback.to_vec()) {
        Ok(clip) => Some(clip),
        Err(error) => {
            warning(format!(
                "Embedded {block}/{}{variant}.wav is invalid: {error}; skipping sound",
                kind.name()
            ));
            None
        }
    }
}

fn load_library(mut commands: Commands, mut assets: ResMut<Assets<BlockClip>>) {
    let root = override_root();
    let mut library = Vec::with_capacity(BLOCK_NAMES.len());
    for (block, name) in BLOCK_NAMES.iter().enumerate() {
        let mut sounds = std::array::from_fn(|_| std::array::from_fn(|_| None));
        for kind in KINDS {
            for variant in 0..3 {
                sounds[kind.index()][variant] = load_clip(
                    root.as_deref(),
                    name,
                    kind,
                    variant + 1,
                    EMBEDDED[block][kind.index()][variant],
                    |message| warn!("{message}"),
                )
                .map(|clip| assets.add(clip));
            }
        }
        library.push(sounds);
    }
    commands.insert_resource(SoundLibrary(library));
}

#[derive(Resource)]
struct SoundRandom(u64);
impl Default for SoundRandom {
    fn default() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(1, |time| time.as_nanos() as u64);
        Self(seed.max(1))
    }
}
impl SoundRandom {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn choice(&mut self) -> (usize, f32) {
        let variant = (self.next() % 3) as usize;
        let unit = (self.next() >> 40) as f32 / ((1_u32 << 24) - 1) as f32;
        (variant, 0.95 + unit * 0.10)
    }
}

#[derive(Component)]
struct SoundLifetime(Timer);

fn expire_emitters(
    mut commands: Commands,
    time: Res<Time>,
    mut emitters: Query<(Entity, &mut SoundLifetime)>,
) {
    for (entity, mut lifetime) in &mut emitters {
        if lifetime.0.tick(time.delta()).is_finished() {
            commands.entity(entity).try_despawn();
        }
    }
}

fn play_block_sounds(
    mut commands: Commands,
    mut events: MessageReader<BlockSound>,
    library: Option<Res<SoundLibrary>>,
    mut random: ResMut<SoundRandom>,
    listener: Query<&GlobalTransform, With<SpatialListener>>,
    active: Query<(), With<SoundLifetime>>,
) {
    let listener = listener.iter().next().map(GlobalTransform::translation);
    let mut remaining = 64_usize.saturating_sub(active.iter().count()).min(24);
    for event in events.read() {
        let (Some(library), Some(listener)) = (library.as_ref(), listener) else {
            continue;
        };
        if event.block == 0 || !event.position.is_finite() || remaining == 0 {
            continue;
        }
        let block = if event.block == game_core::CHEST {
            game_core::PLANKS
        } else {
            event.block
        };
        let Some(material) = library.0.get(block as usize - 1) else {
            continue;
        };
        let distance = event.position.distance(listener);
        if distance > 48.0 {
            continue;
        }
        let (variant, pitch) = random.choice();
        let Some(clip) = material[event.kind.index()][variant].as_ref() else {
            continue;
        };
        let level = match event.kind {
            SoundKind::Break => 0.75,
            SoundKind::Step => 0.65,
            SoundKind::Place => 0.72,
        };
        let attenuation = 1.0 / (1.0 + (distance / 8.0).powi(2));
        debug!(block = event.block, kind = ?event.kind, position = ?event.position,
            variant = variant + 1, pitch, "Block sound queued");
        commands.spawn((
            AudioPlayer(clip.clone()),
            PlaybackSettings::DESPAWN
                .with_spatial(true)
                .with_spatial_scale(SpatialScale::new(0.08))
                .with_speed(pitch)
                .with_volume(Volume::Linear(level * attenuation))
                .with_duration(Duration::from_secs(5)),
            Transform::from_translation(event.position),
            SoundLifetime(Timer::from_seconds(5.0, TimerMode::Once)),
        ));
        remaining -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_embedded_materials_have_three_distinct_decodable_variants_per_event() {
        for material in EMBEDDED {
            for event in material {
                assert_ne!(event[0], event[1]);
                assert_ne!(event[1], event[2]);
                assert_ne!(event[0], event[2]);
                for bytes in event {
                    let decoded = decode(bytes.to_vec()).expect("valid embedded WAV");
                    assert_eq!(decoded.0.channels().get(), 1);
                    assert_eq!(decoded.0.sample_rate().get(), 22050);
                    assert!(decoded.0.total_duration().unwrap().as_secs_f32() > 0.15);
                }
            }
        }
    }

    #[test]
    fn random_selection_covers_variants_and_pitch_stays_in_five_percent_range() {
        let mut rng = SoundRandom(0x1a2b_3c4d);
        let mut seen = [false; 3];
        let mut low = false;
        let mut high = false;
        for _ in 0..1000 {
            let (variant, pitch) = rng.choice();
            seen[variant] = true;
            assert!((0.95..=1.05).contains(&pitch));
            low |= pitch < 0.97;
            high |= pitch > 1.03;
        }
        assert!(seen.into_iter().all(|value| value) && low && high);
    }

    #[test]
    fn missing_override_warns_and_uses_embedded_while_no_override_is_quiet() {
        let root = std::env::temp_dir().join(format!("voxel-audio-missing-{}", std::process::id()));
        let mut warnings = Vec::new();
        assert!(
            load_clip(
                Some(&root),
                "stone",
                SoundKind::Break,
                1,
                EMBEDDED[2][0][0],
                |message| warnings.push(message)
            )
            .is_some()
        );
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("Missing sound"));
        warnings.clear();
        assert!(
            load_clip(
                None,
                "stone",
                SoundKind::Break,
                1,
                EMBEDDED[2][0][0],
                |message| warnings.push(message)
            )
            .is_some()
        );
        assert!(warnings.is_empty());
    }

    #[test]
    fn malformed_override_warns_and_falls_back_without_decoder_panics() {
        let root = std::env::temp_dir().join(format!("voxel-audio-damaged-{}", std::process::id()));
        let directory = root.join("glass");
        std::fs::create_dir_all(&directory).unwrap();
        std::fs::write(directory.join("break1.ogg"), b"broken ogg audio").unwrap();
        let mut warnings = Vec::new();
        assert!(
            load_clip(
                Some(&root),
                "glass",
                SoundKind::Break,
                1,
                EMBEDDED[10][0][0],
                |message| warnings.push(message)
            )
            .is_some()
        );
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("Cannot decode sound"));
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn truncated_wav_is_rejected_before_playback() {
        let mut bytes = EMBEDDED[0][0][0].to_vec();
        bytes.truncate(bytes.len() - 20);
        assert!(decode(bytes).is_err());
    }

    #[test]
    fn emitter_expires_even_without_an_audio_output_or_sink() {
        let mut app = App::new();
        app.insert_resource(Time::<()>::default())
            .add_systems(Update, expire_emitters);
        let emitter = app
            .world_mut()
            .spawn(SoundLifetime(Timer::from_seconds(5.0, TimerMode::Once)))
            .id();
        app.world_mut()
            .resource_mut::<Time>()
            .advance_by(Duration::from_secs(6));
        app.update();
        assert!(app.world().get_entity(emitter).is_err());
    }
}
