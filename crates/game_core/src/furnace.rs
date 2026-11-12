//! Simple deterministic furnace process. Slots: input (0), fuel (1), output (2).
use crate::{content::ContentRegistry, inventory::Container, items::ItemStack};

const COOK_SECONDS: f32 = 10.0;
const COAL_SECONDS: f32 = 80.0;

#[derive(Clone, Debug, Default)]
pub struct FurnaceState {
    pub progress: f32,
    pub fuel_remaining: f32,
}

impl FurnaceState {
    /// Returns true only when at least one input is smelted or fuel is consumed.
    pub fn tick(&mut self, inventory: &mut Container, seconds: f32, catalog: &ContentRegistry) -> bool {
        if inventory.slots.len() != 3 || !seconds.is_finite() || seconds <= 0.0 {
            return false;
        }
        let mut remaining = seconds.min(120.0);
        let mut changed = false;
        while remaining > 0.0001 {
            let Some(input) = inventory.slots[0].as_ref() else {
                self.progress = 0.0;
                break;
            };
            let Some(output_id) = smelted_item(input, catalog) else {
                self.progress = 0.0;
                break;
            };
            let output_cap = catalog.max_stack(output_id);
            if !inventory.slots[2].as_ref().is_none_or(|s|
                s.item == output_id && s.count < output_cap
            ) {
                break;
            }
            if self.fuel_remaining <= 0.0001 {
                let Some(fuel) = inventory.slots[1].as_mut() else { break; };
                if catalog.get(fuel.item).is_none_or(|item| item.id != "survival:coal") {
                    break;
                }
                fuel.count -= 1;
                if fuel.count == 0 { inventory.slots[1] = None; }
                self.fuel_remaining = COAL_SECONDS;
                changed = true;
            }
            let step = remaining
                .min(self.fuel_remaining)
                .min((COOK_SECONDS - self.progress).max(0.0));
            if step <= 0.0 { break; }
            remaining -= step;
            self.fuel_remaining -= step;
            self.progress += step;
            if self.progress + 0.0001 >= COOK_SECONDS {
                self.progress = 0.0;
                if let Some(input) = inventory.slots[0].as_mut() {
                    input.count -= 1;
                    if input.count == 0 { inventory.slots[0] = None; }
                }
                if let Some(output) = inventory.slots[2].as_mut() {
                    output.count += 1;
                } else {
                    inventory.slots[2] = Some(ItemStack::new(output_id, 1));
                }
                changed = true;
            }
        }
        changed
    }
}

fn smelted_item(input: &ItemStack, catalog: &ContentRegistry) -> Option<crate::content::ItemId> {
    let result = match catalog.get(input.item)?.id.as_str() {
        "survival:raw_iron" => "survival:iron_ingot",
        "survival:raw_gold" => "survival:gold_ingot",
        "foundation:sand" => "foundation:glass",
        _ => return None,
    };
    catalog.find(result)
}
