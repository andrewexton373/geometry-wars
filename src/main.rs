use bevy::{prelude::*, transform};
use bevy_prototype_lyon::prelude::*;

// Defines the amount of time that should elapse between each physics step.
const TIME_STEP: f32 = 1.0 / 60.0;

const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);
const PLAYER_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);

const PLAYER_SIZE: Vec3 = Vec3::new(100.0, 100.0, 0.0);


#[derive(Component)]
struct Player {
    delta_x: f32,
    delta_y: f32,
    delta_rotation: f32
}

impl Player {
    fn new() -> Player {
        Player { delta_x: 0.0, delta_y: 0.0, delta_rotation: 0.0 }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_system(player_movement)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());

    let player_shape = shapes::RegularPolygon {
        sides: 3,
        feature: shapes::RegularPolygonFeature::Radius(20.0),
        ..shapes::RegularPolygon::default()
    };

    commands.spawn()
        .insert(Player::new())
        .insert_bundle(GeometryBuilder::build_as(
            &player_shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::CYAN),
                outline_mode: StrokeMode::new(Color::WHITE, 2.0),
            },
            Transform::default(),
        ));
        // .insert(Collider);
    
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Player, &mut Transform)>
) {
    const ACCELERATION: f32 = 0.5;
    const DECLERATION: f32 = 0.95;
    const MAX_VELOCITY: f32 = 16.0;

    let (mut player, mut trans) = player_query.single_mut();

    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
        player.delta_x -= ACCELERATION;
    }

    if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
        player.delta_x += ACCELERATION;
    }

    if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
        player.delta_y += ACCELERATION;
    }

    if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
        player.delta_y -= ACCELERATION;
    }

    if keyboard_input.pressed(KeyCode::Q){
        player.delta_rotation += ACCELERATION;
    }

    if keyboard_input.pressed(KeyCode::E){
        player.delta_rotation -= ACCELERATION;
    }

    player.delta_x = player.delta_x.clamp(-MAX_VELOCITY, MAX_VELOCITY);
    player.delta_y = player.delta_y.clamp(-MAX_VELOCITY, MAX_VELOCITY);
    // maybe clamp rotation?

    trans.translation.x += player.delta_x;
    trans.translation.y += player.delta_y;

    trans.translation.x = trans.translation.x.clamp(-320.0, 320.0);
    trans.translation.y = trans.translation.y.clamp(-320.0, 320.0);

    trans.rotation.z += player.delta_rotation.to_radians();

    // Decelerate
    player.delta_x *= DECLERATION;
    player.delta_y *= DECLERATION;
    player.delta_rotation *= DECLERATION;

}


