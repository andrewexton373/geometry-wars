use bevy::{app::{App, Plugin, Update}, ecs::schedule::IntoSystemConfigs};

use super::{resources::MouseHoverContext, systems::{ui_mouse_hover_context, update_mouse_hover_context_resource}};

pub struct MouseHoverContextPlugin;

impl Plugin for MouseHoverContextPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MouseHoverContext>()
            .add_systems(Update, (
                update_mouse_hover_context_resource,
                ui_mouse_hover_context.after(update_mouse_hover_context_resource),
            ));
    }
}
