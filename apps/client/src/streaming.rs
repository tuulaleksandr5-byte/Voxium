use crate::session::Game;
use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task, block_on, poll_once},
};
use game_core::{AIR, Chunk, valid_world_height};
use std::collections::{HashMap, HashSet};

type MeshResult = Vec<(u8, Mesh)>;
#[derive(Resource, Default)]
pub struct Streaming {
    generation: Vec<([i32; 2], Task<Chunk>)>,
    meshing: Vec<([i32; 3], Task<MeshResult>)>,
    visuals: HashMap<[i32; 3], Vec<(Entity, Handle<Mesh>)>>,
    pub materials: Vec<Handle<game_render::VoxelMaterial>>,
    pub textures: game_render::BlockTextures,
    pub generation_count: usize,
}
impl Streaming {
    pub fn clear(&mut self, commands: &mut Commands, meshes: &mut Assets<Mesh>) {
        self.generation.clear();
        self.meshing.clear();
        for tiles in self.visuals.drain().map(|(_, v)| v) {
            for (entity, mesh) in tiles {
                commands.entity(entity).despawn();
                meshes.remove(mesh.id());
            }
        }
    }
}
fn distance(pos: [i32; 2], center: [i32; 2]) -> i64 {
    let dx = i64::from(pos[0]) - i64::from(center[0]);
    let dz = i64::from(pos[1]) - i64::from(center[1]);
    dx * dx + dz * dz
}

