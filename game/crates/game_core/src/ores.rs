//! Версионированная генерация четырёх руд со снимком параметров мира.

use crate::{
    COAL_ORE, Chunk, DEEP_STONE, DIAMOND_ORE, GOLD_ORE, IRON_ORE, STONE, valid_world_height,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct OreRule {
    block: u8,
    min_y: i32,
    max_y: i32,
    veins: u32,
    blocks: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OreConfig {
    ores: Vec<OreRule>,
}
impl Default for OreConfig {
    fn default() -> Self {
        Self::from_ron(include_str!("../../../assets/data/ores.ron"))
            .expect("корректные параметры руд")
    }
}
impl OreConfig {
    pub fn from_ron(source: &str) -> Result<Self, String> {
        let config: Self = ron::from_str(source).map_err(|e| e.to_string())?;
        config.validate()?;
        Ok(config)
    }
    pub(crate) fn validate(&self) -> Result<(), String> {
        let mut seen = HashSet::new();
        for ore in &self.ores {
            if !matches!(ore.block, COAL_ORE | IRON_ORE | GOLD_ORE | DIAMOND_ORE)
                || !seen.insert(ore.block)
                || !valid_world_height(ore.min_y)
                || !valid_world_height(ore.max_y)
                || ore.min_y > ore.max_y
                || !(1..=64).contains(&ore.veins)
                || !(1..=64).contains(&ore.blocks)
            {
                return Err("Некорректное или повторное правило генерации руды".into());
            }
        }
        if seen.len() != 4 {
            return Err("Нужны правила всех четырёх руд".into());
        }
        Ok(())
    }
}

/// Параметры проверяются один раз, до передачи генератора фоновым задачам.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WorldGenerator {
    ore_config: Option<OreConfig>,
}
impl WorldGenerator {
    pub fn from_snapshot(version: u32, snapshot: Option<OreConfig>) -> Result<Self, String> {
        match (version, snapshot) {
            (1, None) => Ok(Self::default()),
            (2, Some(config)) => {
                config.validate()?;
                Ok(Self {
                    ore_config: Some(config),
                })
            }
            _ => Err("Неподдерживаемый генератор или отсутствующий/лишний снимок руд".into()),
        }
    }
    #[must_use]
    pub fn version(&self) -> u32 {
        if self.ore_config.is_some() { 2 } else { 1 }
    }
    #[must_use]
    pub fn snapshot(&self) -> Option<&OreConfig> {
        self.ore_config.as_ref()
    }
    #[must_use]
    pub fn generate(&self, seed: u64, position: [i32; 2]) -> Chunk {
        let mut chunk = Chunk::generate(seed, position);
        let Some(config) = &self.ore_config else {
            return chunk;
        };
        // Пустой результат старого генератора сохраняет семантику недопустимых координат.
        if chunk.sections.is_empty() {
            return chunk;
        }
        for ore in &config.ores {
            let radius = i64::from((ore.blocks - 1).div_ceil(16));
            for x in i64::from(position[0]) - radius..=i64::from(position[0]) + radius {
                for z in i64::from(position[1]) - radius..=i64::from(position[1]) + radius {
                    if !valid_origin(x, z) {
                        continue;
                    }
                    let mut rng = Random(
                        seed ^ mix(x as u64)
                            ^ mix((z as u64).wrapping_add(0x123456789))
                            ^ mix(u64::from(ore.block)),
                    );
                    for _ in 0..ore.veins {
                        let origin = [
                            x * 16 + rng.below(16),
                            triangular_y(&mut rng, ore),
                            z * 16 + rng.below(16),
                        ];
                        for point in vein(&mut rng, ore, origin) {
                            let (Ok(x), Ok(z)) = (i32::try_from(point[0]), i32::try_from(point[2]))
                            else {
                                continue;
                            };
                            let p = [x, point[1] as i32, z];
                            if matches!(chunk.block(p), STONE | DEEP_STONE) {
                                chunk.write(p, ore.block);
                            }
                        }
                    }
                }
            }
        }
        chunk
    }
}
fn valid_origin(x: i64, z: i64) -> bool {
    [x, z]
        .into_iter()
        .all(|n| n * 16 >= i64::from(i32::MIN) && n * 16 + 15 <= i64::from(i32::MAX))
}
fn triangular_y(rng: &mut Random, ore: &OreRule) -> i64 {
    let span = i64::from(ore.max_y - ore.min_y + 1);
    i64::from(ore.min_y) + (rng.below(span) + rng.below(span)) / 2
}
fn vein(rng: &mut Random, ore: &OreRule, origin: [i64; 3]) -> Vec<[i64; 3]> {
    let mut points = Vec::with_capacity(ore.blocks as usize);
    let mut visited = HashSet::new();
    let mut frontier = vec![origin];
    while points.len() < ore.blocks as usize {
        let index = rng.below(frontier.len() as i64) as usize;
        let p = frontier.swap_remove(index);
        if !visited.insert(p) {
            continue;
        }
        points.push(p);
        for offset in [
            [1, 0, 0],
            [-1, 0, 0],
            [0, 1, 0],
            [0, -1, 0],
            [0, 0, 1],
            [0, 0, -1],
        ] {
            let q = [p[0] + offset[0], p[1] + offset[1], p[2] + offset[2]];
            if (i64::from(ore.min_y)..=i64::from(ore.max_y)).contains(&q[1])
                && !visited.contains(&q)
            {
                frontier.push(q);
            }
        }
    }
    points
}
struct Random(u64);
impl Random {
    fn below(&mut self, upper: i64) -> i64 {
        self.0 = self.0.wrapping_add(0x9e3779b97f4a7c15);
        (mix(self.0) % upper as u64) as i64
    }
}
fn mix(mut n: u64) -> u64 {
    n = (n ^ (n >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    n = (n ^ (n >> 27)).wrapping_mul(0x94d049bb133111eb);
    n ^ (n >> 31)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_rules_pin_the_four_requested_ore_bands_and_vein_sizes() {
        let actual: Vec<_> = OreConfig::default()
            .ores
            .into_iter()
            .map(|ore| (ore.block, ore.min_y, ore.max_y, ore.veins, ore.blocks))
            .collect();
        assert_eq!(
            actual,
            [
                (COAL_ORE, 0, 192, 20, 17),
                (IRON_ORE, -64, 72, 12, 9),
                (GOLD_ORE, -64, 32, 4, 9),
                (DIAMOND_ORE, -64, 16, 7, 8),
            ]
        );
    }

    #[test]
    fn triangular_origins_favor_the_middle_and_include_both_bounds() {
        let ore = &OreConfig::default().ores[0];
        let mut rng = Random(42);
        let mut counts = [0; 193];
        for _ in 0..200_000 {
            let y = triangular_y(&mut rng, ore);
            assert!((0..=192).contains(&y));
            counts[y as usize] += 1;
        }
        assert!(counts[0] > 0 && counts[192] > 0);
        let center: u32 = counts[86..107].iter().sum();
        let edge: u32 = counts[..21].iter().sum();
        assert!(center > edge * 4, "center={center}, edge={edge}");
    }

    #[test]
    fn veins_have_requested_unique_connected_candidates_even_in_one_height_band() {
        let ore = OreRule {
            block: DIAMOND_ORE,
            min_y: -40,
            max_y: -40,
            veins: 1,
            blocks: 64,
        };
        let points = vein(&mut Random(42), &ore, [-1, -40, -1]);
        assert_eq!(points.len(), 64);
        let mut reached = HashSet::new();
        for p in points {
            assert_eq!(p[1], -40);
            assert!(
                reached.is_empty()
                    || reached.iter().any(|q: &[i64; 3]| (0..3)
                        .map(|axis| (p[axis] - q[axis]).abs())
                        .sum::<i64>()
                        == 1)
            );
            assert!(reached.insert(p));
        }
    }

    #[test]
    fn neighboring_origins_reach_the_other_side_of_a_negative_seam() {
        let mut config = OreConfig::default();
        for ore in &mut config.ores {
            ore.min_y = 300;
            ore.max_y = 300;
            ore.veins = 1;
        }
        let diamond = &mut config.ores[3];
        diamond.min_y = -40;
        diamond.max_y = -40;
        diamond.blocks = 64;
        let ore = diamond.clone();
        let generator = WorldGenerator::from_snapshot(2, Some(config)).unwrap();
        let mut crossings = 0;
        for seed in 0..16 {
            let x = -1_i64;
            let z = -1_i64;
            let mut rng = Random(
                seed ^ mix(x as u64)
                    ^ mix((z as u64).wrapping_add(0x123456789))
                    ^ mix(u64::from(DIAMOND_ORE)),
            );
            let origin = [
                x * 16 + rng.below(16),
                triangular_y(&mut rng, &ore),
                z * 16 + rng.below(16),
            ];
            let points = vein(&mut rng, &ore, origin);
            let target = generator.generate(seed, [0, -1]);
            let base = Chunk::generate(seed, [0, -1]);
            for p in points {
                if p[0] >= 0 && p[0] < 16 && p[2] >= -16 && p[2] < 0 {
                    let p = p.map(|v| v as i32);
                    if matches!(base.block(p), STONE | DEEP_STONE) {
                        crossings += 1;
                        assert_eq!(target.block(p), DIAMOND_ORE, "seed={seed}, point={p:?}");
                    }
                }
            }
        }
        assert!(crossings > 0);
    }
}
