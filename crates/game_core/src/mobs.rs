//! Мобы: поведение, физика и бой без зависимости от Bevy.
//!
//! Клиент передаёт выборку блоков и положение игрока, а обратно получает
//! события (урон игроку, выпавшие предметы). Отрисовка и анимация читают
//! публичные поля `Mob`: фазу шага, скорость, вспышку урона и таймер гибели.

use crate::{
    BEDROCK, GRASS, WATER, WORLD_MIN_Y, interaction::move_player, is_leaves, is_solid,
    valid_world_height,
};
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MobKind {
    Cow,
    Pig,
    Sheep,
    Chicken,
    Ghoul,
    Spider,
}

#[derive(Debug)]
pub struct MobSpec {
    pub id: &'static str,
    pub name: &'static str,
    pub hostile: bool,
    pub max_health: f32,
    pub speed: f32,
    /// Высота тела для столкновений с блоками.
    pub height: f32,
    /// Полуширина области попадания удара игрока.
    pub half_width: f32,
    pub attack: f32,
    /// (предмет, минимум, максимум) при гибели.
    pub drops: &'static [(&'static str, u16, u16)],
}

impl MobKind {
    pub const ALL: [Self; 6] = [
        Self::Cow,
        Self::Pig,
        Self::Sheep,
        Self::Chicken,
        Self::Ghoul,
        Self::Spider,
    ];
    const PASSIVE: [Self; 4] = [Self::Cow, Self::Pig, Self::Sheep, Self::Chicken];

