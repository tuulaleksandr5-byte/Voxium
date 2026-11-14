//! Печь: три ячейки контейнера (сырьё, топливо, результат) и таймеры горения.
//!
//! Логика не зависит от Bevy. Клиент хранит ячейки в обычном `Container`,
//! поэтому клики, Shift-перенос и высыпание при разрушении уже работают.

use crate::{
    content::{ContentRegistry, ItemId},
    inventory::Container,
    items::ItemStack,
};
use serde::Deserialize;
use std::collections::HashMap;

/// Вид печи: коптильня берёт только еду, плавильня — только руду; обе вдвое быстрее.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FurnaceKind {
    Furnace,
    Smoker,
    Blast,
}
impl FurnaceKind {
    /// Вид печи и признак горения по ID блока.
    #[must_use]
    pub fn of_block(block: u8) -> Option<(Self, bool)> {
        Some(match block {
            crate::FURNACE => (Self::Furnace, false),
            crate::FURNACE_LIT => (Self::Furnace, true),
            crate::SMOKER => (Self::Smoker, false),
            crate::SMOKER_LIT => (Self::Smoker, true),
            crate::BLAST_FURNACE => (Self::Blast, false),
            crate::BLAST_FURNACE_LIT => (Self::Blast, true),
            _ => return None,
        })
    }
    /// ID блока этого вида в выбранном состоянии.
    #[must_use]
    pub fn block(self, lit: bool) -> u8 {
        match (self, lit) {
            (Self::Furnace, false) => crate::FURNACE,
            (Self::Furnace, true) => crate::FURNACE_LIT,
            (Self::Smoker, false) => crate::SMOKER,
            (Self::Smoker, true) => crate::SMOKER_LIT,
            (Self::Blast, false) => crate::BLAST_FURNACE,
            (Self::Blast, true) => crate::BLAST_FURNACE_LIT,
        }
    }
    fn speed(self) -> f32 {
        if self == Self::Furnace { 1.0 } else { 2.0 }
    }
    fn accepts(self, input: &str, catalog: &ContentRegistry) -> bool {
        match self {
            Self::Furnace => true,
            Self::Smoker => catalog
                .find(input)
                .and_then(|id| catalog.get(id))
                .is_some_and(|item| item.kind == crate::content::ItemKind::Food),
            Self::Blast => {
                input.contains("raw_iron") || input.contains("raw_gold") || input.ends_with("_ore")
            }
        }
    }
}

