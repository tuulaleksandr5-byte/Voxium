//! Transactional recipes for the personal 2×2 crafting grid.
use crate::{content::ContentRegistry, inventory::Inventory, items::ItemStack};
use serde::Deserialize;
use std::collections::HashSet;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash)]
#[serde(deny_unknown_fields)]
struct CountedItem {
    item: String,
    count: u16,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Recipe {
    id: String,
    width: usize,
    height: usize,
    inputs: Vec<Option<CountedItem>>,
    output: CountedItem,
}
#[derive(Clone, Debug)]
pub struct RecipeBook {
    recipes: Vec<Recipe>,
}
impl Default for RecipeBook {
    fn default() -> Self {
        Self::from_ron(
            include_str!("../../../assets/data/recipes.ron"),
            &ContentRegistry::default(),
        )
        .expect("встроенные рецепты проверены тестами")
    }
}
impl RecipeBook {
    /// Validate before replacing a live book; item names survive registry reloads.
    pub fn from_ron(source: &str, catalog: &ContentRegistry) -> Result<Self, String> {
        let recipes: Vec<Recipe> = ron::Options::default()
            .with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME)
            .from_str(source)
            .map_err(|error| error.to_string())?;
        let mut ids = HashSet::new();
        let mut patterns = HashSet::new();
        for recipe in &recipes {
            if !valid_id(&recipe.id) || !ids.insert(&recipe.id) {
                return Err("Недопустимый или повторный ID рецепта".into());
            }
            if !(1..=3).contains(&recipe.width)
                || !(1..=3).contains(&recipe.height)
                || recipe.inputs.len() != recipe.width * recipe.height
            {
                return Err("Рецепт должен помещаться в сетку 3×3".into());
            }
            // Require a tight bounding rectangle, so every translation is unambiguous.
            let occupied = |x: usize, y: usize| recipe.inputs[y * recipe.width + x].is_some();
            if !(0..recipe.width).any(|x| occupied(x, 0))
                || !(0..recipe.width).any(|x| occupied(x, recipe.height - 1))
                || !(0..recipe.height).any(|y| occupied(0, y))
                || !(0..recipe.height).any(|y| occupied(recipe.width - 1, y))
            {
                return Err("Пустые края рецепта недопустимы".into());
            }
            for ingredient in recipe
                .inputs
                .iter()
                .flatten()
                .chain(std::iter::once(&recipe.output))
            {
                let item = catalog
                    .find(&ingredient.item)
                    .ok_or("Неизвестный предмет рецепта")?;
                if ingredient.count == 0 || ingredient.count > catalog.max_stack(item) {
                    return Err("Недопустимое количество в рецепте".into());
                }
            }
            if !patterns.insert((recipe.width, recipe.height, &recipe.inputs)) {
                return Err("Повторяющийся рисунок рецепта".into());
            }
        }
        Ok(Self { recipes })
    }

    pub fn preview(
        &self,
        grid: &[Option<ItemStack>; 4],
        catalog: &ContentRegistry,
    ) -> Option<ItemStack> {
        self.matching(grid, 2, catalog).map(|(_, _, output)| output)
    }

    /// Результат верстака 3×3; рецепты 2×2 тоже работают на верстаке.
    pub fn preview_workbench(
        &self,
        grid: &[Option<ItemStack>],
        catalog: &ContentRegistry,
    ) -> Option<ItemStack> {
        if grid.len() != 9 {
            return None;
        }
        self.matching(grid, 3, catalog).map(|(_, _, output)| output)
    }

    /// A result is never stored: transfer a whole portion before consuming its inputs.
    pub fn take_result(
        &self,
        inventory: &mut Inventory,
        all: bool,
        catalog: &ContentRegistry,
    ) -> bool {
        let mut grid = inventory.crafting.to_vec();
        let changed = self.take_from_grid(inventory, &mut grid, 2, all, catalog);
        if changed {
            for (slot, value) in inventory.crafting.iter_mut().zip(grid) {
                *slot = value;
            }
        }
        changed
    }

    /// То же для сетки 3×3 верстака (первые 9 слотов контейнера).
    pub fn take_workbench_result(
        &self,
        inventory: &mut Inventory,
        grid: &mut [Option<ItemStack>],
        all: bool,
        catalog: &ContentRegistry,
    ) -> bool {
        grid.len() == 9 && self.take_from_grid(inventory, grid, 3, all, catalog)
    }

