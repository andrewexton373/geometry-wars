use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::GameCamera;

use super::events::EnginePowerEvent;
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