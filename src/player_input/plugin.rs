use bevy::app::{Plugin, App, Update};
use bevy::math::Vec2;

use super::resources::{
    MouseScreenPosition,
    MouseWorldPosition
};

use super::events::EnginePowerEvent;

use super::systems::{
    update_mouse_screen_position_resource,
    update_mouse_world_position_resource,
    scroll_events
};

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EnginePowerEvent>()
            .insert_resource(MouseWorldPosition(Vec2::ZERO))
            .insert_resource(MouseScreenPosition(Vec2::ZERO))
            .add_systems(
                Update,
                (
                    update_mouse_world_position_resource,
                    update_mouse_screen_position_resource,
                    scroll_events,
                ),
            );
    }
}