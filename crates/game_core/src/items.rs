use crate::content::ItemId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ItemStack {
    pub item: ItemId,
    pub count: u16,
}

impl ItemStack {
    pub fn new(item: ItemId, count: u16) -> Self {
        Self { item, count }
    }
}
