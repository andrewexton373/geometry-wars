use bevy::app::{App, Plugin, Update};

use super::systems::handle_deposit_inventory_event;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_deposit_inventory_event);
    }
}
