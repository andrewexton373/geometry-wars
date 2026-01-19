use bevy::color::palettes::css::WHITE;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::camera::components::GameCamera;
use crate::player::components::Player;

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct CrosshairGizmos;

pub fn crosshair_gizmo_config(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<CrosshairGizmos>();
    config.line.width = 0.5;
}

const POINTER_SIZE: f32 = 20.0;

pub fn draw_crosshair(
    player_query: Query<&Transform, With<Player>>,
    mut gizmos: Gizmos<CrosshairGizmos>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    _cursor_event: MessageReader<CursorMoved>,
    q_camera: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
) {
    let window = window_query.single().expect("No Window");

    let (camera, camera_transform) = q_camera.single().expect("No Camera");

    if let Some(pos) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate())
    {
        let player_trans = player_query.single().expect("No Player Transform");

        // Player to Mouse Line Segment
        gizmos.line_2d(player_trans.translation.truncate(), pos, Color::from(WHITE));

        // Crosshair X-Axis
        gizmos.line_2d(
            Vec2::new(-POINTER_SIZE, 0.0) + pos,
            Vec2::new(POINTER_SIZE, 0.0) + pos,
            Color::from(WHITE),
        );

        // Crosshair Y-Axis
        gizmos.line_2d(
            Vec2::new(-0.0, -POINTER_SIZE) + pos,
            Vec2::new(0.0, POINTER_SIZE) + pos,
            Color::from(WHITE),
        );
    }
}
