use bevy::{app::Update, prelude::{App, Plugin}};

use super::systems::ui_ship_inventory;

pub struct ShipInventoryPlugin;

impl Plugin for ShipInventoryPlugin {
    fn build(&self,app: &mut App) {
        app
            .add_systems(Update, (ui_ship_inventory));
    }
}