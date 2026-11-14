//! Native creative voxel sandbox. See README for implemented scope and validation.
#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod app_icon;
mod block_audio;
mod console;
mod control_input;
mod footsteps;
mod graphics_assets;
mod inventory_client;
mod inventory_render;
mod item_icons;
mod item_textures;
mod mining_client;
mod mobs_client;
mod pixel_icons;
mod player;
mod session;
mod settings;
mod streaming;
mod survival_client;
mod tool_textures;
mod ui;

use bevy::{
    app::AppExit,
    prelude::*,
    window::{MonitorSelection, PresentMode, WindowCloseRequested, WindowMode},
};
use game_core::{
    SEA_LEVEL, World,
    persistence::{SaveData, list_worlds, load_world, save_world},
    surface_height,
};
use session::{Game, Mode};
use std::time::{SystemTime, UNIX_EPOCH};
use streaming::Streaming;
use ui::{Action, UiIntent};

fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: concat!(
                    "Voxel Sandbox — ",
                    env!("CARGO_PKG_VERSION"),
                    " · блоки, мобы и пиксели"
                )
                .into(),
                resolution: (1280, 720).into(),
                ..default()
            }),
            close_when_requested: false,
            ..default()
        }),
    )
    .add_plugins((
        game_render::VoxelRenderPlugin,
        block_audio::BlockAudioPlugin,
    ))
    .init_resource::<Game>()
    .init_resource::<Streaming>()
    .init_resource::<graphics_assets::BlockIcons>()
    .init_resource::<UiIntent>()
    .insert_resource(GlobalAmbientLight {
        color: Color::WHITE,
        brightness: 300.0,
        ..default()
    })
    .add_message::<survival_client::PlayerActionMessage>()
    .add_message::<survival_client::SurvivalStateChanged>()
    .add_systems(Startup, setup)
    .add_systems(Update, survival_client::reload_config.before(actions))
    .add_systems(Update, app_icon::set_icon)
    .add_systems(Update, graphics_assets::apply_style.after(actions))
    .add_systems(
        Update,
        (
            actions,
            streaming::stream_world,
            player::controls,
            survival_client::apply_actions,
            survival_client::simulate,
            survival_client::observe_changes,
            player::sky,
            autosave,
            close_save,
        )
            .chain(),
    );
    console::plugin(&mut app);
    inventory_client::plugin(&mut app);
    mining_client::plugin(&mut app);
    mobs_client::plugin(&mut app);
    item_icons::plugin(&mut app);
    inventory_render::plugin(&mut app);
    ui::plugin(&mut app);
    app.run();
}

