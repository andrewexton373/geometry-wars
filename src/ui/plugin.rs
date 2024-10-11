use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::hexgrid::systems::update_selected_hex;

use super::{
    build_mode::plugin::BuildModeUIPlugin, context_clue::plugin::ContextCluePlugin,
    damage_indicator::plugin::DamageIndicatorPlugin, helpers::absorb_egui_inputs,
    mouse_coordinates::plugin::MouseCoordinatesPlugin,
    mouse_hover_context::plugin::MouseHoverContextPlugin,
    ship_hover_context::plugin::ShipHoverContext, ship_information::plugin::ShipInformationPlugin,
    ship_inventory::plugin::ShipInventoryPlugin, space_station_menu::plugin::SpaceStationMenu,
};

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(EguiPlugin)
            .add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)))
            .add_plugins((
                ContextCluePlugin,
                ShipInventoryPlugin,
                ShipInformationPlugin,
                SpaceStationMenu,
                MouseHoverContextPlugin,
                MouseCoordinatesPlugin,
                DamageIndicatorPlugin,
                // ShipHoverContext
                BuildModeUIPlugin,
            ))
            .add_systems(
                PreUpdate,
                (absorb_egui_inputs)
                    // .after(bevy_egui::systems::process_input_system)
                    .after(update_selected_hex)
                    .before(bevy_egui::EguiSet::BeginPass),
            );
    }
}
