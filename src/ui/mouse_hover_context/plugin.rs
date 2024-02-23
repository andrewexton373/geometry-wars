use bevy::app::{App, Plugin, Update};

use super::systems::ui_mouse_hover_context;

pub struct MouseHoverContextPlugin;

impl Plugin for MouseHoverContextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (ui_mouse_hover_context));
    }
}