#[allow(clippy::too_many_arguments)]
fn setup(
    mut commands: Commands,
    mut stream: ResMut<Streaming>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<game_render::VoxelMaterial>>,
    mut game: ResMut<Game>,
    mut icons: ResMut<graphics_assets::BlockIcons>,
    mut window: Single<&mut Window>,
    mut volume: ResMut<bevy::audio::GlobalVolume>,
) {
    stream.textures = graphics_assets::load_face_textures();
    let loaded = graphics_assets::load_atlas_for(game.settings.pixel_textures).and_then(|atlas| {
        let baked = game_render::bake_atlas_icons(&atlas, &stream.textures)?;
        let handles =
            game_render::create_materials_from_atlas(&mut images, &mut materials, &atlas)?;
        Ok((baked, handles))
    });
    match loaded {
        Ok((baked, handles)) => {
            icons.handles = baked.into_iter().map(|image| images.add(image)).collect();
            icons.pixel = Some(game.settings.pixel_textures);
            stream.materials = handles;
        }
        Err(error) => {
            error!("Original atlas unavailable: {error}");
            icons.error = Some(error.clone());
            game.status = error;
            game.ui_dirty = true;
        }
    }
    let sky_image = images.add(game_render::gradient_sky());
    commands.spawn((
        Camera3d::default(),
        player::PlayerCamera,
        bevy::audio::SpatialListener::new(0.3),
        bevy::camera::Exposure::default(),
        bevy::light::ShadowFilteringMethod::Gaussian,
        bevy::light::Skybox {
            image: Some(sky_image),
            brightness: 900.,
            ..default()
        },
        Transform::from_xyz(0.5, 85., 0.5),
        DistanceFog {
            color: Color::srgb(0.5, 0.7, 0.85),
            falloff: FogFalloff::Linear {
                start: 35.,
                end: 64.,
            },
            ..default()
        },
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 12000.0,
            shadow_maps_enabled: true,
            shadow_normal_bias: 0.6,
            shadow_depth_bias: 0.02,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -1.0, 0.4, 0.0)),
        player::Sun,
        bevy::light::CascadeShadowConfigBuilder {
            num_cascades: 2,
            maximum_distance: 96.,
            first_cascade_far_bound: 24.,
            overlap_proportion: 0.2,
            ..default()
        }
        .build(),
    ));
    apply_window(&game.settings, &mut window);
    *volume = bevy::audio::GlobalVolume::new(bevy::audio::Volume::Linear(game.settings.volume));
}
fn apply_window(s: &settings::Settings, window: &mut Window) {
    let size = [(1280, 720), (1600, 900), (1920, 1080)][s.resolution];
    // Resizing a borderless fullscreen surface makes some Windows drivers
    // leave fullscreen. Keep its monitor-owned size and only change actual options.
    let mode = if s.fullscreen {
        WindowMode::BorderlessFullscreen(MonitorSelection::Current)
    } else {
        WindowMode::Windowed
    };
    if window.mode != mode {
        window.mode = mode;
    }
    if !s.fullscreen
        && (window.resolution.width() != size.0 as f32
            || window.resolution.height() != size.1 as f32)
    {
        window.resolution.set(size.0 as f32, size.1 as f32);
    }
    let present = if s.vsync {
        PresentMode::AutoVsync
    } else {
        PresentMode::AutoNoVsync
    };
    if window.present_mode != present {
        window.present_mode = present;
    }
}
fn save(game: &mut Game) -> bool {
    let Some(id) = game.world_id.as_ref() else {
        return true;
    };
    match save_world(&settings::save_root(), id, &game.save_data()) {
        Ok(()) => {
            game.status = "Мир сохранён".into();
            game.autosave = 0.;
            true
        }
        Err(error) => {
            game.status = format!("Ошибка сохранения: {error}");
            game.ui_dirty = true;
            false
        }
    }
}
fn open(
    game: &mut Game,
    id: String,
    data: SaveData,
    stream: &mut Streaming,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
) -> Result<(), String> {
    let generator =
        game_core::ores::WorldGenerator::from_snapshot(data.generator_version, data.ore_config)?;
    stream.clear(commands, meshes);
    game.generator = generator;
    game.world = World::new(data.seed);
    game.world.edits = data.edits.into_iter().collect();
    game.world_id = Some(id);
    inventory_client::reset_session(game, false);
    game.name_input = data.name;
    game.feet = Vec3::from_array(data.player);
    game.velocity = Vec3::ZERO;
    game.grounded = false;
    game.height = 1.3;
    game.flying = false;
    game.game_mode = game_core::survival::GameMode::Survival;
    game.game_rules = game_core::survival::GameRules::default();
    game.vitals = game_core::survival::Vitals::new(&game.survival_config);
    game.spawn = Vec3::from_array(spawn_position(game.world.seed));
    game.pending_respawn = false;
    game.fall_peak = None;
    game.debug_open = false;
    game.yaw = data.yaw;
    game.pitch = data.pitch;
    game.clock = data.time;
    game.autosave = 0.;
    game.revision += 1;
    game.status = "Загрузка мира...".into();
    game.screen(Mode::Playing);
    Ok(())
}
#[allow(clippy::too_many_arguments)]
fn actions(
    mut commands: Commands,
    mut intents: ResMut<UiIntent>,
    mut game: ResMut<Game>,
    mut stream: ResMut<Streaming>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut window: Single<&mut Window>,
    mut volume: ResMut<bevy::audio::GlobalVolume>,
    icons: Res<graphics_assets::BlockIcons>,
    mut player_actions: MessageWriter<survival_client::PlayerActionMessage>,
    mut exit: MessageWriter<AppExit>,
) {
    for action in std::mem::take(&mut intents.0) {
        match action {
            Action::CycleGameMode => {
                game.creation_mode = match game.creation_mode {
                    game_core::survival::GameMode::Survival => {
                        game_core::survival::GameMode::Creative
                    }
                    game_core::survival::GameMode::Creative => {
                        game_core::survival::GameMode::Survival
                    }
                };
                game.ui_dirty = true;
            }
            Action::Respawn => {
                player_actions.write(survival_client::PlayerActionMessage(
                    survival_client::PlayerAction::Respawn,
                ));
            }
            Action::New => {
                game.input_field = 0;
                game.status.clear();
                game.screen(Mode::Create);
            }
            Action::Create => {
                if let Some(error) = &icons.error {
                    game.status.clone_from(error);
                    game.ui_dirty = true;
                    continue;
                }
                if game.name_input.trim().is_empty() {
                    game.status = "Введите название мира".into();
                    game.ui_dirty = true;
                    continue;
                }
                let nanos = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map_or(0, |d| d.as_nanos());
                let seed = if game.seed_input.trim().is_empty() {
                    nanos as u64
                } else {
                    match game.seed_input.trim().parse::<u64>() {
                        Ok(seed) => seed,
                        Err(_) => {
                            game.status = "Зерно: целое число от 0 до 18446744073709551615".into();
                            game.ui_dirty = true;
                            continue;
                        }
                    }
                };
                let ore_source = inventory_client::config_source(
                    "VOXEL_ORES_CONFIG",
                    "ores.ron",
                    include_str!("../../../assets/data/ores.ron"),
                )
                .1;
                let ore_config = match ore_source
                    .and_then(|source| game_core::ores::OreConfig::from_ron(&source))
                {
                    Ok(config) => config,
                    Err(error) => {
                        game.status = format!("Не удалось создать мир: {error}");
                        game.ui_dirty = true;
                        continue;
                    }
                };
                let id = format!("world-{nanos:x}");
                let spawn = spawn_position(seed);
                let data = SaveData {
                    format_version: game_core::persistence::CURRENT_FORMAT_VERSION,
                    generator_version: game_core::persistence::CURRENT_GENERATOR_VERSION,
                    ore_config: Some(ore_config),
                    name: game.name_input.trim().into(),
                    seed,
                    player: spawn,
                    yaw: 0.4,
                    pitch: -0.15,
                    time: 240.,
                    edits: Vec::new(),
                };
                match save_world(&settings::save_root(), &id, &data) {
                    Ok(()) => {
                        if let Err(error) =
                            open(&mut game, id, data, &mut stream, &mut commands, &mut meshes)
                        {
                            game.status = error;
                            game.ui_dirty = true;
                            continue;
                        }
                        game.game_mode = game.creation_mode;
                        let creative = game.game_mode == game_core::survival::GameMode::Creative;
                        inventory_client::reset_session(&mut game, creative);
                        // Ждём чанк и ставим на безопасную поверхность без стартового урона.
                        game.pending_respawn = true;
                    }
                    Err(e) => {
                        game.status = e;
                        game.ui_dirty = true;
                    }
                }
            }
            Action::LoadMenu => {
                match list_worlds(&settings::save_root()) {
                    Ok(worlds) => {
                        game.worlds = worlds;
                        game.status.clear();
                    }
                    Err(e) => game.status = e,
                }
                game.screen(Mode::Load);
            }
            Action::Load(id) => {
                if let Some(error) = &icons.error {
                    game.status.clone_from(error);
                    game.ui_dirty = true;
                    continue;
                }
                match load_world(&settings::save_root(), &id).and_then(|data| {
                    open(&mut game, id, data, &mut stream, &mut commands, &mut meshes)
                }) {
                    Ok(()) => {}
                    Err(e) => {
                        game.status = format!("Не удалось загрузить мир: {e}");
                        game.ui_dirty = true;
                    }
                }
            }
            Action::Resume => game.screen(Mode::Playing),
            Action::Save => {
                save(&mut game);
            }
            Action::MainMenu => {
                if save(&mut game) {
                    stream.clear(&mut commands, &mut meshes);
                    inventory_client::close_inventory(&mut game);
                    inventory_client::reset_session(&mut game, false);
                    game.world = World::new(0);
                    game.generator = Default::default();
                    game.world_id = None;
                    game.screen(Mode::Menu);
                }
            }
            Action::Exit => {
                if save(&mut game) {
                    exit.write(AppExit::Success);
                }
            }
            Action::Settings => {
                game.return_to = game.mode;
                game.screen(Mode::Settings);
            }
            Action::Back => {
                let mode = if game.mode == Mode::Settings {
                    game.return_to
                } else if game.world_id.is_some() {
                    Mode::Paused
                } else {
                    Mode::Menu
                };
                game.screen(mode);
            }
            Action::SelectSlot(slot) => {
                game.selected = slot.min(8);
                game.ui_dirty = true;
            }
            Action::RandomSeed => {
                game.seed_input = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map_or(0, |d| d.as_nanos() as u64)
                    .to_string();
                game.input_field = 1;
                game.ui_dirty = true;
            }
            Action::FocusName => {
                game.input_field = 0;
                game.ui_dirty = true;
            }
            Action::FocusSeed => {
                game.input_field = 1;
                game.ui_dirty = true;
            }
            action => {
                match action {
                    Action::CycleDistance => {
                        game.settings.distance = if game.settings.distance >= 16 {
                            4
                        } else {
                            game.settings.distance + 2
                        }
                    }
                    Action::ToggleVsync => game.settings.vsync = !game.settings.vsync,
                    Action::ToggleFullscreen => {
                        game.settings.fullscreen = !game.settings.fullscreen
                    }
                    Action::CycleResolution => {
                        game.settings.resolution = (game.settings.resolution + 1) % 3
                    }
                    Action::ToggleTextures => {
                        game.settings.pixel_textures = !game.settings.pixel_textures
                    }
                    _ => {}
                }
                apply_window(&game.settings, &mut window);
                *volume = bevy::audio::GlobalVolume::new(bevy::audio::Volume::Linear(
                    game.settings.volume,
                ));
                if let Err(e) = game.settings.save() {
                    game.status = format!("Не удалось сохранить настройки: {e}");
                }
                game.ui_dirty = true;
            }
        }
    }
}
fn autosave(time: Res<Time>, mut game: ResMut<Game>) {
    if game.mode == Mode::Playing {
        game.autosave += time.delta_secs();
        if game.autosave >= 30. {
            game.autosave = 0.; // Back off on failures too; never retry disk IO every frame.
            save(&mut game);
        }
    }
}
fn close_save(mut events: MessageReader<WindowCloseRequested>, mut intents: ResMut<UiIntent>) {
    if events.read().next().is_some() {
        intents.0.push(Action::Exit);
    }
}

