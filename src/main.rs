use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

// Defines the amount of time that should elapse between each physics step.
const TIME_STEP: f32 = 1.0 / 60.0;

const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);
const PLAYER_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);

const PLAYER_SIZE: Vec3 = Vec3::new(100.0, 100.0, 0.0);

#[derive(Component)]
struct Player {
    position: Position,
    rotation: Rotation
}

#[derive(Component)]
struct Position {
    x: f32,
    y: f32
}

#[derive(Component)]
struct Rotation {
    angle: f32
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_system(movement)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());

    let player_shape = shapes::RegularPolygon {
        sides: 3,
        feature: shapes::RegularPolygonFeature::Radius(100.0),
        ..shapes::RegularPolygon::default()
    };

    commands.spawn()
        .insert(Player {
            position: Position { x: 0.0, y: 0.0 },
            rotation: Rotation { angle: 0.0 }
        })
        .insert_bundle(GeometryBuilder::build_as(
            &player_shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::CYAN),
                outline_mode: StrokeMode::new(Color::WHITE, 10.0),
            },
            Transform::default(),
        ));
        // .insert(Collider);

        
    
}

fn movement(mut commands: Commands) {

}

