//! Предметы в мире: физика, срок жизни, объединение и безопасный подбор.

use std::collections::BTreeMap;

use serde::Deserialize;

use crate::{content::ContentRegistry, inventory::Inventory, items::ItemStack};

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DropConfig {
    pub pickup_delay: f32,
    pub pickup_radius: f32,
    pub lifetime: f32,
    pub max_drops: usize,
    pub gravity: f32,
    pub attraction_speed: f32,
    pub merge_radius: f32,
}

impl Default for DropConfig {
    fn default() -> Self {
        Self::from_ron(include_str!("../../../assets/data/drops.ron"))
            .expect("встроенная конфигурация предметов должна быть корректной")
    }
}

impl DropConfig {
    pub fn from_ron(source: &str) -> Result<Self, String> {
        let config: Self = ron::from_str(source).map_err(|error| error.to_string())?;
        if !config.pickup_delay.is_finite()
            || config.pickup_delay < 0.0
            || !config.pickup_radius.is_finite()
            || !(0.0..=16.0).contains(&config.pickup_radius)
            || config.pickup_radius == 0.0
            || !config.lifetime.is_finite()
            || config.lifetime <= config.pickup_delay
            || !(1..=4096).contains(&config.max_drops)
            || !config.gravity.is_finite()
            || !(0.0..=100.0).contains(&config.gravity)
            || !config.attraction_speed.is_finite()
            || !(0.0..=32.0).contains(&config.attraction_speed)
            || config.attraction_speed == 0.0
            || !config.merge_radius.is_finite()
            || !(0.0..=4.0).contains(&config.merge_radius)
        {
            return Err("некорректные параметры предметов в мире".into());
        }
        Ok(config)
    }
}

#[derive(Clone, Debug)]
pub struct Drop {
    pub id: u64,
    pub stack: ItemStack,
    pub position: [f32; 3],
    pub velocity: [f32; 3],
    pub age: f32,
    // Объединение хранит старейший срок жизни и отдельно задержку новейшей части.
    pickup_age: f32,
}

#[derive(Default, Debug)]
pub struct DropStore {
    pub entities: BTreeMap<u64, Drop>,
    next_id: u64,
}

impl DropStore {
    pub fn spawn(
        &mut self,
        stack: ItemStack,
        position: [f32; 3],
        velocity: [f32; 3],
        config: &DropConfig,
    ) -> u64 {
        let id = self.next_id;
        self.next_id = self
            .next_id
            .checked_add(1)
            .expect("исчерпаны идентификаторы предметов");
        if stack.count == 0
            || !position.iter().all(|value| value.is_finite())
            || config.max_drops == 0
        {
            return id;
        }
        while self.entities.len() >= config.max_drops {
            self.evict_oldest();
        }
        self.entities.insert(
            id,
            Drop {
                id,
                stack,
                position,
                velocity: velocity.map(|value| if value.is_finite() { value } else { 0.0 }),
                age: 0.0,
                pickup_age: 0.0,
            },
        );
        id
    }

    pub fn clear(&mut self) {
        self.entities.clear();
    }

