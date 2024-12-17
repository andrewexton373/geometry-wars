use bevy::app::{App, Plugin, Update};

use super::systems::deposit_inventory;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_observer(deposit_inventory);
    }
}
