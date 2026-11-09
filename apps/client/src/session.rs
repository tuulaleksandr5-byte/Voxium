use crate::settings::Settings;
use bevy::prelude::*;
use game_core::{
    World,
    persistence::SaveData,
    survival::{GameMode, GameRules, SurvivalConfig, Vitals},
};
use game_core::{
    content::ContentRegistry,
    drops::{DropConfig, DropStore},
    inventory::{Container, Inventory},
    items::ItemStack,
};
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mode {
    Menu,
    Create,
    Load,
    Settings,
    Playing,
    Paused,
    Inventory,
    Dead,
    Console,
}

#[derive(Resource)]
pub struct Game {
    pub content: ContentRegistry,
    pub inventory: Inventory,
    pub containers: HashMap<[i32; 3], Container>,
    pub open_container: Option<[i32; 3]>,
    pub drops: DropStore,
    pub drop_config: DropConfig,
    pub mode: Mode,
    pub game_mode: GameMode,
    pub creation_mode: GameMode,
    pub vitals: Vitals,
    pub game_rules: GameRules,
    pub survival_config: SurvivalConfig,
    pub spawn: Vec3,
    pub pending_respawn: bool,
    pub fall_peak: Option<f32>,
    pub return_to: Mode,
    pub ui_dirty: bool,
    pub status: String,
    pub world: World,
    pub world_id: Option<String>,
    pub name_input: String,
    pub seed_input: String,
    pub input_field: usize,
    pub worlds: Vec<(String, String)>,
    pub feet: Vec3,
    pub velocity: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub grounded: bool,
    pub flying: bool,
    pub debug_open: bool,
    pub chunk_grid: bool,
    pub fps: f32,
    pub height: f32,
    pub hotbar: [u8; 9],
    pub selected: usize,
    pub clock: f64,
    pub autosave: f32,
    pub settings: Settings,
    pub revision: u64,
}
impl Default for Game {
    fn default() -> Self {
        Self {
            content: ContentRegistry::default(),
            inventory: Inventory::default(),
            containers: HashMap::new(),
            open_container: None,
            drops: DropStore::default(),
            drop_config: DropConfig::default(),
            mode: Mode::Menu,
            game_mode: GameMode::Survival,
            creation_mode: GameMode::Survival,
            vitals: Vitals::new(&SurvivalConfig::default()),
            game_rules: GameRules::default(),
            survival_config: SurvivalConfig::default(),
            spawn: Vec3::new(0.5, 85., 0.5),
            pending_respawn: false,
            fall_peak: None,
            return_to: Mode::Menu,
            ui_dirty: true,
            status: String::new(),
            world: World::new(0),
            world_id: None,
            name_input: "Мой мир".into(),
            seed_input: String::new(),
            input_field: 0,
            worlds: Vec::new(),
            feet: Vec3::new(0.5, 85.0, 0.5),
            velocity: Vec3::ZERO,
            yaw: 0.0,
            pitch: -0.2,
            grounded: false,
            flying: false,
            debug_open: false,
            chunk_grid: false,
            fps: 0.,
            height: 1.8,
            hotbar: [0; 9],
            selected: 0,
            clock: 240.0,
            autosave: 0.0,
            settings: Settings::load(),
            revision: 0,
        }
    }
}
impl Game {
    pub fn item_for_slot(&self, index: usize) -> Option<&ItemStack> {
        self.inventory.hotbar.get(index).and_then(Option::as_ref)
    }
    pub fn selected_block(&self) -> u8 {
        self.item_for_slot(self.selected)
            .and_then(|s| self.content.block_for_item(s.item))
            .unwrap_or(game_core::AIR)
    }
    pub fn screen(&mut self, mode: Mode) {
        if self.mode == Mode::Inventory && mode != Mode::Inventory {
            crate::inventory_client::close_inventory(self);
        }
        self.mode = mode;
        self.ui_dirty = true;
    }
    pub fn save_data(&self) -> SaveData {
        // Сундук до этапа 9 живёт в RAM; сохраняем AIR под ним, не отменяя раскопку.
        let mut edits: Vec<_> = self
            .world
            .edits
            .iter()
            .map(|(p, b)| {
                (
                    *p,
                    if *b == game_core::CHEST {
                        game_core::AIR
                    } else {
                        *b
                    },
                )
            })
            .collect();
        edits.sort_unstable();
        SaveData {
            format_version: 1,
            generator_version: 1,
            name: self.name_input.clone(),
            seed: self.world.seed,
            player: if self.vitals.is_dead() {
                self.spawn.to_array()
            } else {
                self.feet.to_array()
            },
            yaw: self.yaw,
            pitch: self.pitch,
            time: self.clock,
            edits,
        }
    }
}

#[cfg(test)]
mod inventory_save_tests {
    use super::*;

    #[test]
    fn runtime_chest_keeps_its_excavated_air_in_legacy_snapshot() {
        let mut game = Game::default();
        game.world.insert(game_core::Chunk::generate(0, [0, 0]));
        let chest = [8, 200, 8];
        let ordinary = [9, 200, 8];
        assert!(game.world.set(chest, game_core::PLANKS));
        assert!(game.world.set(chest, game_core::AIR));
        assert!(game.world.set(chest, game_core::CHEST));
        assert!(game.world.set(ordinary, game_core::PLANKS));
        let snapshot = game.save_data();
        assert!(snapshot.edits.contains(&(chest, game_core::AIR)));
        assert!(snapshot.edits.contains(&(ordinary, game_core::PLANKS)));
        assert!(snapshot.edits.iter().all(|(_, id)| *id < game_core::CHEST));
    }
}
