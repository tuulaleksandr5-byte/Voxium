use crate::content::{ArmorSlot, ContentRegistry, ItemKind};
use crate::items::ItemStack;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum SlotRef {
    Hotbar(usize),
    Main(usize),
    Crafting(usize),
    Workbench(usize),
    Armor(ArmorSlot),
    Offhand,
    Container(usize),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ClickButton {
    Left,
    Right,
}

#[derive(Clone, Debug, Default)]
pub struct Inventory {
    pub hotbar: [Option<ItemStack>; 9],
    pub main: [Option<ItemStack>; 27],
    pub armor: [Option<ItemStack>; 4],
    pub offhand: Option<ItemStack>,
    pub cursor: Option<ItemStack>,
    pub crafting: [Option<ItemStack>; 4],
    pub workbench: [Option<ItemStack>; 9],
}

#[derive(Clone, Debug)]
pub struct Container {
    pub slots: Vec<Option<ItemStack>>,
}

impl Container {
    pub fn new(size: usize) -> Self {
        Self {
            slots: vec![None; size],
        }
    }
}

impl Inventory {
    pub fn get(&self, slot: SlotRef) -> Option<&ItemStack> {
        match slot {
            SlotRef::Hotbar(index) => self.hotbar.get(index)?.as_ref(),
            SlotRef::Main(index) => self.main.get(index)?.as_ref(),
            SlotRef::Crafting(index) => self.crafting.get(index)?.as_ref(),
            SlotRef::Workbench(index) => self.workbench.get(index)?.as_ref(),
            SlotRef::Armor(armor) => self.armor.get(armor.index())?.as_ref(),
            SlotRef::Offhand => self.offhand.as_ref(),
            SlotRef::Container(_) => None,
        }
    }

    pub fn set(
        &mut self,
        slot: SlotRef,
        stack: Option<ItemStack>,
        catalog: &ContentRegistry,
    ) -> Result<(), String> {
        if let Some(stack) = &stack {
            if !valid_stack(stack, catalog) {
                return Err("Неизвестный предмет или недопустимый размер стопки".into());
            }
            if !accepts(slot, stack, catalog) {
                return Err("Предмет не подходит для этого слота брони".into());
            }
        }
        let destination = self.slot_mut(slot).ok_or("Слот игрока не существует")?;
        *destination = stack;
        Ok(())
    }

    pub fn insert(&mut self, mut stack: ItemStack, catalog: &ContentRegistry) -> Option<ItemStack> {
        insert_slots(
            self.hotbar.iter_mut().chain(self.main.iter_mut()),
            &mut stack,
            catalog,
        );
        (stack.count > 0).then_some(stack)
    }

    pub fn click(
        &mut self,
        mut container: Option<&mut Container>,
        slot: SlotRef,
        button: ClickButton,
        shift: bool,
        catalog: &ContentRegistry,
    ) -> bool {
        if shift {
            return self.shift_click(container, slot, catalog);
        }
        if let Some(cursor) = &self.cursor
            && (!valid_stack(cursor, catalog) || !accepts(slot, cursor, catalog))
        {
            return false;
        }
        // Курсор временно отделён, чтобы отказ не требовал отката инвентаря.
        let mut cursor = self.cursor.take();
        let target = match slot {
            SlotRef::Container(index) => container
                .as_mut()
                .and_then(|chest| chest.slots.get_mut(index)),
            _ => self.slot_mut(slot),
        };
        let changed = target.is_some_and(|target| click_slot(target, &mut cursor, button, catalog));
        self.cursor = cursor;
        changed
    }

    pub fn take_selected(&mut self, index: usize, all: bool) -> Option<ItemStack> {
        let slot = self.hotbar.get_mut(index)?;
        let stack = slot.as_mut()?;
        if all || stack.count <= 1 {
            slot.take()
        } else {
            stack.count -= 1;
            Some(ItemStack::new(stack.item, 1))
        }
    }

    pub fn close_cursor(&mut self, catalog: &ContentRegistry) -> Option<ItemStack> {
        let stack = self.cursor.take()?;
        self.insert(stack, catalog)
    }

    /// Return ingredients once; callers own any overflow and must preserve or drop it.
    pub fn close_crafting(&mut self, catalog: &ContentRegistry) -> [Option<ItemStack>; 4] {
        std::mem::take(&mut self.crafting)
            .map(|stack| stack.and_then(|stack| self.insert(stack, catalog)))
    }

    pub fn close_workbench(&mut self, catalog: &ContentRegistry) -> [Option<ItemStack>; 9] {
        std::mem::take(&mut self.workbench)
            .map(|stack| stack.and_then(|stack| self.insert(stack, catalog)))
    }

    pub fn drain_all(&mut self) -> Vec<ItemStack> {
        self.hotbar
            .iter_mut()
            .chain(self.main.iter_mut())
            .chain(self.armor.iter_mut())
            .chain(self.crafting.iter_mut())
            .chain(self.workbench.iter_mut())
            .chain(std::iter::once(&mut self.offhand))
            .chain(std::iter::once(&mut self.cursor))
            .filter_map(Option::take)
            .collect()
    }

    fn slot_mut(&mut self, slot: SlotRef) -> Option<&mut Option<ItemStack>> {
        match slot {
            SlotRef::Hotbar(index) => self.hotbar.get_mut(index),
            SlotRef::Main(index) => self.main.get_mut(index),
            SlotRef::Crafting(index) => self.crafting.get_mut(index),
            SlotRef::Workbench(index) => self.workbench.get_mut(index),
            SlotRef::Armor(armor) => self.armor.get_mut(armor.index()),
            SlotRef::Offhand => Some(&mut self.offhand),
            SlotRef::Container(_) => None,
        }
    }

    fn shift_click(
        &mut self,
        container: Option<&mut Container>,
        slot: SlotRef,
        catalog: &ContentRegistry,
    ) -> bool {
        if self.cursor.is_some() {
            return false;
        }
        if let SlotRef::Container(index) = slot {
            let Some(source) = container.and_then(|chest| chest.slots.get_mut(index)) else {
                return false;
            };
            let Some(stack) = source.as_ref().filter(|stack| valid_stack(stack, catalog)) else {
                return false;
            };
            let before = stack.count;
            *source = self.insert(stack.clone(), catalog);
            return source.as_ref().map_or(0, |stack| stack.count) < before;
        }
        let Some(source) = self.get(slot).filter(|stack| valid_stack(stack, catalog)) else {
            return false;
        };
        let mut stack = source.clone();
        let before = stack.count;
        if let Some(container) = container {
            insert_slots(container.slots.iter_mut(), &mut stack, catalog);
        } else {
            match slot {
                SlotRef::Hotbar(_) => insert_slots(self.main.iter_mut(), &mut stack, catalog),
                SlotRef::Main(_) => insert_slots(self.hotbar.iter_mut(), &mut stack, catalog),
                SlotRef::Armor(_) | SlotRef::Offhand | SlotRef::Crafting(_) | SlotRef::Workbench(_) => insert_slots(
                    self.hotbar.iter_mut().chain(self.main.iter_mut()),
                    &mut stack,
                    catalog,
                ),
                SlotRef::Container(_) => unreachable!(),
            }
        }
        let changed = stack.count < before;
        *self.slot_mut(slot).expect("проверенный слот игрока") = (stack.count > 0).then_some(stack);
        changed
    }
}

fn valid_stack(stack: &ItemStack, catalog: &ContentRegistry) -> bool {
    stack.count > 0 && stack.count <= catalog.max_stack(stack.item)
}

fn accepts(slot: SlotRef, stack: &ItemStack, catalog: &ContentRegistry) -> bool {
    match slot {
        SlotRef::Armor(expected) => {
            matches!(catalog.get(stack.item).map(|item| &item.kind), Some(ItemKind::Armor { slot }) if *slot == expected)
        }
        _ => true,
    }
}

fn insert_slots<'a>(
    slots: impl Iterator<Item = &'a mut Option<ItemStack>>,
    stack: &mut ItemStack,
    catalog: &ContentRegistry,
) {
    let limit = catalog.max_stack(stack.item);
    if limit == 0 || stack.count == 0 {
        return;
    }
    // Сначала заполняем существующие стопки, затем свободные ячейки.
    // Индексы задаются вызывающим кодом: панель быстрого доступа раньше рюкзака.
    let mut slots: Vec<_> = slots.collect();
    for destination in &mut slots {
        if let Some(existing) = destination.as_mut()
            && existing.item == stack.item
            && existing.count < limit
        {
            let moved = stack.count.min(limit - existing.count);
            existing.count += moved;
            stack.count -= moved;
        }
    }
    for destination in slots {
        if destination.is_none() && stack.count > 0 {
            let moved = stack.count.min(limit);
            *destination = Some(ItemStack::new(stack.item, moved));
            stack.count -= moved;
        }
    }
}