    fn take_from_grid(
        &self,
        inventory: &mut Inventory,
        grid: &mut [Option<ItemStack>],
        side: usize,
        all: bool,
        catalog: &ContentRegistry,
    ) -> bool {
        if all && inventory.cursor.is_some() {
            return false;
        }
        let target_recipe = self
            .matching(grid, side, catalog)
            .map(|(recipe, _, _)| recipe.id.as_str());
        let mut changed = false;
        while let Some((recipe, costs, output)) = self.matching(grid, side, catalog) {
            if Some(recipe.id.as_str()) != target_recipe {
                break;
            }
            let mut candidate = inventory.clone();
            if all {
                if candidate.insert(output, catalog).is_some() {
                    break;
                }
            } else if let Some(cursor) = &mut candidate.cursor {
                if cursor.item != output.item || cursor.count == 0 {
                    break;
                }
                let Some(count) = cursor
                    .count
                    .checked_add(output.count)
                    .filter(|count| *count <= catalog.max_stack(cursor.item))
                else {
                    break;
                };
                cursor.count = count;
            } else {
                candidate.cursor = Some(output);
            }
            *inventory = candidate;
            for (slot, cost) in grid.iter_mut().zip(costs) {
                if cost > 0
                    && let Some(stack) = slot
                {
                    stack.count -= cost;
                    if stack.count == 0 {
                        *slot = None;
                    }
                }
            }
            changed = true;
            if !all {
                break;
            }
        }
        changed
    }

    /// Used before committing an item catalog reload.
    pub fn validate_catalog(&self, catalog: &ContentRegistry) -> Result<(), String> {
        for recipe in &self.recipes {
            for ingredient in recipe
                .inputs
                .iter()
                .flatten()
                .chain(std::iter::once(&recipe.output))
            {
                let item = catalog
                    .find(&ingredient.item)
                    .ok_or("Неизвестный предмет рецепта")?;
                if ingredient.count > catalog.max_stack(item) {
                    return Err("Новый размер стопки несовместим с рецептами".into());
                }
            }
        }
        Ok(())
    }

    fn matching(
        &self,
        grid: &[Option<ItemStack>],
        side: usize,
        catalog: &ContentRegistry,
    ) -> Option<(&Recipe, Vec<u16>, ItemStack)> {
        if grid
            .iter()
            .flatten()
            .any(|stack| stack.count == 0 || stack.count > catalog.max_stack(stack.item))
        {
            return None;
        }
        for recipe in &self.recipes {
            if recipe.width > side || recipe.height > side {
                continue;
            }
            let Some(output_item) = catalog.find(&recipe.output.item) else {
                continue;
            };
            if recipe.output.count > catalog.max_stack(output_item) {
                continue;
            }
            for y in 0..=side - recipe.height {
                for x in 0..=side - recipe.width {
                    let mut costs = vec![0; side * side];
                    let matches = (0..side * side).all(|index| {
                        let gx = index % side;
                        let gy = index / side;
                        let ingredient = if gx >= x
                            && gx < x + recipe.width
                            && gy >= y
                            && gy < y + recipe.height
                        {
                            recipe.inputs[(gy - y) * recipe.width + gx - x].as_ref()
                        } else {
                            None
                        };
                        match (ingredient, &grid[index]) {
                            (None, None) => true,
                            (Some(input), Some(stack))
                                if catalog.find(&input.item) == Some(stack.item)
                                    && stack.count >= input.count =>
                            {
                                costs[index] = input.count;
                                true
                            }
                            _ => false,
                        }
                    });
                    if matches {
                        return Some((
                            recipe,
                            costs,
                            ItemStack::new(output_item, recipe.output.count),
                        ));
                    }
                }
            }
        }
        None
    }
}

