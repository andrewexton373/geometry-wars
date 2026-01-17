use bevy::{
    app::Update,
    prelude::{App, Plugin},
};
use bevy_egui::EguiPrimaryContextPass;

use super::systems::ui_ship_inventory;

pub struct ShipInventoryPlugin;

impl Plugin for ShipInventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(EguiPrimaryContextPass, ui_ship_inventory);
    }
}