    pub fn tick(
        &mut self,
        dt: f32,
        player: Option<[f32; 3]>,
        inventory: &mut Inventory,
        catalog: &ContentRegistry,
        config: &DropConfig,
        sample: impl Fn([i32; 3]) -> Option<u8>,
    ) -> bool {
        if !dt.is_finite() || dt <= 0.0 {
            return false;
        }
        self.entities.retain(|_, drop| {
            // Срок жизни учитывает всё прошедшее время, даже при ограничении физики.
            drop.age += dt;
            drop.pickup_age += dt;
            drop.age < config.lifetime && drop.stack.count > 0
        });
        while self.entities.len() > config.max_drops {
            self.evict_oldest();
        }
        self.merge(catalog, config, &sample);
        let player = player.filter(|point| point.iter().all(|value| value.is_finite()));
        let physics_dt = dt.min(0.25);
        let steps = (physics_dt / (1.0 / 60.0)).ceil() as usize;
        let step_dt = physics_dt / steps as f32;
        let mut changed = false;
        self.entities.retain(|_, drop| {
            for _ in 0..steps {
                drop.velocity[1] -= config.gravity * step_dt;
                if let Some(player) = player.filter(|_| drop.pickup_age >= config.pickup_delay) {
                    let distance = distance(drop.position, player);
                    if distance > 0.0
                        && distance <= config.pickup_radius
                        && path_clear(drop.position, player, &sample)
                    {
                        let speed = config.attraction_speed.min(distance / step_dt);
                        for (axis, target) in player.iter().enumerate() {
                            drop.velocity[axis] =
                                (*target - drop.position[axis]) / distance * speed;
                        }
                    }
                }
                move_drop(drop, step_dt, &sample);
            }
            if let Some(player) = player.filter(|_| drop.pickup_age >= config.pickup_delay)
                && distance(drop.position, player) <= config.pickup_radius.min(0.25)
                && path_clear(drop.position, player, &sample)
            {
                let before = drop.stack.count;
                match inventory.insert(drop.stack.clone(), catalog) {
                    None => {
                        changed = true;
                        return false;
                    }
                    Some(rest) => {
                        changed |= rest.count < before;
                        drop.stack = rest;
                    }
                }
            }
            true
        });
        changed
    }

    fn evict_oldest(&mut self) {
        let oldest = self
            .entities
            .values()
            .max_by(|left, right| {
                left.age
                    .total_cmp(&right.age)
                    .then_with(|| right.id.cmp(&left.id))
            })
            .map(|drop| drop.id);
        if let Some(id) = oldest {
            self.entities.remove(&id);
        }
    }

    fn merge(
        &mut self,
        catalog: &ContentRegistry,
        config: &DropConfig,
        sample: &impl Fn([i32; 3]) -> Option<u8>,
    ) {
        let ids: Vec<_> = self.entities.keys().copied().collect();
        for (index, &left_id) in ids.iter().enumerate() {
            for &right_id in &ids[index + 1..] {
                let (Some(left), Some(right)) =
                    (self.entities.get(&left_id), self.entities.get(&right_id))
                else {
                    continue;
                };
                if left.stack.item != right.stack.item
                    || distance(left.position, right.position) > config.merge_radius
                {
                    continue;
                }
                let transfer = catalog
                    .max_stack(left.stack.item)
                    .saturating_sub(left.stack.count)
                    .min(right.stack.count);
                if transfer == 0 || !path_clear(left.position, right.position, sample) {
                    continue;
                }
                let age = left.age.max(right.age);
                let pickup_age = left.pickup_age.min(right.pickup_age);
                let remaining = right.stack.count - transfer;
                let left = self.entities.get_mut(&left_id).unwrap();
                left.stack.count += transfer;
                left.age = age;
                left.pickup_age = pickup_age;
                if remaining == 0 {
                    self.entities.remove(&right_id);
                } else {
                    self.entities.get_mut(&right_id).unwrap().stack.count = remaining;
                }
            }
        }
    }
}

fn distance(from: [f32; 3], to: [f32; 3]) -> f32 {
    ((from[0] - to[0]).powi(2) + (from[1] - to[1]).powi(2) + (from[2] - to[2]).powi(2)).sqrt()
}

fn passable(point: [f32; 3], sample: &impl Fn([i32; 3]) -> Option<u8>) -> bool {
    sample(point.map(|value| value.floor() as i32))
        .is_some_and(|block| !crate::blocks::is_solid(block))
}

fn path_clear(from: [f32; 3], to: [f32; 3], sample: &impl Fn([i32; 3]) -> Option<u8>) -> bool {
    // Короткая трасса не даёт подбирать или объединять предметы через стену.
    let steps = (distance(from, to) / 0.1).ceil().max(1.0) as usize;
    (0..=steps).all(|step| {
        let fraction = step as f32 / steps as f32;
        passable(
            std::array::from_fn(|axis| from[axis] + (to[axis] - from[axis]) * fraction),
            sample,
        )
    })
}

