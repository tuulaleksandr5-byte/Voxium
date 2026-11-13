//! Печка: рецепты плавки, топливо и пошаговое состояние. Не зависит от Bevy.
//!
//! Слоты контейнера печки: `INPUT`, `FUEL`, `OUTPUT`. Выходной слот только для
//! забора предметов; допустимость остальных слотов проверяет [`FurnaceState::click`].
use crate::{
    content::{ContentRegistry, ItemId},
    inventory::{ClickButton, Container, Inventory, SlotRef},
    items::ItemStack,
};
use serde::Deserialize;
use std::collections::HashSet;

pub const INPUT: usize = 0;
pub const FUEL: usize = 1;
pub const OUTPUT: usize = 2;
pub const SLOTS: usize = 3;
/// Максимальный шаг моделирования, секунд (большие `dt` делятся на шаги).
const STEP: f32 = 0.05;

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct FuelDef {
    item: String,
    burn_seconds: f32,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SmeltDef {
    input: String,
    output: Output,
    seconds: f32,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Output {
    item: String,
    count: u16,
}
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Raw {
    fuels: Vec<FuelDef>,
    recipes: Vec<SmeltDef>,
}

#[derive(Clone, Debug)]
pub struct SmeltRecipe {
    pub input: ItemId,
    pub output: ItemStack,
    pub seconds: f32,
}

#[derive(Clone, Debug)]
pub struct SmeltingBook {
    fuels: Vec<(ItemId, f32)>,
    recipes: Vec<SmeltRecipe>,
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
        let raw: Raw = ron::from_str(source).map_err(|e| e.to_string())?;
        let mut fuels = Vec::new();
        let mut seen = HashSet::new();
        for fuel in raw.fuels {
            let item = catalog.find(&fuel.item).ok_or("Неизвестное топливо")?;
            if !(fuel.burn_seconds.is_finite() && fuel.burn_seconds > 0.0) || !seen.insert(item) {
                return Err("Недопустимое или повторное топливо".into());
            }
            fuels.push((item, fuel.burn_seconds));
        }
        let mut recipes = Vec::new();
        seen.clear();
        for recipe in raw.recipes {
            let input = catalog
                .find(&recipe.input)
                .ok_or("Неизвестный вход плавки")?;
            let output = catalog
                .find(&recipe.output.item)
                .ok_or("Неизвестный результат плавки")?;
            if !(recipe.seconds.is_finite() && recipe.seconds > 0.0)
                || recipe.output.count == 0
                || recipe.output.count > catalog.max_stack(output)
                || !seen.insert(input)
            {
                return Err("Недопустимый рецепт плавки".into());
            }
            recipes.push(SmeltRecipe {
                input,
                output: ItemStack::new(output, recipe.output.count),
                seconds: recipe.seconds,
            });
        }
        Ok(Self { fuels, recipes })
    }

    #[must_use]
    pub fn recipe_for(&self, item: ItemId) -> Option<&SmeltRecipe> {
        self.recipes.iter().find(|r| r.input == item)
    }
    #[must_use]
    pub fn fuel_seconds(&self, item: ItemId) -> Option<f32> {
        self.fuels.iter().find(|f| f.0 == item).map(|f| f.1)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FurnaceState {
    /// Сколько секунд ещё горит текущая порция топлива.
    pub burn_left: f32,
    /// Длительность текущей порции (для индикатора пламени).
    pub burn_total: f32,
    /// Прогресс текущей плавки, секунд.
    pub progress: f32,
}

impl FurnaceState {
    #[must_use]
    pub fn new_slots() -> Container {
        Container::new(SLOTS)
    }

    #[must_use]
    pub fn is_burning(&self) -> bool {
        self.burn_left > 0.0
    }

    /// Доля оставшегося пламени, 0..=1.
    #[must_use]
    pub fn flame(&self) -> f32 {
        if self.burn_total > 0.0 {
            (self.burn_left / self.burn_total).clamp(0.0, 1.0)
        } else {
            0.0
        }
    }

    /// Доля прогресса стрелки, 0..=1 (для текущего входа).
    #[must_use]
    pub fn arrow(&self, container: &Container, book: &SmeltingBook) -> f32 {
        container
            .slots
            .get(INPUT)
            .and_then(Option::as_ref)
            .and_then(|stack| book.recipe_for(stack.item))
            .map_or(0.0, |r| (self.progress / r.seconds).clamp(0.0, 1.0))
    }

    /// Продвигает печку на `dt` секунд. Возвращает `true`, если слоты изменились.
    pub fn advance(
        &mut self,
        container: &mut Container,
        dt: f32,
        book: &SmeltingBook,
        catalog: &ContentRegistry,
    ) -> bool {
        if container.slots.len() != SLOTS || !dt.is_finite() || dt <= 0.0 {
            return false;
        }
        let mut changed = false;
        let mut left = dt;
        while left > 0.0 {
            let step = left.min(STEP);
            left -= step;
            changed |= self.step(container, step, book, catalog);
        }
        changed
    }

    fn step(
        &mut self,
        container: &mut Container,
        dt: f32,
        book: &SmeltingBook,
        catalog: &ContentRegistry,
    ) -> bool {
        let mut changed = false;
        let recipe = container.slots[INPUT]
            .as_ref()
            .and_then(|stack| book.recipe_for(stack.item))
            .filter(|recipe| output_fits(&container.slots[OUTPUT], &recipe.output, catalog));
        if self.burn_left <= 0.0
            && recipe.is_some()
            && let Some(stack) = &mut container.slots[FUEL]
            && let Some(seconds) = book.fuel_seconds(stack.item)
        {
            stack.count -= 1;
            if stack.count == 0 {
                container.slots[FUEL] = None;
            }
            self.burn_left = seconds;
            self.burn_total = seconds;
            changed = true;
        }
        if self.burn_left > 0.0 {
            self.burn_left = (self.burn_left - dt).max(0.0);
            if let Some(recipe) = recipe {
                self.progress += dt;
                if self.progress >= recipe.seconds {
                    self.progress = 0.0;
                    let input = container.slots[INPUT].as_mut().expect("вход проверен");
                    input.count -= 1;
                    if input.count == 0 {
                        container.slots[INPUT] = None;
                    }
                    match &mut container.slots[OUTPUT] {
                        Some(out) => out.count += recipe.output.count,
                        slot @ None => *slot = Some(recipe.output.clone()),
                    }
                    changed = true;
                }
            } else {
                self.progress = 0.0;
            }
        } else {
            self.progress = (self.progress - 2.0 * dt).max(0.0);
        }
        changed
    }

    /// Клик по слотам печки. Откатывается целиком, если нарушает правила слотов.
    pub fn click(
        inventory: &mut Inventory,
        container: &mut Container,
        slot: SlotRef,
        button: ClickButton,
        shift: bool,
        book: &SmeltingBook,
        catalog: &ContentRegistry,
    ) -> bool {
        if container.slots.len() != SLOTS {
            return false;
        }
        let mut inv = inventory.clone();
        let mut cont = container.clone();
        let before_output = cont.slots[OUTPUT].clone();
        if !inv.click(Some(&mut cont), slot, button, shift, catalog) {
            return false;
        }
        let input_ok = cont.slots[INPUT]
            .as_ref()
            .is_none_or(|s| book.recipe_for(s.item).is_some());
        let fuel_ok = cont.slots[FUEL]
            .as_ref()
            .is_none_or(|s| book.fuel_seconds(s.item).is_some());
        // Выход можно только забирать: тот же предмет, не больше прежнего.
        let output_ok = match (&before_output, &cont.slots[OUTPUT]) {
            (_, None) => true,
            (Some(a), Some(b)) => a.item == b.item && b.count <= a.count,
            (None, Some(_)) => false,
        };
        if !(input_ok && fuel_ok && output_ok) {
            return false;
        }
        *inventory = inv;
        *container = cont;
        true
    }
}

fn output_fits(slot: &Option<ItemStack>, output: &ItemStack, catalog: &ContentRegistry) -> bool {
    match slot {
        None => true,
        Some(stack) => {
            stack.item == output.item
                && u32::from(stack.count) + u32::from(output.count)
                    <= u32::from(catalog.max_stack(stack.item))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn put(c: &mut Container, slot: usize, catalog: &ContentRegistry, name: &str, n: u16) {
        c.slots[slot] = Some(ItemStack::new(catalog.find(name).unwrap(), n));
    }
    #[test]
    fn builtin_book_is_valid() {
        let _ = SmeltingBook::default();
    }
    #[test]
    fn smelts_one_item_per_recipe_time_and_consumes_fuel() {
        let catalog = ContentRegistry::default();
        let book = SmeltingBook::default();
        let mut c = FurnaceState::new_slots();
        put(&mut c, INPUT, &catalog, "survival:raw_iron", 2);
        put(&mut c, FUEL, &catalog, "survival:coal", 1);
        let mut f = FurnaceState::default();
        f.advance(&mut c, 9.0, &book, &catalog);
        assert!(c.slots[OUTPUT].is_none());
        assert!(f.is_burning());
        f.advance(&mut c, 1.5, &book, &catalog);
        let ingot = catalog.find("survival:iron_ingot").unwrap();
        assert_eq!(c.slots[OUTPUT], Some(ItemStack::new(ingot, 1)));
        assert_eq!(c.slots[INPUT].as_ref().unwrap().count, 1);
        assert!(c.slots[FUEL].is_none(), "уголь сгорел");
        // Одного угля (80 с) хватает минимум на 8 плавок: вторая идёт без нового топлива.
        f.advance(&mut c, 10.0, &book, &catalog);
        assert_eq!(c.slots[OUTPUT].as_ref().unwrap().count, 2);
        assert!(c.slots[INPUT].is_none());
    }
    #[test]
    fn no_fuel_or_full_output_blocks_smelting_without_burning_fuel() {
        let catalog = ContentRegistry::default();
        let book = SmeltingBook::default();
        let mut c = FurnaceState::new_slots();
        put(&mut c, INPUT, &catalog, "foundation:sand", 1);
        let mut f = FurnaceState::default();
        f.advance(&mut c, 30.0, &book, &catalog);
        assert!(c.slots[OUTPUT].is_none());
        put(&mut c, FUEL, &catalog, "survival:coal", 1);
        put(&mut c, OUTPUT, &catalog, "foundation:glass", 64);
        f.advance(&mut c, 30.0, &book, &catalog);
        assert_eq!(c.slots[FUEL].as_ref().unwrap().count, 1, "топливо цело");
        assert_eq!(c.slots[OUTPUT].as_ref().unwrap().count, 64);
    }
    #[test]
    fn click_rules_reject_wrong_items_and_output_placement() {
        let catalog = ContentRegistry::default();
        let book = SmeltingBook::default();
        let mut inv = Inventory::default();
        let mut c = FurnaceState::new_slots();
        inv.cursor = Some(ItemStack::new(catalog.find("foundation:dirt").unwrap(), 3));
        assert!(!FurnaceState::click(
            &mut inv,
            &mut c,
            SlotRef::Container(INPUT),
            ClickButton::Left,
            false,
            &book,
            &catalog
        ));
        assert!(!FurnaceState::click(
            &mut inv,
            &mut c,
            SlotRef::Container(FUEL),
            ClickButton::Left,
            false,
            &book,
            &catalog
        ));
        assert!(!FurnaceState::click(
            &mut inv,
            &mut c,
            SlotRef::Container(OUTPUT),
            ClickButton::Left,
            false,
            &book,
            &catalog
        ));
        assert!(inv.cursor.is_some() && c.slots.iter().all(Option::is_none));
        inv.cursor = Some(ItemStack::new(catalog.find("survival:coal").unwrap(), 3));
        assert!(FurnaceState::click(
            &mut inv,
            &mut c,
            SlotRef::Container(FUEL),
            ClickButton::Left,
            false,
            &book,
            &catalog
        ));
        assert_eq!(c.slots[FUEL].as_ref().unwrap().count, 3);
        // Забрать результат можно.
        put(&mut c, OUTPUT, &catalog, "foundation:glass", 4);
        assert!(FurnaceState::click(
            &mut inv,
            &mut c,
            SlotRef::Container(OUTPUT),
            ClickButton::Left,
            false,
            &book,
            &catalog
        ));
        assert!(c.slots[OUTPUT].is_none() && inv.cursor.is_some());
    }
}
