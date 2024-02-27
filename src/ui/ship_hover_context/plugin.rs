use bevy::app::{App, Plugin, Update};

use super::systems::ui_ship_hover_context;

pub struct ShipHoverContext;

impl Plugin for ShipHoverContext {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, ui_ship_hover_context);
    }
}
