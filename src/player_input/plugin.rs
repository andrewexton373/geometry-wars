use bevy::app::{App, Plugin, PreUpdate, Update};
use bevy::ecs::system::System;
use bevy::math::Vec2;

use crate::rcs::events::RCSThrustPowerEvent;

use super::resources::{MouseScreenPosition, MouseWorldPosition};

use super::events::DepositInventoryEvent;

use super::systems::{
    cancel_player_targeting, grab_mouse, player_camera_control, player_deposit_control,
    player_targeting, scroll_events, update_mouse_screen_position_resource,
    update_mouse_world_position_resource,
};

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DepositInventoryEvent>()
            .insert_resource(MouseWorldPosition(Vec2::ZERO))
            .insert_resource(MouseScreenPosition(Vec2::ZERO))
            .add_systems(
                PreUpdate,
                (
                    // grab_mouse,
                    update_mouse_world_position_resource,
                    update_mouse_screen_position_resource,
                    scroll_events,
                    player_camera_control,
                    player_deposit_control,
                    player_targeting,
                    cancel_player_targeting,
                ),
            );
    }
}
