use crate::player::Player;
use crate::player_input::MouseWorldPosition;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

#[derive(Component)]
pub struct Crosshair {}

pub struct CrosshairPlugin;

impl Plugin for CrosshairPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup,
            Self::spawn_crosshair
        );
        app.add_systems(Update,
            Self::draw_crosshair
        );
    }
}

impl CrosshairPlugin {
    fn spawn_crosshair(mut commands: Commands) {
        let line = shapes::Line(Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0));

        let _crosshair = commands
            .spawn((
                Crosshair {},
                ShapeBundle {
                    path: GeometryBuilder::build_as(&line),
                    transform: Transform {
                        scale: Vec3::new(1.0, 1.0, 1.0),
                        ..Default::default()
                    },
                    ..default()
                },
                Fill::color(Color::rgba(1.0, 1.0, 1.0, 0.45)),
                Stroke::new(Color::rgba(1.0, 1.0, 1.0, 0.1), 1.2),
                Name::new("Crosshair"),
            ))
            .id();
    }

    fn draw_crosshair(
        mouse_position: Res<MouseWorldPosition>,
        player_query: Query<(&Player, &Transform), Without<Crosshair>>,
        mut crosshair_query: Query<(&mut Crosshair, &mut Path)>,
    ) {
        let world_pos = mouse_position.0;
        let (_player, player_trans) = player_query.single();
        let (_crosshair, mut path) = crosshair_query.single_mut();

        // Draw Crosshair
        {
            let line = shapes::Line(player_trans.translation.truncate(), world_pos);
            *path = ShapePath::build_as(&line);
        }
    }
}
