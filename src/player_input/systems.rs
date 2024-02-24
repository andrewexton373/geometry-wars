use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::space_station::resources::CanDeposit;
use crate::GameCamera;

use super::events::{DepositInventoryEvent, EnginePowerEvent};
use super::resources::{
    MouseScreenPosition,
    MouseWorldPosition
};

pub fn update_mouse_world_position_resource(
    mut mouse_position: ResMut<MouseWorldPosition>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    _cursor_event: EventReader<CursorMoved>,
    q_camera: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
) {
    let window = window_query.single();

    let (camera, camera_transform) = q_camera.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        *mouse_position = MouseWorldPosition(world_position);
    }
}

pub fn update_mouse_screen_position_resource(
    mut mouse_position: ResMut<MouseScreenPosition>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    _cursor_event: EventReader<CursorMoved>,
) {
    let window = window_query.single();
    if let Some(pos) = window.cursor_position() {
        *mouse_position = MouseScreenPosition(pos);
    }
}

pub fn scroll_events(
    mut scroll_events: EventReader<MouseWheel>,
    mut engine_events: EventWriter<EnginePowerEvent>,
) {
    use bevy::input::mouse::MouseScrollUnit;

    for event in scroll_events.read() {
        match event.unit {
            MouseScrollUnit::Line => {
                engine_events.send(EnginePowerEvent(event.y));
            }
            MouseScrollUnit::Pixel => {
                engine_events.send(EnginePowerEvent(event.y));
            }
        }
    }
}

/// Allow the player to use , and . to zoom the viewport in and out.
pub fn player_camera_control(
    kb: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut OrthographicProjection, With<Camera2d>>,
) {
    let dist = 0.75 * time.delta().as_secs_f32();

    for mut projection in query.iter_mut() {
        let mut log_scale = projection.scale.ln();

        if kb.pressed(KeyCode::Period) {
            log_scale -= dist;
        }
        if kb.pressed(KeyCode::Comma) {
            log_scale += dist;
        }

        projection.scale = log_scale.exp();
    }
}

// TODO: Idea?
// Mark InventoryItems with Deposit Component on Event
// Use this system to deposit marked inventory items in Base Station
pub fn player_deposit_control(
    kb: Res<Input<KeyCode>>,
    can_deposit: Res<CanDeposit>,
    mut deposit_events: EventWriter<DepositInventoryEvent>,
) {
    // If player pressed space and they're in depositing range
    if kb.just_pressed(KeyCode::Space) && can_deposit.0 {
       deposit_events.send(DepositInventoryEvent);
    }
}