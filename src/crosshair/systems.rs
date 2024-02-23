use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use super::components::Crosshair;
use crate::player::components::Player;
use crate::player_input::MouseWorldPosition;

pub fn spawn_crosshair(mut commands: Commands) {
    let line = shapes::Line(Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0));

    let _crosshair = commands
        .spawn((
            Crosshair {},
            ShapeBundle {
                path: GeometryBuilder::build_as(&line),
                ..default()
            },
            Fill::color(Color::rgba(1.0, 1.0, 1.0, 0.45)),
            Stroke::new(Color::rgba(1.0, 1.0, 1.0, 0.1), 1.2),
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
