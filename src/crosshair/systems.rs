use bevy::{prelude::*, transform};
use bevy_prototype_lyon::prelude::*;

use crate::player::components::Player;
use crate::player_input::resources::MouseWorldPosition;

use super::components::Crosshair;

/// Spawns a Bundle for the mouse crosshair.
/// This provides the user feedback on where their mouse is relative to their ship.
pub fn spawn_crosshair(mut commands: Commands) {
    let line = shapes::Line(Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0));

    let _crosshair = commands
        .spawn((
            Crosshair {},
            ShapeBundle {
                path: GeometryBuilder::build_as(&line),
                spatial: SpatialBundle {
                    transform: Transform {
                        translation: Vec3 {
                            x: 0.0,
                            y: 0.0,
                            z: 1.0,
                        },
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
            Fill::color(Color::rgba(1.0, 1.0, 1.0, 1.0)),
            Stroke::new(Color::rgba(1.0, 1.0, 1.0, 0.33), 1.2),
            Name::new("Crosshair"),
        ))
        .id();
}

pub fn draw_crosshair(
    mouse_position: Res<MouseWorldPosition>,
    player_query: Query<(&Player, &Transform), Without<Crosshair>>,
    mut crosshair_query: Query<&mut Path, With<Crosshair>>,
) {
    let world_pos = mouse_position.0;
    let (_player, player_trans) = player_query.single();
    let mut path = crosshair_query.single_mut();

    // Draw Crosshair
    {
        let line = shapes::Line(player_trans.translation.truncate(), world_pos);
        *path = ShapePath::build_as(&line);
    }
}

#[derive(Component)]
pub struct MousePointer;

const POINTER_SIZE: f32 = 10.0;

pub fn spawn_pointer(mut commands: Commands) {
    let N = shapes::Line(
        Vec2::ZERO,
        Vec2 {
            x: 0.0,
            y: POINTER_SIZE,
        },
    );
    let S = shapes::Line(
        Vec2::ZERO,
        Vec2 {
            x: 0.0,
            y: -POINTER_SIZE,
        },
    );
    let E = shapes::Line(
        Vec2::ZERO,
        Vec2 {
            x: POINTER_SIZE,
            y: 0.0,
        },
    );
    let W = shapes::Line(
        Vec2::ZERO,
        Vec2 {
            x: -POINTER_SIZE,
            y: 0.0,
        },
    );

    let geometry = GeometryBuilder::new().add(&N).add(&S).add(&E).add(&W);

    let _pointer = commands
        .spawn((
            MousePointer {},
            ShapeBundle {
                path: geometry.build(),
                spatial: SpatialBundle {
                    transform: Transform {
                        translation: Vec3 {
                            x: 0.0,
                            y: 0.0,
                            z: 1.0,
                        },
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
            Fill::color(Color::rgba(1.0, 1.0, 1.0, 0.45)),
            Stroke::new(Color::rgba(1.0, 1.0, 1.0, 1.0), 1.5),
            Name::new("MousePointer"),
        ))
        .id();
}

pub fn update_pointer(
    mouse_position: Res<MouseWorldPosition>,
    mut pointer_query: Query<&mut Transform, With<MousePointer>>,
) {
    let world_pos = mouse_position.0;
    let mut transform = pointer_query.single_mut();

    transform.translation.x = world_pos.x;
    transform.translation.y = world_pos.y;
}
