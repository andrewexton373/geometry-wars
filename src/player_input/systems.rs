use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};

use crate::camera::components::{CameraTarget, GameCamera};
use crate::player::components::Player;
use crate::rcs::events::{RCSThrustPowerEvent, RCSThrustVectorEvent};
use crate::space_station::resources::CanDeposit;
use crate::ui::mouse_hover_context::resources::MouseHoverContext;

use super::events::DepositInventoryEvent;
use super::resources::{MouseScreenPosition, MouseWorldPosition};

pub fn update_mouse_world_position_resource(
    mut mouse_position: ResMut<MouseWorldPosition>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    _cursor_event: MessageReader<CursorMoved>,
    q_camera: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
) {
    let window = window_query.single().expect("No Window");

    let (camera, camera_transform) = q_camera.single().expect("No Camera");

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
    _cursor_event: MessageReader<CursorMoved>,
) {
    let window = window_query.single().expect("No Window");
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
        let player_ent = player_q.single_mut().expect("No Player");
        commands.entity(player_ent).insert(CameraTarget);
    }
}

pub fn scroll_events(
    mut scroll_events: MessageReader<MouseWheel>,
    mut engine_events: MessageWriter<RCSThrustPowerEvent>,
) {
    use bevy::input::mouse::MouseScrollUnit;

    for event in scroll_events.read() {
        match event.unit {
            MouseScrollUnit::Line => {
                engine_events.write(RCSThrustPowerEvent(event.y));
            }
            MouseScrollUnit::Pixel => {
                engine_events.write(RCSThrustPowerEvent(event.y));
            }
        }
    }
}

/// Allow the player to use , and . to zoom the viewport in and out.
pub fn player_camera_control(
    kb: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut projection: Single<&mut Projection, With<Camera2d>>,
) {
    let dist = 0.75 * time.delta().as_secs_f32();

    if let Projection::Orthographic(projection) = &mut **projection {
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

pub fn player_movement_input_handling(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<Entity, With<Player>>,
) {
    let entity = player_query.single_mut().expect("No Player");

    let mut thrust_vector: Vec2 = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
        thrust_vector += -Vec2::X;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
        thrust_vector += Vec2::X;
    }

    if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
        thrust_vector += Vec2::Y;
    }

    if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
        thrust_vector += -Vec2::Y;
    }

    if thrust_vector != Vec2::ZERO {
        commands.write_message(RCSThrustVectorEvent {
            entity,
            thrust_vector,
        });
    }
}

// TODO: Idea?
// Mark InventoryItems with Deposit Component on Event
// Use this system to deposit marked inventory items in Base Station
pub fn player_deposit_control(
    mut commands: Commands,
    kb: Res<ButtonInput<KeyCode>>,
    can_deposit: Res<CanDeposit>,
) {
    // If player pressed space and they're in depositing range
    if kb.just_pressed(KeyCode::Space) && can_deposit.0 {
        commands.write_message(DepositInventoryEvent);
    }
}

// This system grabs the mouse when the left mouse button is pressed
// and releases it when the escape key is pressed
pub fn grab_mouse(
    mut cursor_options: Single<&mut CursorOptions, With<Window>>,
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    // let mut window = windows.single_mut().expect("No Window");

    if mouse.just_pressed(MouseButton::Left) {
        cursor_options.visible = false;
        cursor_options.grab_mode = CursorGrabMode::Locked;
    }

    if key.just_pressed(KeyCode::Escape) {
        cursor_options.visible = true;
        cursor_options.grab_mode = CursorGrabMode::None;
    }
}