fn click_slot(
    target: &mut Option<ItemStack>,
    cursor: &mut Option<ItemStack>,
    button: ClickButton,
    catalog: &ContentRegistry,
) -> bool {
    if target
        .as_ref()
        .is_some_and(|stack| !valid_stack(stack, catalog))
    {
        return false;
    }
    match (cursor.as_mut(), target.as_mut()) {
        (None, None) => false,
        (None, Some(source)) => {
            if button == ClickButton::Left {
                *cursor = target.take();
            } else {
                let count = source.count / 2 + source.count % 2;
                *cursor = Some(ItemStack::new(source.item, count));
                source.count -= count;
                if source.count == 0 {
                    *target = None;
                }
            }
            true
        }
        (Some(held), None) => {
            if button == ClickButton::Left {
                *target = cursor.take();
            } else {
                *target = Some(ItemStack::new(held.item, 1));
                held.count -= 1;
                if held.count == 0 {
                    *cursor = None;
                }
            }
            true
        }
        (Some(held), Some(destination)) if held.item == destination.item => {
            let available = catalog.max_stack(held.item) - destination.count;
            let moved = held
                .count
                .min(available)
                .min(if button == ClickButton::Right {
                    1
                } else {
                    u16::MAX
                });
            if moved == 0 {
                return false;
            }
            destination.count += moved;
            held.count -= moved;
            if held.count == 0 {
                *cursor = None;
            }
            true
        }
        (Some(_), Some(_)) if button == ClickButton::Left => {
            std::mem::swap(target, cursor);
            true
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::{ItemId, ItemKind};

    fn block(catalog: &ContentRegistry) -> ItemId {
        catalog.item_for_block(1).unwrap()
    }

    fn egg(catalog: &ContentRegistry) -> ItemId {
        catalog
            .iter()
            .find(|(_, item)| item.max_stack == 16)
            .unwrap()
            .0
    }

    fn helmet(catalog: &ContentRegistry) -> ItemId {
        catalog
            .iter()
            .find(|(_, item)| {
                matches!(
                    item.kind,
                    ItemKind::Armor {
                        slot: ArmorSlot::Head
                    }
                )
            })
            .unwrap()
            .0
    }

    fn fill_storage(inventory: &mut Inventory, item: ItemId, count: u16) {
        inventory.hotbar.fill(Some(ItemStack::new(item, count)));
        inventory.main.fill(Some(ItemStack::new(item, count)));
    }

    #[test]
    fn crafting_slots_support_click_split_shift_and_drain_without_auto_insert() {
        let catalog = ContentRegistry::default();
        let item = block(&catalog);
        let mut inventory = Inventory::default();
        assert!(
            inventory
                .set(
                    SlotRef::Crafting(4),
                    Some(ItemStack::new(item, 3)),
                    &catalog
                )
                .is_err()
        );
        inventory
            .set(
                SlotRef::Crafting(1),
                Some(ItemStack::new(item, 5)),
                &catalog,
            )
            .unwrap();
        assert!(inventory.click(
            None,
            SlotRef::Crafting(1),
            ClickButton::Right,
            false,
            &catalog
        ));
        assert_eq!(inventory.cursor, Some(ItemStack::new(item, 3)));
        assert!(inventory.click(
            None,
            SlotRef::Crafting(2),
            ClickButton::Right,
            false,
            &catalog
        ));
        assert_eq!(inventory.crafting[2], Some(ItemStack::new(item, 1)));
        assert!(!inventory.click(
            None,
            SlotRef::Crafting(2),
            ClickButton::Left,
            true,
            &catalog
        ));
        inventory.close_cursor(&catalog);
        assert!(inventory.click(
            None,
            SlotRef::Crafting(1),
            ClickButton::Left,
            true,
            &catalog
        ));
        assert!(inventory.crafting[1].is_none());
        assert_eq!(inventory.hotbar[0], Some(ItemStack::new(item, 4)));
        assert_eq!(
            inventory.drain_all().iter().map(|s| s.count).sum::<u16>(),
            5
        );
        assert!(inventory.drain_all().is_empty());
        fill_storage(&mut inventory, item, 64);
        assert_eq!(
            inventory.insert(ItemStack::new(item, 1), &catalog),
            Some(ItemStack::new(item, 1))
        );
        assert!(inventory.crafting.iter().all(Option::is_none));
    }
    #[test]
    fn close_crafting_returns_only_overflow_and_drains_ingredients_once() {
        let catalog = ContentRegistry::default();
        let item = block(&catalog);
        let mut inventory = Inventory::default();
        fill_storage(&mut inventory, item, 64);
        inventory.hotbar[0] = Some(ItemStack::new(item, 63));
        inventory.crafting[0] = Some(ItemStack::new(item, 3));
        inventory.crafting[3] = Some(ItemStack::new(item, 2));
        assert_eq!(
            inventory.close_crafting(&catalog),
            [
                Some(ItemStack::new(item, 2)),
                None,
                None,
                Some(ItemStack::new(item, 2))
            ]
        );
        assert_eq!(inventory.hotbar[0], Some(ItemStack::new(item, 64)));
        assert_eq!(inventory.close_crafting(&catalog), [None, None, None, None]);
    }

    #[test]
    fn insert_merges_before_filling_empty_slots_and_preserves_overflow() {
        let catalog = ContentRegistry::default();
        let item = block(&catalog);
        let mut inventory = Inventory::default();
        fill_storage(&mut inventory, item, 64);
        inventory.hotbar[0] = Some(ItemStack::new(item, 60));
        inventory.hotbar[1] = None;
        inventory.main[0] = Some(ItemStack::new(item, 63));
        assert_eq!(
            inventory.insert(ItemStack::new(item, 80), &catalog),
            Some(ItemStack::new(item, 11))
        );
        assert_eq!(inventory.hotbar[0], Some(ItemStack::new(item, 64)));
        assert_eq!(inventory.hotbar[1], Some(ItemStack::new(item, 64)));
        assert_eq!(inventory.main[0], Some(ItemStack::new(item, 64)));
        assert!(inventory.offhand.is_none());
        assert!(inventory.armor.iter().all(Option::is_none));
    }

    #[test]
    fn insert_splits_large_input_at_item_specific_limit() {
        let catalog = ContentRegistry::default();
        let item = egg(&catalog);
        let mut inventory = Inventory::default();
        assert_eq!(inventory.insert(ItemStack::new(item, 35), &catalog), None);
        assert_eq!(inventory.hotbar[0], Some(ItemStack::new(item, 16)));
        assert_eq!(inventory.hotbar[1], Some(ItemStack::new(item, 16)));
        assert_eq!(inventory.hotbar[2], Some(ItemStack::new(item, 3)));
        let unknown = ItemStack::new(ItemId(u16::MAX), 4);
        assert_eq!(inventory.insert(unknown.clone(), &catalog), Some(unknown));
    }

    #[test]
    fn set_rejects_bounds_unknown_zero_overfull_and_wrong_armor_without_mutation() {
        let catalog = ContentRegistry::default();
        let item = block(&catalog);
        let head = helmet(&catalog);
        let mut inventory = Inventory::default();
        inventory.hotbar[0] = Some(ItemStack::new(item, 2));
        for (slot, stack) in [
            (SlotRef::Hotbar(9), ItemStack::new(item, 1)),
            (SlotRef::Main(27), ItemStack::new(item, 1)),
            (SlotRef::Container(0), ItemStack::new(item, 1)),
            (SlotRef::Hotbar(0), ItemStack::new(item, 0)),
            (SlotRef::Hotbar(0), ItemStack::new(item, 65)),
            (SlotRef::Hotbar(0), ItemStack::new(ItemId(u16::MAX), 1)),
            (SlotRef::Armor(ArmorSlot::Head), ItemStack::new(item, 1)),
            (SlotRef::Armor(ArmorSlot::Feet), ItemStack::new(head, 1)),
        ] {
            assert!(inventory.set(slot, Some(stack), &catalog).is_err());
        }
        assert_eq!(
            inventory.get(SlotRef::Hotbar(0)),
            Some(&ItemStack::new(item, 2))
        );
        assert!(inventory.get(SlotRef::Container(0)).is_none());
        assert!(
            inventory
                .set(
                    SlotRef::Armor(ArmorSlot::Head),
                    Some(ItemStack::new(head, 1)),
                    &catalog
                )
                .is_ok()
        );
        assert!(
            inventory
                .set(SlotRef::Offhand, Some(ItemStack::new(item, 64)), &catalog)
                .is_ok()
        );
        assert!(inventory.set(SlotRef::Hotbar(0), None, &catalog).is_ok());
        assert!(inventory.get(SlotRef::Hotbar(0)).is_none());
    }

    #[test]
    fn left_click_picks_merges_with_limit_and_swaps_different_items() {
        let catalog = ContentRegistry::default();
        let item = block(&catalog);
        let other = egg(&catalog);
        let mut inventory = Inventory::default();
        inventory.hotbar[0] = Some(ItemStack::new(item, 10));
        inventory.main[0] = Some(ItemStack::new(item, 60));
        inventory.main[1] = Some(ItemStack::new(other, 3));
        assert!(inventory.click(None, SlotRef::Hotbar(0), ClickButton::Left, false, &catalog));
        assert!(inventory.hotbar[0].is_none());
        assert_eq!(inventory.cursor, Some(ItemStack::new(item, 10)));
        assert!(inventory.click(None, SlotRef::Main(0), ClickButton::Left, false, &catalog));
        assert_eq!(inventory.main[0], Some(ItemStack::new(item, 64)));
        assert_eq!(inventory.cursor, Some(ItemStack::new(item, 6)));
        assert!(!inventory.click(None, SlotRef::Main(0), ClickButton::Left, false, &catalog));
        assert!(inventory.click(None, SlotRef::Main(1), ClickButton::Left, false, &catalog));
        assert_eq!(inventory.main[1], Some(ItemStack::new(item, 6)));
        assert_eq!(inventory.cursor, Some(ItemStack::new(other, 3)));
        assert!(inventory.click(None, SlotRef::Hotbar(0), ClickButton::Left, false, &catalog));
        assert!(inventory.cursor.is_none());
    }

    #[test]
    fn right_click_takes_ceil_half_and_places_exactly_one() {
        let catalog = ContentRegistry::default();
        let item = egg(&catalog);
        let mut inventory = Inventory::default();
        inventory.main[0] = Some(ItemStack::new(item, 5));
        assert!(inventory.click(None, SlotRef::Main(0), ClickButton::Right, false, &catalog));
        assert_eq!(inventory.main[0], Some(ItemStack::new(item, 2)));
        assert_eq!(inventory.cursor, Some(ItemStack::new(item, 3)));
        assert!(inventory.click(None, SlotRef::Main(1), ClickButton::Right, false, &catalog));
        assert_eq!(inventory.main[1], Some(ItemStack::new(item, 1)));
        assert_eq!(inventory.cursor, Some(ItemStack::new(item, 2)));
        assert!(inventory.click(None, SlotRef::Main(0), ClickButton::Right, false, &catalog));
        assert_eq!(inventory.main[0], Some(ItemStack::new(item, 3)));
        assert_eq!(inventory.cursor, Some(ItemStack::new(item, 1)));
        assert!(inventory.click(None, SlotRef::Offhand, ClickButton::Right, false, &catalog));
        assert!(inventory.cursor.is_none());
        assert_eq!(inventory.offhand, Some(ItemStack::new(item, 1)));
    }

    #[test]
    fn shift_click_moves_between_player_sections_and_preserves_partial_remainder() {
        let catalog = ContentRegistry::default();
        let item = block(&catalog);
        let mut inventory = Inventory::default();
        inventory.main.fill(Some(ItemStack::new(item, 64)));
        inventory.main[0] = Some(ItemStack::new(item, 62));
        inventory.hotbar[0] = Some(ItemStack::new(item, 5));
        assert!(inventory.click(None, SlotRef::Hotbar(0), ClickButton::Left, true, &catalog));
        assert_eq!(inventory.main[0], Some(ItemStack::new(item, 64)));
        assert_eq!(inventory.hotbar[0], Some(ItemStack::new(item, 3)));
        assert!(!inventory.click(None, SlotRef::Hotbar(0), ClickButton::Left, true, &catalog));
        assert!(inventory.click(None, SlotRef::Main(1), ClickButton::Right, true, &catalog));
        assert!(inventory.main[1].is_none());
        assert_eq!(inventory.hotbar[0], Some(ItemStack::new(item, 64)));
        assert_eq!(inventory.hotbar[1], Some(ItemStack::new(item, 3)));
    }

    #[test]
    fn chest_shift_transfer_uses_open_container_and_keeps_overflow_at_source() {
        let catalog = ContentRegistry::default();
        let item = egg(&catalog);
        let mut inventory = Inventory::default();
        let mut chest = Container::new(2);
        inventory.hotbar[0] = Some(ItemStack::new(item, 10));
        chest.slots[0] = Some(ItemStack::new(item, 14));
        chest.slots[1] = Some(ItemStack::new(item, 16));
        assert!(inventory.click(
            Some(&mut chest),
            SlotRef::Hotbar(0),
            ClickButton::Left,
            true,
            &catalog
        ));
        assert_eq!(inventory.hotbar[0], Some(ItemStack::new(item, 8)));
        assert_eq!(chest.slots[0], Some(ItemStack::new(item, 16)));
        assert!(inventory.main.iter().all(Option::is_none));
        assert!(inventory.click(
            Some(&mut chest),
            SlotRef::Container(0),
            ClickButton::Left,
            true,
            &catalog
        ));
        assert!(chest.slots[0].is_none());
        assert_eq!(inventory.hotbar[0], Some(ItemStack::new(item, 16)));
        assert_eq!(inventory.hotbar[1], Some(ItemStack::new(item, 8)));
        assert!(!inventory.click(
            Some(&mut chest),
            SlotRef::Container(9),
            ClickButton::Left,
            false,
            &catalog
        ));
        assert!(!inventory.click(
            None,
            SlotRef::Container(0),
            ClickButton::Left,
            false,
            &catalog
        ));
    }

    #[test]
    fn armor_rejection_and_shift_with_cursor_leave_everything_unchanged() {
        let catalog = ContentRegistry::default();
        let item = block(&catalog);
        let head = helmet(&catalog);
        let mut inventory = Inventory {
            cursor: Some(ItemStack::new(item, 3)),
            ..Inventory::default()
        };
        inventory.hotbar[0] = Some(ItemStack::new(item, 4));
        assert!(!inventory.click(
            None,
            SlotRef::Armor(ArmorSlot::Head),
            ClickButton::Left,
            false,
            &catalog
        ));
        assert!(!inventory.click(
            None,
            SlotRef::Armor(ArmorSlot::Head),
            ClickButton::Right,
            false,
            &catalog
        ));
        assert!(!inventory.click(None, SlotRef::Hotbar(0), ClickButton::Left, true, &catalog));
        assert_eq!(inventory.cursor, Some(ItemStack::new(item, 3)));
        assert_eq!(inventory.hotbar[0], Some(ItemStack::new(item, 4)));
        inventory.cursor = Some(ItemStack::new(head, 1));
        assert!(!inventory.click(
            None,
            SlotRef::Armor(ArmorSlot::Feet),
            ClickButton::Left,
            false,
            &catalog
        ));
        assert!(inventory.click(
            None,
            SlotRef::Armor(ArmorSlot::Head),
            ClickButton::Left,
            false,
            &catalog
        ));
        inventory.cursor = Some(ItemStack::new(item, 3));
        assert!(!inventory.click(
            None,
            SlotRef::Armor(ArmorSlot::Head),
            ClickButton::Left,
            false,
            &catalog
        ));
        assert_eq!(
            inventory.armor[ArmorSlot::Head.index()],
            Some(ItemStack::new(head, 1))
        );
        assert_eq!(inventory.cursor, Some(ItemStack::new(item, 3)));
    }

    #[test]
    fn container_clicks_split_and_place_without_touching_player_storage() {
        let catalog = ContentRegistry::default();
        let item = egg(&catalog);
        let mut inventory = Inventory::default();
        let mut chest = Container::new(1);
        chest.slots[0] = Some(ItemStack::new(item, 3));
        assert!(inventory.click(
            Some(&mut chest),
            SlotRef::Container(0),
            ClickButton::Right,
            false,
            &catalog
        ));
        assert_eq!(inventory.cursor, Some(ItemStack::new(item, 2)));
        assert_eq!(chest.slots[0], Some(ItemStack::new(item, 1)));
        assert!(inventory.click(
            Some(&mut chest),
            SlotRef::Container(0),
            ClickButton::Left,
            false,
            &catalog
        ));
        assert_eq!(chest.slots[0], Some(ItemStack::new(item, 3)));
        assert!(inventory.cursor.is_none());
        assert!(inventory.hotbar.iter().all(Option::is_none));
    }

    #[test]
    fn close_cursor_restores_what_fits_and_returns_overflow_once() {
        let catalog = ContentRegistry::default();
        let item = block(&catalog);
        let mut inventory = Inventory::default();
        fill_storage(&mut inventory, item, 64);
        inventory.main[26] = Some(ItemStack::new(item, 62));
        inventory.cursor = Some(ItemStack::new(item, 5));
        assert_eq!(
            inventory.close_cursor(&catalog),
            Some(ItemStack::new(item, 3))
        );
        assert_eq!(inventory.main[26], Some(ItemStack::new(item, 64)));
        assert!(inventory.cursor.is_none());
        assert_eq!(inventory.close_cursor(&catalog), None);
    }

    #[test]
    fn dropping_selected_one_or_all_handles_last_item_and_invalid_indices() {
        let catalog = ContentRegistry::default();
        let item = block(&catalog);
        let mut inventory = Inventory::default();
        inventory.hotbar[8] = Some(ItemStack::new(item, 3));
        assert_eq!(inventory.take_selected(9, true), None);
        assert_eq!(
            inventory.take_selected(8, false),
            Some(ItemStack::new(item, 1))
        );
        assert_eq!(inventory.hotbar[8], Some(ItemStack::new(item, 2)));
        assert_eq!(
            inventory.take_selected(8, true),
            Some(ItemStack::new(item, 2))
        );
        assert!(inventory.hotbar[8].is_none());
        inventory.hotbar[8] = Some(ItemStack::new(item, 1));
        assert_eq!(
            inventory.take_selected(8, false),
            Some(ItemStack::new(item, 1))
        );
        assert_eq!(inventory.take_selected(8, false), None);
    }

    #[test]
    fn drain_all_includes_every_slot_and_cursor_exactly_once() {
        let catalog = ContentRegistry::default();
        let item = block(&catalog);
        let mut inventory = Inventory::default();
        fill_storage(&mut inventory, item, 1);
        inventory.armor.fill(Some(ItemStack::new(item, 2)));
        inventory.offhand = Some(ItemStack::new(item, 3));
        inventory.cursor = Some(ItemStack::new(item, 4));
        let drained = inventory.drain_all();
        assert_eq!(drained.len(), 42);
        assert_eq!(
            drained
                .iter()
                .map(|stack| u32::from(stack.count))
                .sum::<u32>(),
            51
        );
        assert!(inventory.drain_all().is_empty());
        assert!(
            inventory
                .hotbar
                .iter()
                .chain(&inventory.main)
                .chain(&inventory.armor)
                .all(Option::is_none)
        );
        assert!(inventory.offhand.is_none());
        assert!(inventory.cursor.is_none());
    }

    #[test]
    fn maximum_input_count_cannot_overflow_or_escape_slot_limits() {
        let catalog = ContentRegistry::default();
        let item = block(&catalog);
        let mut inventory = Inventory::default();
        assert_eq!(
            inventory.insert(ItemStack::new(item, u16::MAX), &catalog),
            Some(ItemStack::new(item, 63_231))
        );
        assert!(
            inventory
                .hotbar
                .iter()
                .chain(&inventory.main)
                .all(|slot| { slot.as_ref() == Some(&ItemStack::new(item, 64)) })
        );
    }

    #[test]
    fn invalid_targets_and_right_click_on_other_item_preserve_held_stack() {
        let catalog = ContentRegistry::default();
        let item = block(&catalog);
        let other = egg(&catalog);
        let mut inventory = Inventory {
            cursor: Some(ItemStack::new(item, 3)),
            ..Inventory::default()
        };
        inventory.main[0] = Some(ItemStack::new(other, 7));
        inventory.main[1] = Some(ItemStack::new(item, 64));
        let mut chest = Container::new(0);
        for slot in [
            SlotRef::Hotbar(9),
            SlotRef::Main(27),
            SlotRef::Container(0),
            SlotRef::Main(0),
            SlotRef::Main(1),
        ] {
            assert!(!inventory.click(Some(&mut chest), slot, ClickButton::Right, false, &catalog));
            assert_eq!(inventory.cursor, Some(ItemStack::new(item, 3)));
        }
        assert_eq!(inventory.main[0], Some(ItemStack::new(other, 7)));
        assert_eq!(inventory.main[1], Some(ItemStack::new(item, 64)));
        inventory.cursor = None;
        inventory.hotbar[0] = Some(ItemStack::new(item, 3));
        assert!(!inventory.click(
            Some(&mut chest),
            SlotRef::Hotbar(0),
            ClickButton::Left,
            true,
            &catalog
        ));
        assert_eq!(inventory.hotbar[0], Some(ItemStack::new(item, 3)));
    }

    #[test]
    fn shift_unequips_to_storage_without_auto_equipping_other_items() {
        let catalog = ContentRegistry::default();
        let head = helmet(&catalog);
        let mut inventory = Inventory::default();
        inventory
            .set(
                SlotRef::Armor(ArmorSlot::Head),
                Some(ItemStack::new(head, 1)),
                &catalog,
            )
            .unwrap();
        assert!(inventory.click(
            None,
            SlotRef::Armor(ArmorSlot::Head),
            ClickButton::Left,
            true,
            &catalog
        ));
        assert!(inventory.armor[ArmorSlot::Head.index()].is_none());
        assert_eq!(inventory.hotbar[0], Some(ItemStack::new(head, 1)));
        inventory.cursor = Some(ItemStack::new(head, 1));
        assert_eq!(inventory.close_cursor(&catalog), None);
        assert_eq!(inventory.hotbar[1], Some(ItemStack::new(head, 1)));
        assert!(inventory.armor.iter().all(Option::is_none));
    }

    #[test]
    fn mixed_click_sequence_conserves_each_item_and_enforces_all_caps() {
        let catalog = ContentRegistry::default();
        let item = block(&catalog);
        let eggs = egg(&catalog);
        let head = helmet(&catalog);
        let mut inventory = Inventory::default();
        let mut chest = Container::new(27);
        inventory.hotbar[0] = Some(ItemStack::new(item, 64));
        inventory.hotbar[1] = Some(ItemStack::new(item, 60));
        inventory.main[0] = Some(ItemStack::new(item, 9));
        inventory.main[1] = Some(ItemStack::new(eggs, 15));
        inventory.offhand = Some(ItemStack::new(eggs, 2));
        inventory.armor[ArmorSlot::Head.index()] = Some(ItemStack::new(head, 1));
        chest.slots[0] = Some(ItemStack::new(eggs, 16));
        chest.slots[1] = Some(ItemStack::new(eggs, 1));
        let mut state = 17_u32;
        for step in 0..1_000 {
            state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            let slot = match state % 7 {
                0 => SlotRef::Hotbar((state / 7 % 10) as usize),
                1 => SlotRef::Main((state / 7 % 28) as usize),
                2 => SlotRef::Container((state / 7 % 28) as usize),
                3 => SlotRef::Offhand,
                4 => SlotRef::Armor(ArmorSlot::Head),
                5 => SlotRef::Armor(ArmorSlot::Feet),
                _ => SlotRef::Main(0),
            };
            let button = if state & 8 == 0 {
                ClickButton::Left
            } else {
                ClickButton::Right
            };
            inventory.click(Some(&mut chest), slot, button, state & 16 == 0, &catalog);
            if step % 37 == 0 {
                assert_eq!(inventory.close_cursor(&catalog), None);
            }
            let mut totals = [0_u32; 3];
            for stack in inventory
                .hotbar
                .iter()
                .chain(&inventory.main)
                .chain(&inventory.armor)
                .chain(std::iter::once(&inventory.offhand))
                .chain(std::iter::once(&inventory.cursor))
                .chain(&chest.slots)
                .flatten()
            {
                assert!(stack.count > 0 && stack.count <= catalog.max_stack(stack.item));
                let index = [item, eggs, head]
                    .iter()
                    .position(|id| *id == stack.item)
                    .unwrap();
                totals[index] += u32::from(stack.count);
            }
            assert_eq!(totals, [133, 34, 1], "шаг {step}");
            assert!(inventory.armor[ArmorSlot::Feet.index()].is_none());
            assert!(
                inventory.armor[ArmorSlot::Head.index()]
                    .as_ref()
                    .is_none_or(|stack| stack.item == head)
            );
        }
    }
}
