use bevy::app::{App, Plugin, Update};

use super::systems::ui_mouse_coordinates;

pub struct MouseCoordinatesPlugin;

impl Plugin for MouseCoordinatesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                ui_mouse_coordinates
            ));
    }
}