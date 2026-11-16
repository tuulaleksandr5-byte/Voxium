//! Поиск рецептов по предмету: «как получить» и «куда идёт» (как обзор рецептов в играх с модами).
use crate::{
    content::{ContentRegistry, ItemId},
    crafting::{RecipeBook, RecipeView},
    smelting::SmeltingBook,
    stonecutting,
};

/// Один способ получить предмет или применить его.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RecipeCard {
    /// Верстак или сетка 2×2.
    Crafting(RecipeView),
    /// Печь: что кладём и что получаем.
    Smelting { input: ItemId, output: ItemId },
    /// Камнерез.
    Stonecutting { input: ItemId, output: ItemId },
    /// Предмет горит в печи указанное число секунд.
    Fuel { item: ItemId, seconds: u32 },
}

/// Рецепты, из которых предмет получается (`uses == false`) или в которых он нужен (`true`).
#[must_use]
pub fn find(
    item: ItemId,
    uses: bool,
    recipes: &RecipeBook,
    smelting: &SmeltingBook,
    catalog: &ContentRegistry,
) -> Vec<RecipeCard> {
    let mut cards = Vec::new();
    for view in recipes.views(catalog) {
        let matches = if uses {
            view.inputs.iter().flatten().any(|(id, _)| *id == item)
        } else {
            view.output.0 == item
        };
        if matches {
            cards.push(RecipeCard::Crafting(view));
        }
    }
    for (input, output) in smelting.pairs() {
        if (uses && input == item) || (!uses && output == item) {
            cards.push(RecipeCard::Smelting { input, output });
        }
    }
    if uses {
        for output in stonecutting::options(item, catalog) {
            cards.push(RecipeCard::Stonecutting {
                input: item,
                output,
            });
        }
        if let Some(seconds) = smelting.fuel_seconds(item) {
            cards.push(RecipeCard::Fuel {
                item,
                seconds: seconds.round() as u32,
            });
        }
    } else {
        for (input, _) in catalog.iter() {
            if stonecutting::options(input, catalog).contains(&item) {
                cards.push(RecipeCard::Stonecutting {
                    input,
                    output: item,
                });
            }
        }
    }
    cards
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn planks_come_from_logs_and_logs_are_used_for_planks_and_fuel() {
        let catalog = ContentRegistry::default();
        let recipes = RecipeBook::default();
        let smelting = SmeltingBook::default();
        let planks = catalog.find("foundation:oak_planks").unwrap();
        let log = catalog.find("foundation:oak_log").unwrap();
        let made = find(planks, false, &recipes, &smelting, &catalog);
        assert!(made.iter().any(|card| matches!(
            card,
            RecipeCard::Crafting(view) if view.output == (planks, 4)
        )));
        let used = find(log, true, &recipes, &smelting, &catalog);
        assert!(
            used.iter()
                .any(|card| matches!(card, RecipeCard::Crafting(_)))
        );
    }

    #[test]
    fn smelting_and_stonecutting_are_listed_both_ways() {
        let catalog = ContentRegistry::default();
        let recipes = RecipeBook::default();
        let smelting = SmeltingBook::default();
        let (input, output) = smelting.pairs()[0];
        assert!(
            find(output, false, &recipes, &smelting, &catalog)
                .contains(&RecipeCard::Smelting { input, output })
        );
        assert!(
            find(input, true, &recipes, &smelting, &catalog)
                .contains(&RecipeCard::Smelting { input, output })
        );
        let sand = catalog.find("foundation:sand").unwrap();
        let gravel = catalog.find("foundation:gravel").unwrap();
        let cut = RecipeCard::Stonecutting {
            input: gravel,
            output: sand,
        };
        assert!(find(gravel, true, &recipes, &smelting, &catalog).contains(&cut));
        assert!(find(sand, false, &recipes, &smelting, &catalog).contains(&cut));
    }

    #[test]
    fn unknown_demand_gives_nothing() {
        let catalog = ContentRegistry::default();
        let bedrock = catalog.find("foundation:bedrock");
        if let Some(item) = bedrock {
            assert!(
                find(
                    item,
                    false,
                    &RecipeBook::default(),
                    &SmeltingBook::default(),
                    &catalog
                )
                .is_empty()
            );
        }
    }
}
