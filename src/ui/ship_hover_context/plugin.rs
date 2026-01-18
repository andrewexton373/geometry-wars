use bevy::app::{App, Plugin, Update};
use bevy_egui::EguiPrimaryContextPass;

use super::systems::ui_ship_hover_context;

pub struct ShipHoverContext;

impl Plugin for ShipHoverContext {
    fn build(&self, app: &mut App) {
        app.add_systems(EguiPrimaryContextPass, ui_ship_hover_context);
    }
}
