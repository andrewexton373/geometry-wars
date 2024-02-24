use bevy::app::{Plugin, App, Update};
use bevy::math::Vec2;

use super::resources::{
    MouseScreenPosition,
    MouseWorldPosition
};

use super::events::{DepositInventoryEvent, EnginePowerEvent};

use super::systems::{
    player_camera_control, player_deposit_control, scroll_events, update_mouse_screen_position_resource, update_mouse_world_position_resource
};

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<EnginePowerEvent>()
            .add_event::<DepositInventoryEvent>()
            .insert_resource(MouseWorldPosition(Vec2::ZERO))
            .insert_resource(MouseScreenPosition(Vec2::ZERO))
            .add_systems(
                Update,
                (
                    update_mouse_world_position_resource,
                    update_mouse_screen_position_resource,
                    scroll_events,
                    player_camera_control,
                    player_deposit_control
                ),
            );
    }
}