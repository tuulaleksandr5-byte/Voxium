use game_core::{
    content::ContentRegistry,
    crafting::RecipeBook,
    furnace::FurnaceState,
    inventory::{Container, Inventory},
    items::ItemStack,
};

fn stack(catalog: &ContentRegistry, id: &str, count: u16) -> Option<ItemStack> {
    Some(ItemStack::new(catalog.find(id).expect("item in catalog"), count))
}

#[test]
fn three_by_three_workbench_crafts_tool_and_consumes_ingredients_once() {
    let catalog = ContentRegistry::default();
    let recipes = RecipeBook::default();
    let mut inventory = Inventory::default();
    inventory.workbench[0] = stack(&catalog, "survival:iron_ingot", 1);
    inventory.workbench[1] = stack(&catalog, "survival:iron_ingot", 1);
    inventory.workbench[2] = stack(&catalog, "survival:iron_ingot", 1);
    inventory.workbench[4] = stack(&catalog, "survival:stick", 1);
    inventory.workbench[7] = stack(&catalog, "survival:stick", 1);
    let output = recipes.preview_workbench(&inventory.workbench, &catalog).unwrap();
    assert_eq!(output.item, catalog.find("survival:iron_pickaxe").unwrap());
    assert!(recipes.take_workbench_result(&mut inventory, false, &catalog));
    assert_eq!(inventory.cursor, Some(output));
    assert!(inventory.workbench.iter().all(Option::is_none));
    assert!(!recipes.take_workbench_result(&mut inventory, false, &catalog));
}

#[test]
fn furnace_consumes_one_coal_and_smelt_input_without_duplicates() {
    let catalog = ContentRegistry::default();
    let mut container = Container::new(3);
    container.slots[0] = stack(&catalog, "survival:raw_iron", 2);
    container.slots[1] = stack(&catalog, "survival:coal", 1);
    let mut furnace = FurnaceState::default();
    assert!(!furnace.tick(&mut container, 0.0, &catalog));
    assert!(furnace.tick(&mut container, 10.0, &catalog));
    assert_eq!(container.slots[0], stack(&catalog, "survival:raw_iron", 1));
    assert_eq!(container.slots[1], None);
    assert_eq!(container.slots[2], stack(&catalog, "survival:iron_ingot", 1));
    assert!(furnace.tick(&mut container, 10.0, &catalog));
    assert_eq!(container.slots[0], None);
    assert_eq!(container.slots[2], stack(&catalog, "survival:iron_ingot", 2));
    assert!(!furnace.tick(&mut container, 10.0, &catalog));
}

#[test]
fn furnace_refuses_full_or_mismatched_output_without_consuming_fuel() {
    let catalog = ContentRegistry::default();
    let mut container = Container::new(3);
    container.slots[0] = stack(&catalog, "survival:raw_gold", 1);
    container.slots[1] = stack(&catalog, "survival:coal", 1);
    container.slots[2] = stack(&catalog, "survival:iron_ingot", 64);
    let mut furnace = FurnaceState::default();
    assert!(!furnace.tick(&mut container, 10.0, &catalog));
    assert_eq!(container.slots[0], stack(&catalog, "survival:raw_gold", 1));
    assert_eq!(container.slots[1], stack(&catalog, "survival:coal", 1));
    assert_eq!(container.slots[2], stack(&catalog, "survival:iron_ingot", 64));
}
