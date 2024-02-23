use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use super::{context_clue::plugin::ContextCluePlugin, ship_inventory::plugin::ShipInventoryPlugin, systems::{ui_mouse_hover_context, ui_ship_information, ui_station_menu}};

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
            .add_plugins(EguiPlugin)
            .add_plugins((ContextCluePlugin, ShipInventoryPlugin))
            .add_systems(
                Update,
                (
                    ui_ship_information,
                    ui_station_menu,
                    ui_mouse_hover_context,
                    // Self::ui_ship_hover_context,
                ),
            );
    }
}