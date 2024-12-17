use bevy::color::palettes::css::WHITE;
use bevy::prelude::*;

use crate::player::components::Player;
use crate::player_input::resources::MouseWorldPosition;

use super::components::Crosshair;

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct CrosshairGizmos;

pub fn crosshair_gizmo_config(
    mut config_store: ResMut<GizmoConfigStore>,
) {
    let (config, _) = config_store.config_mut::<CrosshairGizmos>();
    config.line_width = 0.5;
}

const POINTER_SIZE: f32 = 20.0;

pub fn draw_crosshair(
    mouse_position: Res<MouseWorldPosition>,
    player_query: Query<&Transform, With<Player>>,
    mut gizmos: Gizmos<CrosshairGizmos>,
) {
    let player_trans = player_query.single();

    // Player to Mouse Line Segment
    gizmos.line_2d(
        player_trans.translation.truncate(),
        mouse_position.0,
        Color::from(WHITE),
    );

    // Crosshair X-Axis
    gizmos.line_2d(
        Vec2::new(-POINTER_SIZE, 0.0) + mouse_position.0,
        Vec2::new(POINTER_SIZE, 0.0) + mouse_position.0,
        Color::from(WHITE),
    );

    // Crosshair Y-Axis
    gizmos.line_2d(
        Vec2::new(-0.0, -POINTER_SIZE) + mouse_position.0,
        Vec2::new(0.0, POINTER_SIZE) + mouse_position.0,
        Color::from(WHITE),
    );
}