pub const FURNACE_INPUT: usize = 0;
pub const FURNACE_FUEL: usize = 1;
pub const FURNACE_OUTPUT: usize = 2;
pub const FURNACE_SLOTS: usize = 3;

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SmeltingRecipe {
    input: String,
    output: String,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Fuel {
    item: String,
    seconds: f32,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SmeltingSource {
    seconds_per_item: f32,
    recipes: Vec<SmeltingRecipe>,
    fuels: Vec<Fuel>,
}

#[derive(Clone, Debug)]
pub struct SmeltingBook {
    seconds_per_item: f32,
    recipes: HashMap<ItemId, ItemId>,
    fuels: HashMap<ItemId, f32>,
}
impl Default for SmeltingBook {
    fn default() -> Self {
        Self::from_ron(
            include_str!("../../../assets/data/smelting.ron"),
            &ContentRegistry::default(),
        )
        .expect("встроенные рецепты плавки проверены тестами")
    }
}
impl SmeltingBook {
    pub fn from_ron(source: &str, catalog: &ContentRegistry) -> Result<Self, String> {
        let source: SmeltingSource = ron::from_str(source).map_err(|e| e.to_string())?;
        if !source.seconds_per_item.is_finite() || source.seconds_per_item <= 0.0 {
            return Err("Время плавки должно быть положительным".into());
        }
        let find = |id: &str| {
            catalog
                .find(id)
                .ok_or_else(|| format!("Неизвестный предмет плавки: {id}"))
        };
        let mut recipes = HashMap::new();
        for recipe in &source.recipes {
            if recipes
                .insert(find(&recipe.input)?, find(&recipe.output)?)
                .is_some()
            {
                return Err(format!("Повторный рецепт плавки: {}", recipe.input));
            }
        }
        let mut fuels = HashMap::new();
        for fuel in &source.fuels {
            if !fuel.seconds.is_finite()
                || fuel.seconds <= 0.0
                || fuels.insert(find(&fuel.item)?, fuel.seconds).is_some()
            {
                return Err(format!("Некорректное топливо: {}", fuel.item));
            }
        }
        Ok(Self {
            seconds_per_item: source.seconds_per_item,
            recipes,
            fuels,
        })
    }
    #[must_use]
    pub fn result(&self, input: ItemId) -> Option<ItemId> {
        self.recipes.get(&input).copied()
    }
    /// Результат с учётом того, что принимает этот вид печи.
    #[must_use]
    pub fn result_for(
        &self,
        kind: FurnaceKind,
        input: ItemId,
        catalog: &ContentRegistry,
    ) -> Option<ItemId> {
        let id = &catalog.get(input)?.id;
        kind.accepts(id, catalog)
            .then(|| self.result(input))
            .flatten()
    }
    #[must_use]
    pub fn fuel_seconds(&self, item: ItemId) -> Option<f32> {
        self.fuels.get(&item).copied()
    }
    #[must_use]
    pub fn seconds_per_item(&self) -> f32 {
        self.seconds_per_item
    }
}

/// Таймеры одной печи. Содержимое лежит в контейнере из трёх ячеек.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FurnaceState {
    pub burn_left: f32,
    pub burn_total: f32,
    pub progress: f32,
}
impl FurnaceState {
    #[must_use]
    pub fn lit(&self) -> bool {
        self.burn_left > 0.0
    }
    /// Доля оставшегося топлива, 0…1.
    #[must_use]
    pub fn fuel_fraction(&self) -> f32 {
        if self.burn_total > 0.0 {
            (self.burn_left / self.burn_total).clamp(0.0, 1.0)
        } else {
            0.0
        }
    }
    /// Доля готовности текущего предмета, 0…1.
    #[must_use]
    pub fn progress_fraction(&self, book: &SmeltingBook) -> f32 {
        (self.progress / book.seconds_per_item).clamp(0.0, 1.0)
    }

    /// Продвигает печь на `dt` секунд. Возвращает `true`, если изменились ячейки.
    /// Топливо поджигается только когда есть что плавить и куда класть результат.
    pub fn tick(
        &mut self,
        container: &mut Container,
        dt: f32,
        book: &SmeltingBook,
        catalog: &ContentRegistry,
    ) -> bool {
        self.tick_kind(FurnaceKind::Furnace, container, dt, book, catalog)
    }

    /// То же для любого вида печи: коптильня и плавильня работают вдвое быстрее
    /// и вдвое быстрее жгут топливо.
    pub fn tick_kind(
        &mut self,
        kind: FurnaceKind,
        container: &mut Container,
        dt: f32,
        book: &SmeltingBook,
        catalog: &ContentRegistry,
    ) -> bool {
        if container.slots.len() < FURNACE_SLOTS || !dt.is_finite() || dt <= 0.0 {
            return false;
        }
        let mut changed = false;
        let mut remaining = dt.min(60.0) * kind.speed();
        // Длинный кадр обрабатывается шагами, чтобы не терять предметы и топливо.
        while remaining > 0.0 {
            let output = Self::smeltable(kind, container, book, catalog);
            if self.burn_left <= 0.0 {
                let fuel = container.slots[FURNACE_FUEL]
                    .as_ref()
                    .and_then(|stack| book.fuel_seconds(stack.item));
                match (output, fuel) {
                    (Some(_), Some(seconds)) => {
                        take_one(&mut container.slots[FURNACE_FUEL]);
                        self.burn_left = seconds;
                        self.burn_total = seconds;
                        changed = true;
                    }
                    _ => {
                        self.progress = (self.progress - remaining * 2.0).max(0.0);
                        break;
                    }
                }
            }
            let step = remaining.min(self.burn_left);
            let Some(output) = output else {
                self.burn_left -= step;
                self.progress = 0.0;
                remaining -= step;
                continue;
            };
            let step = step.min(book.seconds_per_item - self.progress).max(1e-4);
            self.burn_left = (self.burn_left - step).max(0.0);
            self.progress += step;
            remaining -= step;
            if self.progress >= book.seconds_per_item - 1e-4 {
                self.progress = 0.0;
                take_one(&mut container.slots[FURNACE_INPUT]);
                match &mut container.slots[FURNACE_OUTPUT] {
                    Some(stack) => stack.count += 1,
                    slot => *slot = Some(ItemStack::new(output, 1)),
                }
                changed = true;
            }
        }
        changed
    }

    fn smeltable(
        kind: FurnaceKind,
        container: &Container,
        book: &SmeltingBook,
        catalog: &ContentRegistry,
    ) -> Option<ItemId> {
        let input = container.slots[FURNACE_INPUT].as_ref()?;
        let output = book.result_for(kind, input.item, catalog)?;
        match &container.slots[FURNACE_OUTPUT] {
            None => Some(output),
            Some(stack) if stack.item == output && stack.count < catalog.max_stack(output) => {
                Some(output)
            }
            _ => None,
        }
    }
}

fn take_one(slot: &mut Option<ItemStack>) {
    if let Some(stack) = slot {
        stack.count -= 1;
        if stack.count == 0 {
            *slot = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stack(catalog: &ContentRegistry, id: &str, count: u16) -> Option<ItemStack> {
        Some(ItemStack::new(catalog.find(id).unwrap(), count))
    }

    #[test]
    fn furnace_burns_fuel_only_for_valid_input_and_produces_exact_counts() {
        let catalog = ContentRegistry::default();
        let book = SmeltingBook::default();
        let mut furnace = Container::new(FURNACE_SLOTS);
        let mut state = FurnaceState::default();
        furnace.slots[FURNACE_FUEL] = stack(&catalog, "survival:coal", 2);
        furnace.slots[FURNACE_INPUT] = stack(&catalog, "foundation:dirt", 3);
        assert!(!state.tick(&mut furnace, 5.0, &book, &catalog));
        assert!(!state.lit());
        assert_eq!(
            furnace.slots[FURNACE_FUEL],
            stack(&catalog, "survival:coal", 2)
        );

        furnace.slots[FURNACE_INPUT] = stack(&catalog, "survival:raw_iron", 3);
        assert!(state.tick(&mut furnace, 1.0, &book, &catalog));
        assert!(state.lit());
        assert_eq!(
            furnace.slots[FURNACE_FUEL],
            stack(&catalog, "survival:coal", 1)
        );
        assert!(furnace.slots[FURNACE_OUTPUT].is_none());
        for _ in 0..70 {
            state.tick(&mut furnace, 0.1, &book, &catalog);
        }
        assert_eq!(
            furnace.slots[FURNACE_OUTPUT],
            stack(&catalog, "survival:iron_ingot", 1)
        );
        // Один большой шаг не теряет предметы: уголь горит 64 с, плавка 8 с.
        state.tick(&mut furnace, 60.0, &book, &catalog);
        assert_eq!(
            furnace.slots[FURNACE_OUTPUT],
            stack(&catalog, "survival:iron_ingot", 3)
        );
        assert!(furnace.slots[FURNACE_INPUT].is_none());
        assert_eq!(
            furnace.slots[FURNACE_FUEL],
            stack(&catalog, "survival:coal", 1)
        );
    }

    #[test]
    fn blocked_output_pauses_smelting_without_consuming_new_fuel() {
        let catalog = ContentRegistry::default();
        let book = SmeltingBook::default();
        let mut furnace = Container::new(FURNACE_SLOTS);
        let mut state = FurnaceState::default();
        furnace.slots[FURNACE_INPUT] = stack(&catalog, "foundation:sand", 4);
        furnace.slots[FURNACE_FUEL] = stack(&catalog, "survival:stick", 8);
        furnace.slots[FURNACE_OUTPUT] = stack(&catalog, "foundation:dirt", 1);
        assert!(!state.tick(&mut furnace, 30.0, &book, &catalog));
        assert_eq!(
            furnace.slots[FURNACE_FUEL],
            stack(&catalog, "survival:stick", 8)
        );
        furnace.slots[FURNACE_OUTPUT] = None;
        state.tick(&mut furnace, 17.0, &book, &catalog);
        assert_eq!(
            furnace.slots[FURNACE_OUTPUT],
            stack(&catalog, "foundation:glass", 2)
        );
        assert!(state.progress_fraction(&book) <= 1.0 && state.fuel_fraction() <= 1.0);
    }

    #[test]
    fn invalid_books_are_rejected() {
        let catalog = ContentRegistry::default();
        let source = include_str!("../../../assets/data/smelting.ron");
        assert!(SmeltingBook::from_ron(source, &catalog).is_ok());
        for invalid in [
            source.replace("seconds_per_item: 8.0", "seconds_per_item: 0.0"),
            source.replace("survival:raw_iron", "survival:missing"),
            source.replace("seconds: 64.0", "seconds: -1.0"),
            source.replace(
                "(input: \"foundation:sand\"",
                "(input: \"survival:raw_gold\"",
            ),
            "(".into(),
        ] {
            assert!(SmeltingBook::from_ron(&invalid, &catalog).is_err());
        }
    }

    #[test]
    fn smoker_and_blast_furnace_filter_inputs_and_work_twice_as_fast() {
        let catalog = ContentRegistry::default();
        let book = SmeltingBook::default();
        for (kind, good, bad, output) in [
            (
                FurnaceKind::Smoker,
                "builder:raw_beef",
                "survival:raw_iron",
                "builder:cooked_beef",
            ),
            (
                FurnaceKind::Blast,
                "survival:raw_iron",
                "builder:raw_beef",
                "survival:iron_ingot",
            ),
        ] {
            let mut furnace = Container::new(FURNACE_SLOTS);
            let mut state = FurnaceState::default();
            furnace.slots[FURNACE_FUEL] = stack(&catalog, "survival:coal", 1);
            furnace.slots[FURNACE_INPUT] = stack(&catalog, bad, 2);
            assert!(!state.tick_kind(kind, &mut furnace, 10.0, &book, &catalog));
            assert!(!state.lit());
            furnace.slots[FURNACE_INPUT] = stack(&catalog, good, 2);
            // 4.1 с реального времени = 8.2 с плавки: ровно один предмет.
            state.tick_kind(kind, &mut furnace, 4.1, &book, &catalog);
            assert_eq!(furnace.slots[FURNACE_OUTPUT], stack(&catalog, output, 1));
            assert_eq!(FurnaceKind::of_block(kind.block(true)), Some((kind, true)));
            assert_eq!(crate::unlit(kind.block(true)), kind.block(false));
        }
        assert_eq!(FurnaceKind::of_block(crate::STONE), None);
    }
}