pub fn stream_world(
    mut commands: Commands,
    mut game: ResMut<Game>,
    mut stream: ResMut<Streaming>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if game.world_id.is_none() {
        return;
    }
    let center = [
        (game.feet.x.floor() as i32).div_euclid(16),
        (game.feet.z.floor() as i32).div_euclid(16),
    ];
    let radius = game.settings.distance;
    let removed: Vec<_> = game
        .world
        .chunks
        .keys()
        .copied()
        .filter(|p| distance(*p, center) > i64::from((radius + 1) * (radius + 1)))
        .collect();
    for pos in removed {
        game.world.unload(pos);
        let keys: Vec<_> = stream
            .visuals
            .keys()
            .copied()
            .filter(|s| s[0] == pos[0] && s[2] == pos[1])
            .collect();
        for key in keys {
            if let Some(tiles) = stream.visuals.remove(&key) {
                for (entity, mesh) in tiles {
                    commands.entity(entity).despawn();
                    meshes.remove(mesh.id());
                }
            }
        }
    }
    let mut i = 0;
    while i < stream.generation.len() {
        if let Some(chunk) = block_on(poll_once(&mut stream.generation[i].1)) {
            let (pos, _) = stream.generation.swap_remove(i);
            if distance(pos, center) <= i64::from((radius + 1) * (radius + 1)) {
                game.world.insert(chunk);
                stream.generation_count += 1;
            }
        } else {
            i += 1;
        }
    }
    if game.status.starts_with("Загрузка мира") && game.world.chunks.contains_key(&center)
    {
        game.status.clear();
    }
    let in_flight: HashSet<_> = stream.generation.iter().map(|v| v.0).collect();
    if stream.generation.len() < 2 {
        let mut candidates = Vec::new();
        for dx in -radius..=radius {
            for dz in -radius..=radius {
                let p = [center[0] + dx, center[1] + dz];
                if dx * dx + dz * dz <= radius * radius
                    && !game.world.chunks.contains_key(&p)
                    && !in_flight.contains(&p)
                {
                    candidates.push(p);
                }
            }
        }
        candidates.sort_by_key(|p| distance(*p, center));
        for pos in candidates.into_iter().take(2 - stream.generation.len()) {
            let seed = game.world.seed;
            let generator = game.generator.clone();
            stream.generation.push((
                pos,
                AsyncComputeTaskPool::get().spawn(async move { generator.generate(seed, pos) }),
            ));
        }
    }
    let mut uploaded = 0;
    let mut i = 0;
    while i < stream.meshing.len() && uploaded < 2 {
        if let Some(result) = block_on(poll_once(&mut stream.meshing[i].1)) {
            let (key, _) = stream.meshing.swap_remove(i);
            if game.world.dirty.contains(&key) || !game.world.chunks.contains_key(&[key[0], key[2]])
            {
                continue;
            }
            if let Some(tiles) = stream.visuals.remove(&key) {
                for (entity, mesh) in tiles {
                    commands.entity(entity).despawn();
                    meshes.remove(mesh.id());
                }
            }
            let mut tiles = Vec::new();
            for (id, mesh) in result {
                let handle = meshes.add(mesh);
                let entity = commands
                    .spawn((
                        Mesh3d(handle.clone()),
                        MeshMaterial3d(stream.materials[id as usize].clone()),
                        Transform::from_xyz(
                            (key[0] * 16) as f32,
                            (key[1] * 16) as f32,
                            (key[2] * 16) as f32,
                        ),
                    ))
                    .id();
                if matches!(id, 12 | 13) {
                    commands.entity(entity).insert(bevy::light::NotShadowCaster);
                }
                tiles.push((entity, handle));
            }
            stream.visuals.insert(key, tiles);
            uploaded += 1;
        } else {
            i += 1;
        }
    }
    let pending: HashSet<_> = stream.meshing.iter().map(|v| v.0).collect();
    let mut dirty: Vec<_> = game
        .world
        .dirty
        .iter()
        .copied()
        .filter(|p| !pending.contains(p))
        .collect();
    dirty.sort_by_key(|p| {
        distance([p[0], p[2]], center) * 16
            + i64::from((p[1] - (game.feet.y as i32).div_euclid(16)).abs())
    });
    for key in dirty {
        if stream.meshing.len() >= 2 {
            break;
        }
        game.world.dirty.remove(&key);
        if !game.world.chunks.contains_key(&[key[0], key[2]]) {
            continue;
        }
        let has_section = game
            .world
            .chunks
            .get(&[key[0], key[2]])
            .is_some_and(|c| c.sections.contains_key(&key[1]));
        if !has_section && !stream.visuals.contains_key(&key) {
            continue;
        }
        let chunks = game.world.chunks.clone();
        let textures = stream.textures.clone();
        stream.meshing.push((
            key,
            AsyncComputeTaskPool::get().spawn(async move {
                game_render::mesh_section_with_textures(
                    key,
                    |p| {
                        if !valid_world_height(p[1]) {
                            return AIR;
                        }
                        chunks
                            .get(&[p[0].div_euclid(16), p[2].div_euclid(16)])
                            .map_or(AIR, |chunk| chunk.block(p))
                    },
                    &textures,
                )
            }),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn background_generation_captures_world_snapshot_and_clear_cancels_queues() {
        let config = game_core::ores::OreConfig::from_ron(
            &include_str!("../../../assets/data/ores.ron").replace("veins: 20", "veins: 1"),
        )
        .unwrap();
        let generator = game_core::ores::WorldGenerator::from_snapshot(2, Some(config)).unwrap();
        let game = Game {
            generator: generator.clone(),
            world: game_core::World::new(42),
            world_id: Some("snapshot".into()),
            feet: Vec3::new(-0.5, 100., -0.5),
            ..Default::default()
        };
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .insert_resource(game)
            .init_resource::<Streaming>()
            .init_resource::<Assets<Mesh>>()
            .add_systems(Update, stream_world);
        app.update();
        let tasks = std::mem::take(&mut app.world_mut().resource_mut::<Streaming>().generation);
        assert_eq!(tasks.len(), 2);
        app.world_mut().resource_mut::<Game>().generator = Default::default();
        for (pos, task) in tasks {
            assert_eq!(block_on(task), generator.generate(42, pos));
        }
        app.update();
        let mut stream = app.world_mut().remove_resource::<Streaming>().unwrap();
        assert!(!stream.generation.is_empty());
        stream.meshing.push((
            [0, 0, 0],
            AsyncComputeTaskPool::get().spawn(std::future::pending()),
        ));
        let mut meshes = app.world_mut().remove_resource::<Assets<Mesh>>().unwrap();
        let mut queue = bevy::ecs::world::CommandQueue::default();
        let mut commands = Commands::new(&mut queue, app.world());
        stream.clear(&mut commands, &mut meshes);
        assert!(stream.generation.is_empty());
        assert!(stream.meshing.is_empty());
    }
}
