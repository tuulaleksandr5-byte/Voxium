# Block audio

The game includes 135 original, procedurally synthesized Foley clips: 15 non-air
blocks × break/step/place × three variants. The defaults are 22,050 Hz, mono,
16-bit PCM WAV. There are no downloaded samples or runtime generation dependencies.
All default clips are embedded in the executable; the Windows EXE can play them
without a separate sounds directory.

`tools/generate_block_sounds.py` uses only the Python standard library. Its stable
SHA-256 seeds reproduce the checked-in files byte for byte:

```sh
python3 tools/generate_block_sounds.py
```

Grass, leaves and snow use soft filtered rustles; dirt, sand and gravel use
granular crunches and low thuds. Stone, deepslate, cobblestone, sandstone and
bedrock use short inharmonic knocks. Logs and planks have different hollow wooden
resonances. Glass combines bright tinkling resonances with small scattered
fragments; water uses filtered splashes and falling bubble tones. Breaking has
a longer scattered tail, footsteps a softer compact crunch, and placing a shorter
rounded impact. Each of the three variants has its own deterministic texture,
particle timing and resonant tuning. Peaks are softened with gentle saturation.

## Integration

The module targets Bevy **exactly 0.20.1**. Enable the `wav` Bevy feature and retain
the default `vorbis` feature for Ogg overrides. The client also depends on
`rodio = { version = "=0.22.2", default-features = false }`; Bevy supplies its
decoder features. Direct access to rodio's fallible decoder avoids the unwrap
inside Bevy's `AudioSource::decoder` for damaged files.

1. Declare `mod block_audio;` in the client.
2. Add `block_audio::BlockAudioPlugin` after `DefaultPlugins`.
3. Attach `SpatialListener::new(0.3)` to the player camera. Keep one active listener.
4. Write `BlockSound` messages after successful block changes, and on movement
   footfalls while grounded. The `block` is the material touched or changed;
   `position` is its world-space center. Air and unknown IDs are silent.

```rust,ignore
use crate::block_audio::{BlockSound, SoundKind};

sounds.write(BlockSound {
    block: material_id,
    kind: SoundKind::Break, // or Step / Place
    position: block_position.as_vec3() + Vec3::splat(0.5),
});
```

Each message chooses a random variant and an independent speed/pitch multiplier
within 0.95–1.05. Playback uses spatial stereo panning, distance attenuation and
a 48-block cutoff. Footsteps are quieter than breaking. Bevy's `GlobalVolume`
continues to control the overall volume.

There are at most 64 live sound emitters and 24 newly spawned emitters per frame.
Each emitter despawns at completion or after five seconds, including when the
machine has no audio device. Failed/no-listener events are consumed, preventing
a backlog of delayed sounds. Embedded clips are decoded once during startup;
playback clones an immutable decoded PCM buffer and never decodes untrusted data.

## Optional sound replacements

Files use these paths (variants are numbered **1–3**):

```text
assets/sounds/<block>/break1.wav
assets/sounds/<block>/step2.wav
assets/sounds/<block>/place3.wav
```

The block directories follow stable block ID order:
`grass`, `dirt`, `stone`, `deepslate`, `sand`, `gravel`, `oak_log`, `leaves`,
`oak_planks`, `cobblestone`, `glass`, `water`, `bedrock`, `snow`, `sandstone`.

The game first checks `VOXEL_SOUND_DIR` if set; otherwise it checks
`assets/sounds` beside the executable, then `assets/sounds` in the working
directory. An existing root requests disk overrides. Each clip tries `.ogg`
before `.wav`, then its embedded default. A damaged Ogg can therefore fall back
to a valid WAV. Missing requested files, unreadable files, oversized files and
invalid clips emit warnings. A standalone executable with no override directory
uses its embedded library without missing-file warnings.

Overrides must be mono or stereo, no more than 192 kHz, four seconds and 16 MiB.
Ogg page boundaries, final-page markers and CRCs are checked, as are WAV container
lengths. Every clip is fully decoded and checked for invalid floating samples
before it can reach playback. Invalid embedded clips are also skipped with a
warning instead of stopping the game.

## Verification

The module tests decode all 135 embedded clips, check distinct variants,
exercise random selection and the ±5% pitch range, and verify missing/damaged
override warnings and fallback. A truncated WAV is rejected before playback.

```sh
cargo test -p voxel-client block_audio::tests
```