    #[must_use]
    pub fn spec(self) -> &'static MobSpec {
        match self {
            Self::Cow => &MobSpec {
                id: "cow",
                name: "Корова",
                hostile: false,
                max_health: 10.0,
                speed: 1.6,
                height: 1.4,
                half_width: 0.45,
                attack: 0.0,
                drops: &[("survival:leather", 0, 2), ("builder:raw_beef", 1, 3)],
            },
            Self::Pig => &MobSpec {
                id: "pig",
                name: "Свинья",
                hostile: false,
                max_health: 10.0,
                speed: 1.8,
                height: 0.9,
                half_width: 0.45,
                attack: 0.0,
                drops: &[("builder:raw_pork", 1, 3)],
            },
            Self::Sheep => &MobSpec {
                id: "sheep",
                name: "Овца",
                hostile: false,
                max_health: 8.0,
                speed: 1.7,
                height: 1.2,
                half_width: 0.45,
                attack: 0.0,
                drops: &[("builder:wool", 1, 1), ("builder:raw_mutton", 1, 2)],
            },
            Self::Chicken => &MobSpec {
                id: "chicken",
                name: "Курица",
                hostile: false,
                max_health: 4.0,
                speed: 1.5,
                height: 0.8,
                half_width: 0.3,
                attack: 0.0,
                drops: &[("builder:feather", 0, 2), ("builder:raw_chicken", 1, 1)],
            },
            Self::Ghoul => &MobSpec {
                id: "ghoul",
                name: "Гуль",
                hostile: true,
                max_health: 20.0,
                speed: 2.5,
                height: 1.85,
                half_width: 0.4,
                attack: 3.0,
                drops: &[("builder:rotten_meat", 0, 2)],
            },
            Self::Spider => &MobSpec {
                id: "spider",
                name: "Паук",
                hostile: true,
                max_health: 14.0,
                speed: 3.1,
                height: 0.8,
                half_width: 0.6,
                attack: 2.0,
                drops: &[("builder:string", 0, 2)],
            },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Behavior {
    Idle,
    Wander,
    Flee,
    Chase,
}

#[derive(Clone, Debug)]
pub struct Mob {
    pub id: u64,
    pub kind: MobKind,
    pub position: [f32; 3],
    pub velocity: [f32; 3],
    /// Направление тела; при нуле модель смотрит на −Z.
    pub yaw: f32,
    /// Поворот головы относительно тела.
    pub head_yaw: f32,
    pub head_pitch: f32,
    pub health: f32,
    pub behavior: Behavior,
    /// Пройденный путь: фаза шага для анимации ног.
    pub walk: f32,
    /// Фактическая горизонтальная скорость за последний шаг.
    pub speed: f32,
    pub grounded: bool,
    /// Секунды вспышки после полученного удара.
    pub hurt: f32,
    /// Секунды с момента гибели; `None` — жив.
    pub dying: Option<f32>,
    /// Секунды замаха после атаки по игроку.
    pub swing: f32,
    pub age: f32,
    target_yaw: f32,
    look_yaw: f32,
    timer: f32,
    cooldown: f32,
    angry: f32,
    egg: f32,
    threat: [f32; 2],
    impulse: [f32; 2],
}

/// Сколько секунд длится анимация гибели до удаления моба.
pub const DEATH_SECONDS: f32 = 0.9;
const GRAVITY: f32 = 26.0;
const JUMP_SPEED: f32 = 8.2;
const SIGHT: f32 = 18.0;
const DESPAWN_DISTANCE: f32 = 96.0;
const MAX_PASSIVE: usize = 12;
const MAX_HOSTILE: usize = 8;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MobEvents {
    /// Суммарный урон игроку за шаг.
    pub player_damage: f32,
    /// Горизонтальный толчок игроку от удара.
    pub knockback: [f32; 3],
    /// (предмет, количество, позиция) для создания дропа.
    pub drops: Vec<(&'static str, u16, [f32; 3])>,
}

#[derive(Clone, Debug)]
pub struct MobStore {
    pub mobs: BTreeMap<u64, Mob>,
    next_id: u64,
    rng: Random,
    spawn_timer: f32,
    pending_drops: Vec<(&'static str, u16, [f32; 3])>,
}
impl Default for MobStore {
    fn default() -> Self {
        Self::new(0x5EED)
    }
}

impl MobStore {
    #[must_use]
    pub fn new(seed: u64) -> Self {
        Self {
            mobs: BTreeMap::new(),
            next_id: 1,
            rng: Random(seed | 1),
            spawn_timer: 2.0,
            pending_drops: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.mobs.clear();
        self.pending_drops.clear();
        self.spawn_timer = 2.0;
    }

    pub fn spawn(&mut self, kind: MobKind, position: [f32; 3]) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        let yaw = self.rng.range(-std::f32::consts::PI, std::f32::consts::PI);
        let egg = self.rng.range(40.0, 110.0);
        self.mobs.insert(
            id,
            Mob {
                id,
                kind,
                position,
                velocity: [0.0; 3],
                yaw,
                head_yaw: 0.0,
                head_pitch: 0.0,
                health: kind.spec().max_health,
                behavior: Behavior::Idle,
                walk: 0.0,
                speed: 0.0,
                grounded: false,
                hurt: 0.0,
                dying: None,
                swing: 0.0,
                age: 0.0,
                target_yaw: yaw,
                look_yaw: 0.0,
                timer: 1.0,
                cooldown: 0.0,
                angry: 0.0,
                egg,
                threat: [0.0; 2],
                impulse: [0.0; 2],
            },
        );
        id
    }

    /// Ближайший живой моб на луче взгляда: (ID, расстояние).
    #[must_use]
    pub fn raycast(&self, origin: [f32; 3], direction: [f32; 3], reach: f32) -> Option<(u64, f32)> {
        let mut best: Option<(u64, f32)> = None;
        for mob in self.mobs.values().filter(|mob| mob.dying.is_none()) {
            let spec = mob.kind.spec();
            let min = [
                mob.position[0] - spec.half_width,
                mob.position[1],
                mob.position[2] - spec.half_width,
            ];
            let max = [
                mob.position[0] + spec.half_width,
                mob.position[1] + spec.height,
                mob.position[2] + spec.half_width,
            ];
            let (mut near, mut far) = (0.0_f32, reach);
            let mut hit = true;
            for axis in 0..3 {
                if direction[axis].abs() < 1e-6 {
                    if origin[axis] < min[axis] || origin[axis] > max[axis] {
                        hit = false;
                        break;
                    }
                    continue;
                }
                let a = (min[axis] - origin[axis]) / direction[axis];
                let b = (max[axis] - origin[axis]) / direction[axis];
                near = near.max(a.min(b));
                far = far.min(a.max(b));
                if near > far {
                    hit = false;
                    break;
                }
            }
            if hit && best.is_none_or(|(_, distance)| near < distance) {
                best = Some((mob.id, near));
            }
        }
        best
    }

    /// Удар игрока. Возвращает `true`, если моб получил урон.
    pub fn hit(&mut self, id: u64, damage: f32, from: [f32; 3]) -> bool {
        let Some(mob) = self.mobs.get_mut(&id) else {
            return false;
        };
        if mob.dying.is_some() || mob.hurt > 0.2 || !damage.is_finite() || damage <= 0.0 {
            return false;
        }
        mob.health -= damage;
        mob.hurt = 0.4;
        let (dx, dz) = (mob.position[0] - from[0], mob.position[2] - from[2]);
        let length = (dx * dx + dz * dz).sqrt().max(0.01);
        mob.impulse = [dx / length * 7.0, dz / length * 7.0];
        if mob.grounded {
            mob.velocity[1] = 5.0;
        }
        mob.threat = [from[0], from[2]];
        mob.angry = 30.0;
        if !mob.kind.spec().hostile {
            mob.behavior = Behavior::Flee;
            mob.timer = 5.0;
        }
        if mob.health <= 0.0 {
            mob.dying = Some(0.0);
            let center = [
                mob.position[0],
                mob.position[1] + mob.kind.spec().height * 0.5,
                mob.position[2],
            ];
            for (item, min, max) in mob.kind.spec().drops {
                let count = *min + (self.rng.below(u64::from(max - min) + 1)) as u16;
                if count > 0 {
                    self.pending_drops.push((item, count, center));
                }
            }
        }
        true
    }

    /// Один шаг симуляции. `sample` возвращает `None` для незагруженных чанков:
    /// мобы там замирают и не появляются. `daylight` — 0 (ночь) … 1 (полдень).
    pub fn tick(
        &mut self,
        dt: f32,
        player: [f32; 3],
        player_targetable: bool,
        daylight: f32,
        sample: impl Fn([i32; 3]) -> Option<u8>,
    ) -> MobEvents {
        let mut events = MobEvents {
            drops: std::mem::take(&mut self.pending_drops),
            ..Default::default()
        };
        if !dt.is_finite() || dt <= 0.0 {
            return events;
        }
        let dt = dt.min(0.05);
        let solid = |p: [i32; 3]| sample(p).unwrap_or(BEDROCK);
        let mut removed = Vec::new();
        for mob in self.mobs.values_mut() {
            let spec = mob.kind.spec();
            let cell = mob.position.map(|v| v.floor() as i32);
            let offset = [
                player[0] - mob.position[0],
                player[1] - mob.position[1],
                player[2] - mob.position[2],
            ];
            let flat = (offset[0] * offset[0] + offset[2] * offset[2]).sqrt();
            if flat > DESPAWN_DISTANCE || mob.position[1] < (WORLD_MIN_Y - 16) as f32 {
                removed.push(mob.id);
                continue;
            }
            if sample(cell).is_none() && valid_world_height(cell[1]) {
                continue;
            }
            mob.age += dt;
            mob.hurt = (mob.hurt - dt).max(0.0);
            mob.swing = (mob.swing - dt).max(0.0);
            mob.cooldown = (mob.cooldown - dt).max(0.0);
            mob.angry = (mob.angry - dt).max(0.0);
            if let Some(time) = &mut mob.dying {
                *time += dt;
                if *time >= DEATH_SECONDS {
                    removed.push(mob.id);
                }
                mob.speed = 0.0;
                continue;
            }
            // Гули не переносят солнечный свет под открытым небом.
            if mob.kind == MobKind::Ghoul
                && daylight > 0.35
                && (1..24).all(|dy| !is_solid(solid([cell[0], cell[1] + dy, cell[2]])))
            {
                mob.health -= 3.0 * dt;
                mob.hurt = mob.hurt.max(0.12);
                if mob.health <= 0.0 {
                    mob.dying = Some(0.0);
                    continue;
                }
            }
            let aggressive = spec.hostile
                && player_targetable
                && (mob.kind != MobKind::Spider || daylight < 0.3 || mob.angry > 0.0);
            if aggressive && flat < SIGHT && offset[1].abs() < 8.0 {
                mob.behavior = Behavior::Chase;
            } else if mob.behavior == Behavior::Chase {
                mob.behavior = Behavior::Idle;
                mob.timer = 1.0;
            }
            let mut wanted = 0.0;
            match mob.behavior {
                Behavior::Chase => {
                    mob.target_yaw = (-offset[0]).atan2(-offset[2]);
                    mob.look_yaw = 0.0;
                    wanted = if flat > 0.9 { spec.speed } else { 0.0 };
                    if flat < 1.35 && offset[1].abs() < 1.7 && mob.cooldown <= 0.0 {
                        mob.cooldown = 1.0;
                        mob.swing = 0.35;
                        events.player_damage += spec.attack;
                        let length = flat.max(0.01);
                        events.knockback =
                            [offset[0] / length * 6.0, 4.0, offset[2] / length * 6.0];
                    }
                }
                Behavior::Flee => {
                    let (dx, dz) = (
                        mob.position[0] - mob.threat[0],
                        mob.position[2] - mob.threat[1],
                    );
                    mob.target_yaw = (-dx).atan2(-dz);
                    mob.look_yaw = 0.0;
                    wanted = spec.speed * 1.8;
                    mob.timer -= dt;
                    if mob.timer <= 0.0 {
                        mob.behavior = Behavior::Idle;
                        mob.timer = self.rng.range(1.0, 3.0);
                    }
                }
                Behavior::Idle => {
                    mob.timer -= dt;
                    if mob.timer <= 0.0 {
                        if self.rng.unit() < 0.65 {
                            mob.behavior = Behavior::Wander;
                            mob.target_yaw = mob.yaw + self.rng.range(-2.4, 2.4);
                            mob.timer = self.rng.range(2.0, 5.0);
                            mob.look_yaw = 0.0;
                        } else {
                            mob.timer = self.rng.range(1.5, 4.0);
                            mob.look_yaw = self.rng.range(-0.9, 0.9);
                        }
                    }
                }
                Behavior::Wander => {
                    wanted = spec.speed * 0.55;
                    mob.timer -= dt;
                    if mob.timer <= 0.0 {
                        mob.behavior = Behavior::Idle;
                        mob.timer = self.rng.range(1.5, 5.0);
                    }
                }
            }
            // Мирные мобы видят игрока рядом и поворачивают к нему голову.
            if matches!(mob.behavior, Behavior::Idle) && flat < 6.0 && !spec.hostile {
                let to_player = (-offset[0]).atan2(-offset[2]);
                mob.look_yaw = wrap_angle(to_player - mob.yaw).clamp(-1.1, 1.1);
            }
            mob.yaw += wrap_angle(mob.target_yaw - mob.yaw).clamp(-7.0 * dt, 7.0 * dt);
            mob.yaw = wrap_angle(mob.yaw);
            mob.head_yaw += (mob.look_yaw - mob.head_yaw) * (dt * 6.0).min(1.0);
            let pitch = if mob.behavior == Behavior::Chase || flat < 6.0 {
                (offset[1] + 1.5 - spec.height)
                    .atan2(flat.max(0.5))
                    .clamp(-0.6, 0.6)
            } else {
                0.0
            };
            mob.head_pitch += (pitch - mob.head_pitch) * (dt * 6.0).min(1.0);

            let direction = [-mob.yaw.sin(), -mob.yaw.cos()];
            let in_water = solid(cell) == WATER;
            if wanted > 0.0 && mob.behavior != Behavior::Chase && mob.grounded {
                // Не шагать с обрыва и не заходить в воду во время прогулки.
                let ahead = [
                    (mob.position[0] + direction[0] * 0.9).floor() as i32,
                    cell[1],
                    (mob.position[2] + direction[1] * 0.9).floor() as i32,
                ];
                let drop =
                    (0..=3).all(|dy| !is_solid(solid([ahead[0], ahead[1] - dy - 1, ahead[2]])));
                let wet =
                    solid([ahead[0], ahead[1] - 1, ahead[2]]) == WATER || solid(ahead) == WATER;
                if drop || (wet && !in_water) {
                    mob.target_yaw = mob.yaw + self.rng.range(1.6, 3.1);
                    wanted = 0.0;
                }
            }
            if in_water {
                wanted *= 0.6;
            }
            let decay = (-5.0 * dt).exp();
            mob.impulse = [mob.impulse[0] * decay, mob.impulse[1] * decay];
            mob.velocity[0] = direction[0] * wanted + mob.impulse[0];
            mob.velocity[2] = direction[1] * wanted + mob.impulse[1];
            if in_water {
                mob.velocity[1] = (mob.velocity[1] + 30.0 * dt).min(2.2);
            } else {
                mob.velocity[1] -= GRAVITY * dt;
                if mob.kind == MobKind::Chicken {
                    mob.velocity[1] = mob.velocity[1].max(-2.5);
                }
            }
            mob.velocity[1] = mob.velocity[1].max(-60.0);
            let requested = mob.velocity;
            let moved = move_player(mob.position, mob.velocity, dt, spec.height, solid);
            let step = [
                moved.position[0] - mob.position[0],
                moved.position[2] - mob.position[2],
            ];
            let distance = (step[0] * step[0] + step[1] * step[1]).sqrt();
            mob.walk += distance;
            mob.speed = distance / dt;
            mob.position = moved.position;
            mob.velocity = moved.velocity;
            mob.grounded = moved.grounded;
            let blocked = (requested[0].abs() > 0.05 && moved.velocity[0] == 0.0)
                || (requested[2].abs() > 0.05 && moved.velocity[2] == 0.0);
            if blocked && wanted > 0.0 && (mob.grounded || in_water) {
                let ahead = [
                    (mob.position[0] + direction[0] * 0.8).floor() as i32,
                    mob.position[1].floor() as i32 + 1,
                    (mob.position[2] + direction[1] * 0.8).floor() as i32,
                ];
                let wall = is_solid(solid(ahead))
                    || is_solid(solid([
                        ahead[0],
                        ahead[1] + spec.height.ceil() as i32,
                        ahead[2],
                    ]));
                if wall && mob.behavior != Behavior::Chase {
                    mob.target_yaw = mob.yaw + self.rng.range(1.2, 3.1);
                } else {
                    mob.velocity[1] = JUMP_SPEED;
                }
            }
            if mob.kind == MobKind::Chicken {
                mob.egg -= dt;
                if mob.egg <= 0.0 {
                    mob.egg = self.rng.range(60.0, 140.0);
                    events.drops.push(("survival:egg", 1, mob.position));
                }
            }
        }
        for id in removed {
            self.mobs.remove(&id);
        }
        self.spawn_timer -= dt;
        if self.spawn_timer <= 0.0 {
            self.spawn_timer = 1.5;
            self.try_spawn(player, daylight, &sample);
        }
        events
    }

    fn try_spawn(
        &mut self,
        player: [f32; 3],
        daylight: f32,
        sample: &impl Fn([i32; 3]) -> Option<u8>,
    ) {
        let hostile = self
            .mobs
            .values()
            .filter(|mob| mob.kind.spec().hostile)
            .count();
        let passive = self.mobs.len() - hostile;
        let night = daylight < 0.12;
        if (night && hostile >= MAX_HOSTILE)
            || (!night && (passive >= MAX_PASSIVE || daylight < 0.3))
        {
            return;
        }
        let angle = self.rng.range(0.0, std::f32::consts::TAU);
        let radius = self.rng.range(if night { 20.0 } else { 14.0 }, 44.0);
        let x = (player[0] + angle.cos() * radius).floor() as i32;
        let z = (player[2] + angle.sin() * radius).floor() as i32;
        let top = player[1].floor() as i32 + 20;
        let Some(ground) = (top - 44..=top).rev().find(|&y| {
            sample([x, y, z]).is_some_and(is_solid)
                && sample([x, y + 1, z]) == Some(crate::AIR)
                && sample([x, y + 2, z]) == Some(crate::AIR)
        }) else {
            return;
        };
        let Some(block) = sample([x, ground, z]) else {
            return;
        };
        let feet = [x as f32 + 0.5, (ground + 1) as f32, z as f32 + 0.5];
        if night {
            if is_leaves(block) {
                return;
            }
            let kind = if self.rng.unit() < 0.6 {
                MobKind::Ghoul
            } else {
                MobKind::Spider
            };
            self.spawn(kind, feet);
        } else if block == GRASS {
            let kind = MobKind::PASSIVE[self.rng.below(4) as usize];
            let group = 1 + self.rng.below(3) as usize;
            for _ in 0..group.min(MAX_PASSIVE - passive) {
                self.spawn(kind, feet);
            }
        }
    }
}

fn wrap_angle(angle: f32) -> f32 {
    (angle + std::f32::consts::PI).rem_euclid(std::f32::consts::TAU) - std::f32::consts::PI
}

#[derive(Clone, Debug)]
struct Random(u64);
impl Random {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0.wrapping_mul(0x2545_f491_4f6c_dd1d)
    }
    fn unit(&mut self) -> f32 {
        (self.next() >> 40) as f32 / (1_u64 << 24) as f32
    }
    fn range(&mut self, low: f32, high: f32) -> f32 {
        low + (high - low) * self.unit()
    }
    fn below(&mut self, bound: u64) -> u64 {
        (self.next() >> 33) % bound.max(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AIR, STONE};

    /// Плоский мир: трава на y = 63, выше воздух.
    fn flat(p: [i32; 3]) -> Option<u8> {
        Some(match p[1] {
            ..=62 => STONE,
            63 => GRASS,
            _ => AIR,
        })
    }
    const FAR: [f32; 3] = [40.0, 64.0, 40.0];

    #[test]
    fn mobs_fall_onto_the_ground_and_stay_inside_the_world() {
        let mut store = MobStore::new(7);
        let id = store.spawn(MobKind::Cow, [0.5, 70.0, 0.5]);
        for _ in 0..400 {
            store.tick(0.05, FAR, false, 1.0, flat);
        }
        let mob = &store.mobs[&id];
        assert!(mob.grounded);
        assert!((mob.position[1] - 64.0).abs() < 0.01, "{:?}", mob.position);
        assert!(mob.walk > 0.0, "за 20 секунд корова должна пройтись");
    }

    #[test]
    fn hit_makes_passive_mob_flee_and_death_drops_loot_once() {
        let mut store = MobStore::new(3);
        let id = store.spawn(MobKind::Pig, [0.5, 64.0, 0.5]);
        store.tick(0.05, FAR, false, 1.0, flat);
        assert!(store.hit(id, 4.0, [3.5, 64.0, 0.5]));
        assert!(
            !store.hit(id, 4.0, [3.5, 64.0, 0.5]),
            "неуязвимость после удара"
        );
        assert_eq!(store.mobs[&id].behavior, Behavior::Flee);
        for _ in 0..20 {
            store.tick(0.05, FAR, false, 1.0, flat);
        }
        assert!(store.mobs[&id].position[0] < 0.5, "убегает от удара");
        assert!(store.hit(id, 100.0, [3.5, 64.0, 0.5]));
        assert!(store.mobs[&id].dying.is_some());
        assert!(!store.hit(id, 1.0, [0.0; 3]));
        let events = store.tick(0.05, FAR, false, 1.0, flat);
        let pork: u16 = events
            .drops
            .iter()
            .filter(|drop| drop.0 == "builder:raw_pork")
            .map(|drop| drop.1)
            .sum();
        assert!((1..=3).contains(&pork));
        let mut later = MobEvents::default();
        for _ in 0..40 {
            later = store.tick(0.05, FAR, false, 1.0, flat);
        }
        assert!(later.drops.is_empty());
        assert!(
            !store.mobs.contains_key(&id),
            "тело исчезает после анимации"
        );
    }

    #[test]
    fn ghoul_chases_and_damages_player_at_night_but_ignores_creative() {
        let mut store = MobStore::new(11);
        let id = store.spawn(MobKind::Ghoul, [0.5, 64.0, 0.5]);
        let player = [6.5, 64.0, 0.5];
        let mut damage = 0.0;
        for _ in 0..200 {
            damage += store.tick(0.05, player, true, 0.0, flat).player_damage;
        }
        assert!(damage >= 3.0, "гуль должен дойти и ударить");
        assert_eq!(store.mobs[&id].behavior, Behavior::Chase);
        let mut store = MobStore::new(11);
        store.spawn(MobKind::Ghoul, [0.5, 64.0, 0.5]);
        let quiet: f32 = (0..200)
            .map(|_| store.tick(0.05, player, false, 0.0, flat).player_damage)
            .sum();
        assert_eq!(quiet, 0.0);
    }

    #[test]
    fn ghoul_burns_in_daylight_and_spider_is_calm_until_provoked() {
        let mut store = MobStore::new(5);
        let ghoul = store.spawn(MobKind::Ghoul, [0.5, 64.0, 0.5]);
        let spider = store.spawn(MobKind::Spider, [20.5, 64.0, 0.5]);
        let player = [26.5, 64.0, 0.5];
        for _ in 0..200 {
            store.tick(0.05, player, true, 1.0, flat);
        }
        assert!(!store.mobs.contains_key(&ghoul));
        assert_ne!(store.mobs[&spider].behavior, Behavior::Chase);
        assert!(store.hit(spider, 1.0, player));
        store.tick(0.05, player, true, 1.0, flat);
        assert_eq!(store.mobs[&spider].behavior, Behavior::Chase);
    }

    #[test]
    fn raycast_returns_the_nearest_living_mob() {
        let mut store = MobStore::new(1);
        let near = store.spawn(MobKind::Sheep, [0.5, 64.0, -3.0]);
        let far = store.spawn(MobKind::Sheep, [0.5, 64.0, -5.0]);
        let eye = [0.5, 64.6, 0.0];
        let hit = store.raycast(eye, [0.0, 0.0, -1.0], 6.0).unwrap();
        assert_eq!(hit.0, near);
        assert!((hit.1 - 2.55).abs() < 0.01);
        assert!(store.raycast(eye, [0.0, 0.0, 1.0], 6.0).is_none());
        assert!(store.raycast(eye, [0.0, 0.0, -1.0], 2.0).is_none());
        store.hit(near, 100.0, eye);
        assert_eq!(store.raycast(eye, [0.0, 0.0, -1.0], 6.0).unwrap().0, far);
    }

    #[test]
    fn spawning_respects_time_of_day_caps_and_unloaded_chunks() {
        let player = [0.5, 64.0, 0.5];
        let mut day = MobStore::new(9);
        for _ in 0..4000 {
            day.tick(0.05, player, false, 1.0, flat);
        }
        assert!(!day.mobs.is_empty());
        assert!(day.mobs.len() <= MAX_PASSIVE);
        assert!(day.mobs.values().all(|mob| !mob.kind.spec().hostile));
        let mut night = MobStore::new(9);
        for _ in 0..4000 {
            night.tick(0.05, player, false, 0.0, flat);
        }
        assert!(!night.mobs.is_empty());
        assert!(night.mobs.len() <= MAX_HOSTILE);
        assert!(night.mobs.values().all(|mob| mob.kind.spec().hostile));
        let mut unloaded = MobStore::new(9);
        for _ in 0..2000 {
            unloaded.tick(0.05, player, false, 1.0, |_| None);
        }
        assert!(unloaded.mobs.is_empty());
    }

    #[test]
    fn every_drop_refers_to_a_catalog_item() {
        let catalog = crate::content::ContentRegistry::default();
        for kind in MobKind::ALL {
            for (item, min, max) in kind.spec().drops {
                assert!(catalog.find(item).is_some(), "{item}");
                assert!(min <= max);
            }
        }
        assert!(catalog.find("survival:egg").is_some());
    }
}
