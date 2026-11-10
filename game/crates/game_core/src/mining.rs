//! Время разрушения и право на дроп вычисляются независимо.
use crate::{
    AIR, BEDROCK, BLOCKS, WATER,
    content::{ContentRegistry, ItemId, ItemKind},
};
use serde::Deserialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
pub enum ToolKind {
    Hand,
    Pickaxe,
    Axe,
    Shovel,
    Hoe,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tool {
    Hand,
    Equipped { material: usize, kind: ToolKind },
}
impl Tool {
    pub fn from_item(item: Option<ItemId>, catalog: &ContentRegistry) -> Self {
        let Some(def) = item.and_then(|id| catalog.get(id)) else {
            return Self::Hand;
        };
        if def.kind != ItemKind::Tool {
            return Self::Hand;
        }
        let Some(name) = def.id.strip_prefix("survival:") else {
            return Self::Hand;
        };
        let Some((material, kind)) = name.split_once('_') else {
            return Self::Hand;
        };
        let Some(material) = ["wooden", "stone", "iron", "diamond", "golden"]
            .iter()
            .position(|value| *value == material)
        else {
            return Self::Hand;
        };
        let kind = match kind {
            "pickaxe" => ToolKind::Pickaxe,
            "axe" => ToolKind::Axe,
            "shovel" => ToolKind::Shovel,
            "hoe" => ToolKind::Hoe,
            _ => return Self::Hand,
        };
        Self::Equipped { material, kind }
    }
    fn column(self, kind: ToolKind) -> usize {
        match self {
            Self::Equipped {
                material,
                kind: actual,
            } if actual == kind && material < 5 => material + 1,
            _ => 0,
        }
    }
    fn tier(self, kind: ToolKind) -> u8 {
        match self.column(kind) {
            1 | 5 => 1,
            2 => 2,
            3 => 3,
            4 => 4,
            _ => 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
struct MiningRule {
    block: u8,
    hardness: f32,
    tool: ToolKind,
    harvest_tier: u8,
    seconds: Option<[f32; 6]>,
}
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MiningConfig {
    speeds: [f32; 5],
    harvest_factor: f32,
    no_harvest_factor: f32,
    rules: Vec<MiningRule>,
}
impl Default for MiningConfig {
    fn default() -> Self {
        Self::from_ron(include_str!("../../../assets/data/mining.ron"))
            .expect("корректные параметры добычи")
    }
}
impl MiningConfig {
    pub fn from_ron(source: &str) -> Result<Self, String> {
        let config: Self = ron::from_str(source).map_err(|e| e.to_string())?;
        let valid = |n: f32| n.is_finite() && n > 0.;
        if !config.speeds.iter().copied().all(valid)
            || !valid(config.harvest_factor)
            || !valid(config.no_harvest_factor)
        {
            return Err("Некорректные множители добычи".into());
        }
        let mut seen = [false; BLOCKS.len()];
        for rule in &config.rules {
            let id = usize::from(rule.block);
            if id >= BLOCKS.len()
                || matches!(rule.block, AIR | WATER | BEDROCK)
                || seen[id]
                || !valid(rule.hardness)
                || rule.harvest_tier > 4
                || rule.seconds.is_some_and(|s| !s.into_iter().all(valid))
            {
                return Err("Некорректное или повторное правило добычи".into());
            }
            seen[id] = true;
        }
        if BLOCKS
            .iter()
            .any(|b| !matches!(b.id, AIR | WATER | BEDROCK) && !seen[usize::from(b.id)])
        {
            return Err("Отсутствует правило добычи блока".into());
        }
        for rule in &config.rules {
            for tool in std::iter::once(Tool::Hand).chain((0..5).map(|material| Tool::Equipped {
                material,
                kind: rule.tool,
            })) {
                if !config.duration(rule.block, tool).is_some_and(valid) {
                    return Err(
                        "Вычисленное время добычи должно быть конечным и положительным".into(),
                    );
                }
            }
        }
        Ok(config)
    }
    fn rule(&self, block: u8) -> Option<&MiningRule> {
        self.rules.iter().find(|r| r.block == block)
    }
    pub fn can_harvest(&self, block: u8, tool: Tool) -> bool {
        self.rule(block)
            .is_some_and(|r| r.harvest_tier == 0 || tool.tier(r.tool) >= r.harvest_tier)
    }
    pub fn duration(&self, block: u8, tool: Tool) -> Option<f32> {
        let rule = self.rule(block)?;
        let column = tool.column(rule.tool);
        if let Some(seconds) = rule.seconds {
            return Some(seconds[column]);
        }
        let speed = if column == 0 {
            1.
        } else {
            self.speeds[column - 1]
        };
        let factor = if self.can_harvest(block, tool) {
            self.harvest_factor
        } else {
            self.no_harvest_factor
        };
        Some(rule.hardness * factor / speed)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MiningTarget {
    pub position: [i32; 3],
    pub block: u8,
    pub item: Option<ItemId>,
    pub slot: usize,
}
#[derive(Clone, Debug, Default)]
pub struct MiningState {
    target: Option<MiningTarget>,
    elapsed: f32,
    sent: bool,
}
impl MiningState {
    pub fn reset(&mut self) {
        *self = Self::default();
    }
    pub fn target(&self) -> Option<MiningTarget> {
        self.target
    }
    pub fn fraction(&self, duration: f32) -> f32 {
        (self.elapsed / duration).clamp(0., 1.)
    }
    pub fn advance(&mut self, target: MiningTarget, seconds: f32, duration: f32) -> bool {
        if !seconds.is_finite() || seconds < 0. || !duration.is_finite() || duration <= 0. {
            self.reset();
            return false;
        }
        if self.target != Some(target) {
            self.reset();
            self.target = Some(target);
        }
        self.elapsed = (self.elapsed + seconds).min(duration);
        if self.elapsed >= duration && !self.sent {
            self.sent = true;
            return true;
        }
        false
    }
    pub fn consume(&mut self, target: MiningTarget, duration: f32) -> bool {
        if self.target == Some(target)
            && self.sent
            && duration.is_finite()
            && duration > 0.
            && self.elapsed >= duration
        {
            self.reset();
            return true;
        }
        false
    }
}
