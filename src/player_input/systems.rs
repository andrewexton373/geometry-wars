use bevy::input::mouse::{MouseWheel};
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};

use crate::camera::components::{CameraTarget, GameCamera};
use crate::player::components::Player;
use crate::rcs::events::RCSThrustPowerEvent;
use crate::space_station::resources::CanDeposit;
use crate::ui::mouse_hover_context::resources::MouseHoverContext;

use super::events::DepositInventoryEvent;
use super::resources::{MouseScreenPosition, MouseWorldPosition};

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
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
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

pub fn player_targeting(
    mut commands: Commands,
    mut camera_target_q: Query<Entity, With<CameraTarget>>,
    mouse_hover_context: Res<MouseHoverContext>,
    mouse_events: Res<ButtonInput<MouseButton>>,
) {
    // If mouse right click
    if mouse_events.just_pressed(MouseButton::Right) {
        // If mouse hover context is valid,
        if let Some(hover_context_ent) = mouse_hover_context.0 {
            // Remove old CameraTarget Component
            for camera_target in camera_target_q.iter_mut() {
                commands.entity(camera_target).remove::<CameraTarget>();
            }

            // Add CameraTarget to hover context entity
            commands.entity(hover_context_ent).insert(CameraTarget);
        }
    }
}

pub fn cancel_player_targeting(
    mut commands: Commands,
    mut camera_target_q: Query<Entity, With<CameraTarget>>,
    mut player_q: Query<Entity, With<Player>>,
    keyboard_events: Res<ButtonInput<KeyCode>>,
) {
    // If player presses X
    if keyboard_events.just_pressed(KeyCode::KeyX) {
        // Remove old CameraTarget Component
        for camera_target in camera_target_q.iter_mut() {
            commands.entity(camera_target).remove::<CameraTarget>();
        }

        // Add CameraTarget to Player as default
        let player_ent = player_q.get_single_mut().unwrap();
        commands.entity(player_ent).insert(CameraTarget);
    }
}

pub fn scroll_events(
    mut scroll_events: EventReader<MouseWheel>,
    mut engine_events: EventWriter<RCSThrustPowerEvent>,
) {
    use bevy::input::mouse::MouseScrollUnit;

    for event in scroll_events.read() {
        match event.unit {
            MouseScrollUnit::Line => {
                engine_events.send(RCSThrustPowerEvent(event.y));
            }
            MouseScrollUnit::Pixel => {
                engine_events.send(RCSThrustPowerEvent(event.y));
            }
        }
    }
}

/// Allow the player to use , and . to zoom the viewport in and out.
pub fn player_camera_control(
    kb: Res<ButtonInput<KeyCode>>,
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
    kb: Res<ButtonInput<KeyCode>>,
    can_deposit: Res<CanDeposit>,
    mut deposit_events: EventWriter<DepositInventoryEvent>,
) {
    // If player pressed space and they're in depositing range
    if kb.just_pressed(KeyCode::Space) && can_deposit.0 {
        deposit_events.send(DepositInventoryEvent);
    }
}

// This system grabs the mouse when the left mouse button is pressed
// and releases it when the escape key is pressed
pub fn grab_mouse(
    mut windows: Query<&mut Window>,
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    let mut window = windows.single_mut();

    if mouse.just_pressed(MouseButton::Left) {
        window.cursor_options.visible = false;
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor_options.visible = true;
        window.cursor_options.grab_mode = CursorGrabMode::None;
    }
}
