//! Питательность еды и урон оружия: простые таблицы по стабильным ID предметов.

/// (голод, насыщение) для съедобного предмета.
#[must_use]
pub fn food_value(item: &str) -> Option<(f32, f32)> {
    Some(match item {
        "builder:raw_beef" | "builder:raw_pork" => (3.0, 1.8),
        "builder:cooked_beef" | "builder:cooked_pork" => (8.0, 12.8),
        "builder:raw_mutton" | "builder:raw_chicken" => (2.0, 1.2),
        "builder:cooked_mutton" | "builder:cooked_chicken" => (6.0, 7.2),
        "builder:rotten_meat" => (4.0, 0.8),
        _ => return None,
    })
}

/// Урон удара по мобу предметом в руке; пустая рука и блоки бьют на 1.
#[must_use]
pub fn attack_damage(item: Option<&str>) -> f32 {
    let Some((material, tool)) = item
        .and_then(|id| id.strip_prefix("survival:"))
        .and_then(|name| name.split_once('_'))
    else {
        return 1.0;
    };
    let tier = match material {
        "wooden" | "golden" => 0.0,
        "stone" => 1.0,
        "iron" => 2.0,
        "diamond" => 3.0,
        _ => return 1.0,
    };
    match tool {
        "sword" => 4.0 + tier,
        "axe" => 3.0 + tier,
        "pickaxe" => 2.0 + tier,
        "shovel" | "hoe" => 1.5 + tier * 0.5,
        _ => 1.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::{ContentRegistry, ItemKind};

    #[test]
    fn every_food_item_has_a_value_and_only_food_does() {
        let catalog = ContentRegistry::default();
        for (_, item) in catalog.iter() {
            assert_eq!(
                food_value(&item.id).is_some(),
                item.kind == ItemKind::Food,
                "{}",
                item.id
            );
        }
    }

    #[test]
    fn swords_outrank_other_tools_and_hands() {
        assert_eq!(attack_damage(None), 1.0);
        assert_eq!(attack_damage(Some("foundation:dirt")), 1.0);
        assert_eq!(attack_damage(Some("survival:leather_helmet")), 1.0);
        assert_eq!(attack_damage(Some("survival:wooden_sword")), 4.0);
        assert_eq!(attack_damage(Some("survival:diamond_sword")), 7.0);
        assert!(
            attack_damage(Some("survival:iron_axe")) < attack_damage(Some("survival:iron_sword"))
        );
    }
}
