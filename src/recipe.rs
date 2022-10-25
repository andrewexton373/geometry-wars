use crate::inventory::InventoryItem;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Recipe {
    pub items_required: Vec<InventoryItem>,
    pub item_created: InventoryItem,
    pub time_required: f32,
}