fn valid_id(id: &str) -> bool {
    id.split_once(':').is_some_and(|(namespace, name)| {
        !namespace.is_empty()
            && !name.is_empty()
            && namespace.bytes().all(|byte| {
                byte.is_ascii_lowercase()
                    || byte.is_ascii_digit()
                    || matches!(byte, b'_' | b'-' | b'.')
            })
            && name.bytes().all(|byte| {
                byte.is_ascii_lowercase()
                    || byte.is_ascii_digit()
                    || matches!(byte, b'_' | b'-' | b'.' | b'/')
            })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn stack(catalog: &ContentRegistry, name: &str, count: u16) -> Option<ItemStack> {
        Some(ItemStack::new(catalog.find(name).unwrap(), count))
    }
    #[test]
    fn shift_stops_before_a_different_recipe_appears_in_the_remaining_grid() {
        let catalog = ContentRegistry::default();
        let book = RecipeBook::default();
        let mut inventory = Inventory {
            crafting: [
                stack(&catalog, "foundation:oak_planks", 2),
                stack(&catalog, "foundation:oak_planks", 1),
                stack(&catalog, "foundation:oak_planks", 2),
                stack(&catalog, "foundation:oak_planks", 1),
            ],
            ..Default::default()
        };
        assert!(book.take_result(&mut inventory, true, &catalog));
        assert_eq!(
            inventory.hotbar[0],
            stack(&catalog, "survival:crafting_table", 1)
        );
        assert!(inventory.hotbar[1].is_none());
        assert_eq!(
            inventory.crafting,
            [
                stack(&catalog, "foundation:oak_planks", 1),
                None,
                stack(&catalog, "foundation:oak_planks", 1),
                None
            ]
        );
    }
    #[test]
    fn single_ingredient_recipes_match_any_cell_but_reject_extra_input() {
        let catalog = ContentRegistry::default();
        let book = RecipeBook::default();
        for (input, output, count) in [
            ("foundation:oak_log", "foundation:oak_planks", 4),
            ("survival:iron_block", "survival:iron_ingot", 9),
            ("survival:gold_block", "survival:gold_ingot", 9),
            ("survival:diamond_block", "survival:diamond", 9),
        ] {
            for position in 0..4 {
                let mut grid = [None, None, None, None];
                grid[position] = stack(&catalog, input, 2);
                assert_eq!(
                    book.preview(&grid, &catalog),
                    stack(&catalog, output, count)
                );
                grid[(position + 1) % 4] = stack(&catalog, "foundation:dirt", 1);
                assert_eq!(book.preview(&grid, &catalog), None);
            }
        }
    }
    #[test]
    fn result_transfer_rechecks_input_and_rejects_incompatible_or_overfull_cursor_atomically() {
        let catalog = ContentRegistry::default();
        let book = RecipeBook::default();
        let mut inventory = Inventory::default();
        inventory.crafting[3] = stack(&catalog, "foundation:oak_log", 2);
        for cursor in [
            stack(&catalog, "foundation:dirt", 1),
            stack(&catalog, "foundation:oak_planks", 61),
            stack(&catalog, "foundation:oak_planks", u16::MAX),
        ] {
            inventory.cursor = cursor.clone();
            assert!(!book.take_result(&mut inventory, false, &catalog));
            assert_eq!(inventory.cursor, cursor);
            assert_eq!(
                inventory.crafting[3],
                stack(&catalog, "foundation:oak_log", 2)
            );
        }
        inventory.cursor = stack(&catalog, "foundation:oak_planks", 60);
        assert!(book.take_result(&mut inventory, false, &catalog));
        assert_eq!(
            inventory.cursor,
            stack(&catalog, "foundation:oak_planks", 64)
        );
        assert_eq!(
            inventory.crafting[3],
            stack(&catalog, "foundation:oak_log", 1)
        );
        inventory.cursor = None;
        assert!(book.take_result(&mut inventory, false, &catalog));
        assert_eq!(
            inventory.cursor,
            stack(&catalog, "foundation:oak_planks", 4)
        );
        assert!(!book.take_result(&mut inventory, false, &catalog));
        assert!(inventory.crafting.iter().all(Option::is_none));
    }
    #[test]
    fn shift_crafts_maximum_whole_portions_across_stacks_and_keeps_remainder() {
        let catalog = ContentRegistry::default();
        let book = RecipeBook::default();
        let mut inventory = Inventory::default();
        inventory
            .hotbar
            .fill(stack(&catalog, "foundation:dirt", 64));
        inventory.main.fill(stack(&catalog, "foundation:dirt", 64));
        inventory.hotbar[0] = stack(&catalog, "survival:diamond", 62);
        inventory.main[0] = None;
        inventory.crafting[0] = stack(&catalog, "survival:diamond_block", 10);
        inventory.cursor = stack(&catalog, "foundation:dirt", 1);
        assert!(!book.take_result(&mut inventory, true, &catalog));
        inventory.cursor = None;
        assert!(book.take_result(&mut inventory, true, &catalog));
        assert_eq!(inventory.hotbar[0], stack(&catalog, "survival:diamond", 64));
        assert_eq!(inventory.main[0], stack(&catalog, "survival:diamond", 61));
        assert_eq!(
            inventory.crafting[0],
            stack(&catalog, "survival:diamond_block", 3)
        );
        assert!(!book.take_result(&mut inventory, true, &catalog));
        assert_eq!(inventory.main[0], stack(&catalog, "survival:diamond", 61));
    }
    #[test]
    fn counted_inputs_are_consumed_exactly_and_invalid_recipes_return_errors() {
        let catalog = ContentRegistry::default();
        let source = r#"[(id:"test:counted",width:1,height:1,inputs:[(item:"foundation:oak_log",count:2)],output:(item:"foundation:oak_planks",count:3))]"#;
        let book = RecipeBook::from_ron(source, &catalog).unwrap();
        let mut inventory = Inventory::default();
        inventory.crafting[1] = stack(&catalog, "foundation:oak_log", 5);
        assert!(book.take_result(&mut inventory, true, &catalog));
        assert_eq!(
            inventory.hotbar[0],
            stack(&catalog, "foundation:oak_planks", 6)
        );
        assert_eq!(
            inventory.crafting[1],
            stack(&catalog, "foundation:oak_log", 1)
        );
        for invalid in [
            source.replace("width:1", "width:3"),
            source.replace("height:1", "height:0"),
            source.replace("count:2", "count:0"),
            source.replace("count:3", "count:65"),
            source.replace("foundation:oak_log", "test:missing"),
            source.replace("id:", "typo:"),
            source.replace("test:counted", "invalid"),
            source.replace("inputs:[", "inputs:[None,"),
            source.replace("count:2", "count:65536"),
        ] {
            assert!(
                RecipeBook::from_ron(&invalid, &catalog).is_err(),
                "{invalid}"
            );
        }
        let duplicate = format!(
            "[{},{}]",
            &source[1..source.len() - 1],
            &source[1..source.len() - 1]
        );
        assert!(RecipeBook::from_ron(&duplicate, &catalog).is_err());
        assert!(
            RecipeBook::from_ron(
                &duplicate.replacen("test:counted", "test:other", 1),
                &catalog
            )
            .is_err()
        );
        assert!(RecipeBook::from_ron("[(", &catalog).is_err());
        assert!(RecipeBook::from_ron("[]", &catalog).is_ok());
    }
    #[test]
    fn shaped_recipes_require_vertical_sticks_or_complete_square() {
        let catalog = ContentRegistry::default();
        let book = RecipeBook::default();
        for column in 0..2 {
            let mut grid = [None, None, None, None];
            grid[column] = stack(&catalog, "foundation:oak_planks", 2);
            grid[column + 2] = stack(&catalog, "foundation:oak_planks", 3);
            assert_eq!(
                book.preview(&grid, &catalog),
                stack(&catalog, "survival:stick", 4)
            );
        }
        let plank = stack(&catalog, "foundation:oak_planks", 1);
        for grid in [
            [plank.clone(), plank.clone(), None, None],
            [plank.clone(), None, None, plank],
        ] {
            assert_eq!(book.preview(&grid, &catalog), None);
        }
        for (input, output) in [
            ("foundation:oak_planks", "survival:crafting_table"),
            ("foundation:sand", "foundation:sandstone"),
        ] {
            let mut grid = std::array::from_fn(|_| stack(&catalog, input, 1));
            assert_eq!(book.preview(&grid, &catalog), stack(&catalog, output, 1));
            grid[3] = None;
            assert_eq!(book.preview(&grid, &catalog), None);
        }
    }
}

#[cfg(test)]
mod workbench_tests {
    use super::*;
    fn s(c: &ContentRegistry, n: &str, k: u16) -> Option<ItemStack> {
        Some(ItemStack::new(c.find(n).unwrap(), k))
    }
    #[test]
    fn pickaxe_needs_the_workbench_and_consumes_the_grid() {
        let c = ContentRegistry::default();
        let book = RecipeBook::default();
        let p = "foundation:oak_planks";
        let st = "survival:stick";
        let mut grid = vec![
            s(&c, p, 2),
            s(&c, p, 1),
            s(&c, p, 1),
            None,
            s(&c, st, 3),
            None,
            None,
            s(&c, st, 1),
            None,
        ];
        assert_eq!(
            book.preview_workbench(&grid, &c),
            s(&c, "survival:wooden_pickaxe", 1)
        );
        // В личной сетке 2×2 такой рецепт недоступен.
        assert!(book.preview(&Default::default(), &c).is_none());
        let mut inv = Inventory::default();
        assert!(book.take_workbench_result(&mut inv, &mut grid, false, &c));
        assert_eq!(inv.cursor, s(&c, "survival:wooden_pickaxe", 1));
        assert_eq!(grid[0], s(&c, p, 1));
        assert!(grid[1].is_none() && grid[2].is_none());
        assert_eq!(grid[4], s(&c, st, 2));
        assert!(grid[7].is_none());
    }
    #[test]
    fn furnace_recipe_is_a_ring_of_cobblestone() {
        let c = ContentRegistry::default();
        let book = RecipeBook::default();
        let cb = "foundation:cobblestone";
        let mut grid: Vec<_> = (0..9).map(|_| s(&c, cb, 1)).collect();
        assert!(book.preview_workbench(&grid, &c).is_none());
        grid[4] = None;
        assert_eq!(
            book.preview_workbench(&grid, &c),
            s(&c, "survival:furnace", 1)
        );
    }
}
