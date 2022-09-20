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

#[derive(Component)]
struct Projectile {
    velocity: Vec2,
    timer: Timer
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_system(player_movement)
        .add_system(player_fire_weapon)
        .add_system(projectile_movement)
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
    player.delta_rotation = player.delta_rotation.clamp(-MAX_VELOCITY, MAX_VELOCITY);

    trans.translation.x += player.delta_x;
    trans.translation.y += player.delta_y;

    trans.translation.x = trans.translation.x.clamp(-320.0, 320.0);
    trans.translation.y = trans.translation.y.clamp(-320.0, 320.0);

    trans.rotate_z(player.delta_rotation.to_radians());

    // Decelerate
    player.delta_x *= DECLERATION;
    player.delta_y *= DECLERATION;
    player.delta_rotation *= DECLERATION;

}

fn projectile_movement(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &mut Projectile, &mut Transform)>,
    time: Res<Time>
)
{
    for (ent, mut projectile, mut transform) in projectile_query.iter_mut() {
        transform.translation.x += projectile.velocity.x;
        transform.translation.y += projectile.velocity.y;

        projectile.timer.tick(time.delta());

        if projectile.timer.finished() {
            commands.entity(ent).despawn_recursive();
        }
    }
}

fn player_fire_weapon(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    player_query: Query<(&mut Player, &mut Transform)>
)
{
    let (player, transform) = player_query.single();

    // why does this work? https://www.reddit.com/r/rust_gamedev/comments/rphgsf/calculating_bullet_x_and_y_position_based_off_of/
    let velocity = transform.rotation * Vec3::Y;

    // should be just pressed, but it's fun with keyboard_input.pressed()
    if keyboard_input.just_pressed(KeyCode::Space) {
        let player_shape = shapes::Circle {
            ..shapes::Circle::default()
        };
    
        commands.spawn()
            .insert(Projectile {
                velocity: Vec2 { x: velocity.x, y: velocity.y },
                timer: Timer::from_seconds(5.0, false)
            })
            .insert_bundle(GeometryBuilder::build_as(
                &player_shape,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::DARK_GRAY),
                    outline_mode: StrokeMode::new(Color::RED, 1.0),
                },
                transform.clone(),
            ));
    }
}


