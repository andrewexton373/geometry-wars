use bevy::color::palettes::css::WHITE;
use bevy::prelude::*;

use crate::player::components::Player;
use crate::player_input::resources::MouseWorldPosition;

use super::components::Crosshair;

// fn generate_crosshair(line: &shapes::Line) -> Shape {
//     ShapeBuilder::with(line)
//                 .fill(Color::srgba(1.0, 1.0, 1.0, 1.0))
//                 .stroke((Color::srgba(1.0, 1.0, 1.0, 0.33), 1.2))
//                 .build()
// }

/// Spawns a Bundle for the mouse crosshair.
/// This provides the user feedback on where their mouse is relative to their ship.
// pub fn spawn_crosshair(mut commands: Commands) {
//     let line = shapes::Line(Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0));

//     let _crosshair = commands
//         .spawn((
//             Crosshair {},
//             generate_crosshair(&line),
//             Transform::from_xyz(0.0, 0.0, 1.0),
//             Name::new("Crosshair"),
//         ))
//         .id();
// }

pub fn draw_crosshair(
    mouse_position: Res<MouseWorldPosition>,
    player_query: Query<(&Player, &Transform), Without<Crosshair>>,
    // mut crosshair_query: Query<&mut Shape, With<Crosshair>>,
    mut gizmos: Gizmos,
) {
    let (_player, player_trans) = player_query.single();

    // Player to Mouse
    gizmos.line_2d(player_trans.translation.truncate(), mouse_position.0, Color::from(WHITE));

    // Crosshair
    gizmos.line_2d(Vec2::new(-100.0, 0.0) + mouse_position.0, Vec2::new(100.0, 0.0) + mouse_position.0, Color::from(WHITE));
    gizmos.line_2d(Vec2::new(-0.0, -100.0) + mouse_position.0, Vec2::new(0.0, 100.0) + mouse_position.0, Color::from(WHITE));
}

#[derive(Component)]
pub struct MousePointer;

const POINTER_SIZE: f32 = 10.0;

// fn generate_pointer() -> Shape {
//     let N = shapes::Line(
//         Vec2::ZERO,
//         Vec2 {
//             x: 0.0,
//             y: POINTER_SIZE,
//         },
//     );
//     let S = shapes::Line(
//         Vec2::ZERO,
//         Vec2 {
//             x: 0.0,
//             y: -POINTER_SIZE,
//         },
//     );
//     let E = shapes::Line(
//         Vec2::ZERO,
//         Vec2 {
//             x: POINTER_SIZE,
//             y: 0.0,
//         },
//     );
//     let W = shapes::Line(
//         Vec2::ZERO,
//         Vec2 {
//             x: -POINTER_SIZE,
//             y: 0.0,
//         },
//     );

//     ShapeBuilder::new().add(&N).add(&S).add(&E).add(&W).fill(Color::rgba(1.0, 1.0, 1.0, 0.45)).stroke((Stroke::new(Color::rgba(1.0, 1.0, 1.0, 1.0), 1.5))).build()
// }

// pub fn spawn_pointer(mut commands: Commands) {

//     commands
//         .spawn((
//             MousePointer {},
//             // generate_pointer(),
//             Transform::from_xyz(0.0, 0.0, 1.0),
//             Name::new("MousePointer"),
//         ));
// }

// pub fn update_pointer(
//     mouse_position: Res<MouseWorldPosition>,
//     mut pointer_query: Query<&mut Transform, With<MousePointer>>,
// ) {
//     let world_pos = mouse_position.0;
//     let mut transform = pointer_query.single_mut();

//     transform.translation.x = world_pos.x;
//     transform.translation.y = world_pos.y;
// }
