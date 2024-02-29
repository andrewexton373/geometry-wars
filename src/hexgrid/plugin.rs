use bevy::prelude::*;

use crate::AppState;

use super::events::BuildHexBuildingEvent;
use super::resources::{MouseHoverHex, SelectedHex};
use super::systems::{update_mouse_hover_hex, update_selected_hex};
use super::{
    resources::{HighlightedHexes, PlayerHoveringBuilding},
    systems::{
        color_hexes, handle_mouse_interaction, handle_ship_hovering_context, setup_hex_grid,
    },
};

/// World size of the hexagons (outer radius)
pub const HEX_SIZE: Vec2 = Vec2::splat(10.0 * crate::PIXELS_PER_METER);

pub struct HexBasePlugin;

impl Plugin for HexBasePlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_plugin(TilemapPlugin)
            .add_event::<BuildHexBuildingEvent>()
            .init_resource::<PlayerHoveringBuilding>()
            .init_resource::<HighlightedHexes>()
            .init_resource::<SelectedHex>()
            .init_resource::<MouseHoverHex>()
            .add_systems(Startup, setup_hex_grid)
            .add_systems(
                Update,
                (
                    color_hexes.run_if(in_state(AppState::BuildMode)),
                    handle_mouse_interaction,
                    handle_ship_hovering_context,
                    update_mouse_hover_hex,
                    update_selected_hex.after(update_mouse_hover_hex)
                    // Self::handle_build_events,
                ),
            );
    }
}
