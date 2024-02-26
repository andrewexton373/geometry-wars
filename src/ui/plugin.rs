use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use super::{
    context_clue::plugin::ContextCluePlugin, mouse_coordinates::plugin::MouseCoordinatesPlugin, mouse_hover_context::plugin::MouseHoverContextPlugin, ship_hover_context::plugin::ShipHoverContext, ship_information::plugin::ShipInformationPlugin, ship_inventory::plugin::ShipInventoryPlugin, space_station_menu::plugin::SpaceStationMenu
};

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
            .add_plugins(EguiPlugin)
            .add_plugins((
                ContextCluePlugin,
                ShipInventoryPlugin,
                ShipInformationPlugin,
                SpaceStationMenu,
                MouseHoverContextPlugin,
                MouseCoordinatesPlugin
                // ShipHoverContext
            ));
    }
}

