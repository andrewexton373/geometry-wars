use bevy::app::{App, Plugin};
use bevy_egui::EguiPrimaryContextPass;

use super::systems::ui_mouse_coordinates;

pub struct MouseCoordinatesPlugin;

impl Plugin for MouseCoordinatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(EguiPrimaryContextPass, ui_mouse_coordinates);
    }
}