/// Search near the origin for dry land before dropping the player above it.
fn spawn_position(seed: u64) -> [f32; 3] {
    for radius in 0_i32..=32 {
        for dx in -radius..=radius {
            for dz in -radius..=radius {
                if dx.abs() != radius && dz.abs() != radius {
                    continue;
                }
                let x = 8 + dx * 16;
                let z = 8 + dz * 16;
                let y = surface_height(seed, x, z);
                if y > SEA_LEVEL {
                    return [x as f32 + 0.5, (y + 10) as f32, z as f32 + 0.5];
                }
            }
        }
    }
    [
        8.5,
        (surface_height(seed, 8, 8).max(SEA_LEVEL) + 10) as f32,
        8.5,
    ]
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opening_world_uses_its_snapshot_and_invalid_snapshot_keeps_current_session() {
        let config = game_core::ores::OreConfig::from_ron(
            &include_str!("../../../assets/data/ores.ron").replace("veins: 20", "veins: 1"),
        )
        .unwrap();
        let mut data = SaveData {
            seed: 42,
            ore_config: Some(config.clone()),
            ..Default::default()
        };
        let mut game = Game::default();
        let mut stream = Streaming::default();
        let mut meshes = Assets::<Mesh>::default();
        let ecs_world = bevy::ecs::world::World::new();
        let mut queue = bevy::ecs::world::CommandQueue::default();
        let mut commands = Commands::new(&mut queue, &ecs_world);
        open(
            &mut game,
            "snapshot".into(),
            data.clone(),
            &mut stream,
            &mut commands,
            &mut meshes,
        )
        .unwrap();
        assert_eq!(game.generator.snapshot(), Some(&config));
        assert_eq!(game.save_data().ore_config, Some(config));
        let previous = game.save_data();
        let revision = game.revision;
        data.ore_config = None;
        assert!(
            open(
                &mut game,
                "invalid".into(),
                data.clone(),
                &mut stream,
                &mut commands,
                &mut meshes,
            )
            .is_err()
        );
        assert_eq!(game.save_data(), previous);
        assert_eq!(game.world_id.as_deref(), Some("snapshot"));
        assert_eq!(game.revision, revision);
        data.generator_version = 1;
        open(
            &mut game,
            "legacy".into(),
            data,
            &mut stream,
            &mut commands,
            &mut meshes,
        )
        .unwrap();
        assert_eq!(game.generator.version(), 1);
        assert!(game.save_data().ore_config.is_none());
    }

    #[test]
    fn spawn_uses_deterministic_dry_land_for_ocean_origin() {
        let spawn = spawn_position(971);
        assert_eq!(spawn, spawn_position(971));
        let ground = surface_height(971, spawn[0].floor() as i32, spawn[2].floor() as i32);
        assert!(ground > SEA_LEVEL);
        assert_eq!(spawn[1], (ground + 10) as f32);
    }
}

#[cfg(test)]
mod window_regressions {
    use super::*;
    #[test]
    fn applying_settings_does_not_resize_fullscreen_surface() {
        let settings = settings::Settings {
            fullscreen: true,
            resolution: 0,
            ..Default::default()
        };
        let mut window = Window {
            mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
            resolution: (1920, 1080).into(),
            ..default()
        };
        apply_window(&settings, &mut window);
        assert_eq!(window.resolution.physical_width(), 1920);
        assert_eq!(window.resolution.physical_height(), 1080);
        assert_eq!(
            window.mode,
            WindowMode::BorderlessFullscreen(MonitorSelection::Current)
        );
    }
}
