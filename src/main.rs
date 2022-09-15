use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

// Defines the amount of time that should elapse between each physics step.
const TIME_STEP: f32 = 1.0 / 60.0;

const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);
const PLAYER_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);

const PLAYER_SIZE: Vec3 = Vec3::new(100.0, 100.0, 0.0);

#[derive(Component)]
struct Player {
    dX: f32,
    dY: f32
}

impl Player {
    fn new() -> Player {
        Player { dX: 0.0, dY: 0.0 }
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
        feature: shapes::RegularPolygonFeature::Radius(100.0),
        ..shapes::RegularPolygon::default()
    };

    commands.spawn()
        .insert(Player::new())
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

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Player, &mut Transform)>
) {
    const ACCELERATION: f32 = 1.0;
    const DECLERATION: f32 = 0.75;
    const MAX_VELOCITY: f32 = 16.0;

    let (mut player, mut trans) = player_query.single_mut();

    if keyboard_input.pressed(KeyCode::Left) {
        player.dX -= ACCELERATION;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        player.dX += ACCELERATION;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        player.dY += ACCELERATION;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        player.dY -= ACCELERATION;
    }

    player.dX = player.dX.clamp(-MAX_VELOCITY, MAX_VELOCITY);
    player.dY = player.dY.clamp(-MAX_VELOCITY, MAX_VELOCITY);

    trans.translation.x += player.dX;
    trans.translation.y += player.dY;

    trans.translation.x = trans.translation.x.clamp(-320.0, 320.0);
    trans.translation.y = trans.translation.y.clamp(-320.0, 320.0);

    // Decelerate
    player.dX *= DECLERATION;
    player.dY *= DECLERATION;
}