fn move_drop(drop: &mut Drop, dt: f32, sample: &impl Fn([i32; 3]) -> Option<u8>) {
    for axis in 0..3 {
        // Ограничение скорости и короткий шаг исключают туннелирование и долгий цикл.
        drop.velocity[axis] = drop.velocity[axis].clamp(-32.0, 32.0);
        let movement = drop.velocity[axis] * dt;
        let steps = (movement.abs() / 0.2).ceil().max(1.0) as usize;
        for _ in 0..steps {
            let mut candidate = drop.position;
            candidate[axis] += movement / steps as f32;
            if !passable(candidate, sample) {
                drop.velocity[axis] = 0.0;
                break;
            }
            drop.position = candidate;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blocks::{AIR, STONE};

    fn config() -> DropConfig {
        DropConfig {
            gravity: 0.0,
            ..DropConfig::default()
        }
    }

    fn item(catalog: &ContentRegistry) -> crate::content::ItemId {
        catalog.item_for_block(STONE).unwrap()
    }

    fn count(inventory: &Inventory) -> u16 {
        inventory
            .hotbar
            .iter()
            .chain(inventory.main.iter())
            .flatten()
            .map(|stack| stack.count)
            .sum()
    }

    #[test]
    fn pickup_waits_for_delay_and_cannot_duplicate_on_repeated_ticks() {
        let catalog = ContentRegistry::default();
        let config = config();
        let mut inventory = Inventory::default();
        let mut store = DropStore::default();
        store.spawn(
            ItemStack::new(item(&catalog), 4),
            [0.0; 3],
            [0.0; 3],
            &config,
        );
        assert!(!store.tick(
            0.49,
            Some([0.0; 3]),
            &mut inventory,
            &catalog,
            &config,
            |_| Some(AIR)
        ));
        assert_eq!(count(&inventory), 0);
        assert!(store.tick(
            0.01,
            Some([0.0; 3]),
            &mut inventory,
            &catalog,
            &config,
            |_| Some(AIR)
        ));
        assert!(store.entities.is_empty());
        assert!(!store.tick(
            1.0,
            Some([0.0; 3]),
            &mut inventory,
            &catalog,
            &config,
            |_| Some(AIR)
        ));
        assert_eq!(count(&inventory), 4);
    }

    #[test]
    fn full_inventory_and_partial_pickup_preserve_every_item() {
        let catalog = ContentRegistry::default();
        let config = config();
        let mut inventory = Inventory::default();
        for slot in inventory.hotbar.iter_mut().chain(inventory.main.iter_mut()) {
            *slot = Some(ItemStack::new(item(&catalog), 64));
        }
        let mut store = DropStore::default();
        let id = store.spawn(
            ItemStack::new(item(&catalog), 4),
            [0.0; 3],
            [0.0; 3],
            &config,
        );
        assert!(!store.tick(
            0.5,
            Some([0.0; 3]),
            &mut inventory,
            &catalog,
            &config,
            |_| Some(AIR)
        ));
        assert_eq!(store.entities[&id].stack.count, 4);
        inventory.main[26].as_mut().unwrap().count = 63;
        assert!(store.tick(
            0.1,
            Some([0.0; 3]),
            &mut inventory,
            &catalog,
            &config,
            |_| Some(AIR)
        ));
        assert_eq!(store.entities[&id].stack.count, 3);
        assert_eq!(count(&inventory), 2304);
        assert!(!store.tick(
            0.1,
            Some([0.0; 3]),
            &mut inventory,
            &catalog,
            &config,
            |_| Some(AIR)
        ));
        assert_eq!(store.entities[&id].stack.count, 3);
    }

    #[test]
    fn nearby_merges_respect_stack_cap_and_keep_remainder() {
        let catalog = ContentRegistry::default();
        let config = config();
        let mut inventory = Inventory::default();
        let mut store = DropStore::default();
        let first = store.spawn(
            ItemStack::new(item(&catalog), 60),
            [0.0; 3],
            [0.0; 3],
            &config,
        );
        let second = store.spawn(
            ItemStack::new(item(&catalog), 10),
            [0.1, 0.0, 0.0],
            [0.0; 3],
            &config,
        );
        assert!(!store.tick(0.1, None, &mut inventory, &catalog, &config, |_| Some(AIR)));
        assert_eq!(store.entities[&first].stack.count, 64);
        assert_eq!(store.entities[&second].stack.count, 6);
    }

    #[test]
    fn merged_stack_keeps_oldest_lifetime_and_newest_pickup_delay() {
        let catalog = ContentRegistry::default();
        let mut config = config();
        config.lifetime = 10.0;
        let mut inventory = Inventory::default();
        let mut store = DropStore::default();
        let old = store.spawn(
            ItemStack::new(item(&catalog), 1),
            [0.0; 3],
            [0.0; 3],
            &config,
        );
        store.tick(9.0, None, &mut inventory, &catalog, &config, |_| Some(AIR));
        store.spawn(
            ItemStack::new(item(&catalog), 1),
            [0.0; 3],
            [0.0; 3],
            &config,
        );
        assert!(!store.tick(
            0.1,
            Some([0.0; 3]),
            &mut inventory,
            &catalog,
            &config,
            |_| Some(AIR)
        ));
        assert_eq!(store.entities.len(), 1);
        assert_eq!(store.entities[&old].stack.count, 2);
        assert!(store.entities[&old].age >= 9.0);
        assert_eq!(count(&inventory), 0);
        store.tick(0.9, None, &mut inventory, &catalog, &config, |_| Some(AIR));
        assert!(store.entities.is_empty());
    }

    #[test]
    fn distant_items_do_not_merge_or_attract() {
        let catalog = ContentRegistry::default();
        let config = config();
        let mut inventory = Inventory::default();
        let mut store = DropStore::default();
        let id = store.spawn(
            ItemStack::new(item(&catalog), 1),
            [2.0, 0.0, 0.0],
            [0.0; 3],
            &config,
        );
        store.spawn(
            ItemStack::new(item(&catalog), 1),
            [4.0, 0.0, 0.0],
            [0.0; 3],
            &config,
        );
        assert!(!store.tick(
            1.0,
            Some([0.0; 3]),
            &mut inventory,
            &catalog,
            &config,
            |_| Some(AIR)
        ));
        assert_eq!(store.entities.len(), 2);
        assert_eq!(store.entities[&id].position, [2.0, 0.0, 0.0]);
    }

    #[test]
    fn eligible_nearby_drop_moves_toward_player_and_is_picked_up() {
        let catalog = ContentRegistry::default();
        let config = config();
        let mut inventory = Inventory::default();
        let mut store = DropStore::default();
        let id = store.spawn(
            ItemStack::new(item(&catalog), 2),
            [1.4, 0.0, 0.0],
            [0.0; 3],
            &config,
        );
        store.tick(0.5, None, &mut inventory, &catalog, &config, |_| Some(AIR));
        store.tick(
            0.02,
            Some([0.0; 3]),
            &mut inventory,
            &catalog,
            &config,
            |_| Some(AIR),
        );
        assert!(store.entities[&id].position[0] < 1.4);
        for _ in 0..20 {
            store.tick(
                0.02,
                Some([0.0; 3]),
                &mut inventory,
                &catalog,
                &config,
                |_| Some(AIR),
            );
        }
        assert_eq!(count(&inventory), 2);
        assert!(store.entities.is_empty());
    }

    #[test]
    fn long_ticks_expire_using_real_time_and_cap_evicts_oldest() {
        let catalog = ContentRegistry::default();
        let mut config = config();
        config.max_drops = 2;
        let mut inventory = Inventory::default();
        let mut store = DropStore::default();
        let first = store.spawn(
            ItemStack::new(item(&catalog), 1),
            [0.0; 3],
            [0.0; 3],
            &config,
        );
        let second = store.spawn(
            ItemStack::new(item(&catalog), 1),
            [2.0, 0.0, 0.0],
            [0.0; 3],
            &config,
        );
        let third = store.spawn(
            ItemStack::new(item(&catalog), 1),
            [4.0, 0.0, 0.0],
            [0.0; 3],
            &config,
        );
        assert!(!store.entities.contains_key(&first));
        assert!(store.entities.contains_key(&second));
        assert!(store.entities.contains_key(&third));
        assert!(
            !store.tick(300.0, None, &mut inventory, &catalog, &config, |_| Some(
                AIR
            ))
        );
        assert!(store.entities.is_empty());
        let next = store.spawn(
            ItemStack::new(item(&catalog), 1),
            [0.0; 3],
            [0.0; 3],
            &config,
        );
        store.clear();
        assert!(store.entities.is_empty());
        assert!(
            store.spawn(
                ItemStack::new(item(&catalog), 1),
                [0.0; 3],
                [0.0; 3],
                &config
            ) > next
        );
    }

    #[test]
    fn gravity_cannot_tunnel_through_floor_even_at_high_speed() {
        let catalog = ContentRegistry::default();
        let config = DropConfig::default();
        let mut inventory = Inventory::default();
        let mut store = DropStore::default();
        let id = store.spawn(
            ItemStack::new(item(&catalog), 1),
            [0.5, 2.0, 0.5],
            [0.0, -1000.0, 0.0],
            &config,
        );
        for _ in 0..20 {
            store.tick(0.1, None, &mut inventory, &catalog, &config, |point| {
                Some(if point[1] < 1 { STONE } else { AIR })
            });
        }
        assert!(store.entities[&id].position[1] >= 1.0);
        assert!(store.entities[&id].position[1] < 1.3);
        assert_eq!(store.entities[&id].velocity[1], 0.0);
    }

    #[test]
    fn walls_and_unloaded_cells_block_physics_and_pickup() {
        for unloaded in [false, true] {
            let catalog = ContentRegistry::default();
            let config = config();
            let mut inventory = Inventory::default();
            let mut store = DropStore::default();
            let id = store.spawn(
                ItemStack::new(item(&catalog), 1),
                [0.5, 0.5, 0.5],
                [1000.0, 0.0, 0.0],
                &config,
            );
            for _ in 0..10 {
                store.tick(
                    0.1,
                    Some([1.2, 0.5, 0.5]),
                    &mut inventory,
                    &catalog,
                    &config,
                    |point| {
                        if point[0] >= 1 {
                            if unloaded { None } else { Some(STONE) }
                        } else {
                            Some(AIR)
                        }
                    },
                );
            }
            assert!(store.entities[&id].position[0] < 1.0);
            assert_eq!(count(&inventory), 0);
        }
    }

    #[test]
    fn invalid_dt_does_not_change_drops_or_inventory() {
        let catalog = ContentRegistry::default();
        let config = config();
        let mut inventory = Inventory::default();
        let mut store = DropStore::default();
        let id = store.spawn(
            ItemStack::new(item(&catalog), 1),
            [0.0; 3],
            [1.0; 3],
            &config,
        );
        for dt in [0.0, -1.0, f32::NAN, f32::INFINITY] {
            assert!(!store.tick(
                dt,
                Some([0.0; 3]),
                &mut inventory,
                &catalog,
                &config,
                |_| panic!("некорректный dt не должен запрашивать мир")
            ));
            assert_eq!(store.entities[&id].age, 0.0);
            assert_eq!(store.entities[&id].position, [0.0; 3]);
        }
        assert_eq!(count(&inventory), 0);
    }

    #[test]
    fn config_rejects_invalid_values_and_unknown_fields() {
        let valid = include_str!("../../../assets/data/drops.ron");
        assert!(DropConfig::from_ron(valid).is_ok());
        for (from, to) in [
            ("lifetime: 300.0", "lifetime: 0.0"),
            ("max_drops: 512", "max_drops: 0"),
            ("pickup_radius: 1.5", "pickup_radius: NaN"),
            ("gravity: 20.0", "gravity: -1.0"),
            ("pickup_delay: 0.5", "pickup_delay: 301.0"),
        ] {
            assert!(
                DropConfig::from_ron(&valid.replace(from, to)).is_err(),
                "{to}"
            );
        }
        assert!(DropConfig::from_ron(&valid.replace("gravity:", "typo:")).is_err());
    }
}
