use bevy::{prelude::*};
use bevy::render::camera::RenderTarget;
use bevy_prototype_lyon::prelude::*;
use bevy_prototype_lyon::shapes::Line;

// Defines the amount of time that should elapse between each physics step.
const TIME_STEP: f32 = 1.0 / 60.0;

const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);
const PLAYER_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);

const PLAYER_SIZE: Vec3 = Vec3::new(100.0, 100.0, 0.0);

const PI: f32 = 3.14159;
const TWO_PI: f32 = 2.0 * PI;


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

#[derive(Component)]
struct Crosshair {}

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

    let mut player = commands.spawn()
        .insert(Player::new())
        .insert_bundle(GeometryBuilder::build_as(
            &player_shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::CYAN),
                outline_mode: StrokeMode::new(Color::WHITE, 2.0),
            },
            Transform {
                scale: Vec3::new(0.5, 1.0, 1.0),
                ..Default::default()
            }
            // Transform::default(),
        )).id();
        // .insert(Collider);
    
    let line = shapes::Line(
        Vec2::new(0.0, 0.0),
        Vec2::new(0.0, 0.0)
    );

    let crosshair = commands.spawn()
        .insert(Crosshair {})
        .insert_bundle(GeometryBuilder::build_as(
            &line,
        DrawMode::Outlined {
                fill_mode: FillMode::color(Color::rgba(1.0, 1.0, 1.0, 0.45)),
                outline_mode: StrokeMode::new(Color::rgba(1.0, 1.0, 1.0, 0.1), 1.2),
            },
            Transform {
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..Default::default()
            }
        )).id();

}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut player_query: Query<(&mut Player, &mut Transform), Without<Crosshair>>,
    mut crosshair_query: Query<(&mut Crosshair, &mut Path, &mut Transform)>
) {

    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    let (crosshair, mut path, mut crosshair_transform) = crosshair_query.single_mut();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

    const ACCELERATION: f32 = 0.2;
    const DECLERATION: f32 = 0.95;
    const SPIN_ACCELERATION: f32 = 0.4;
    const SPIN_DECELERATION: f32 = 0.1;
    const MAX_VELOCITY: f32 = 6.0;

    let (mut player, mut player_trans) = player_query.single_mut();

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

    // TODO: Rotate towards position mouse is on
    // if keyboard_input.pressed(KeyCode::Q){
    //     player.delta_rotation += SPIN_ACCELERATION;
    // }

    // if keyboard_input.pressed(KeyCode::E){
    //     player.delta_rotation -= SPIN_ACCELERATION;
    // }

     // check if the cursor is inside the window and get its position
     if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        let player_to_mouse = Vec2::new(player_trans.translation.x, player_trans.translation.y) - world_pos;
        eprintln!("World coords: {}/{}", player_to_mouse.x, player_to_mouse.y);

        let line = shapes::Line (
            player_trans.translation.truncate(),
            world_pos,
            // trans.translation.truncate(),
        );

        *path = ShapePath::build_as(&line);

        // TODO: Update Crosshair
        // crosshair_transform.scale = -player_to_mouse.extend(1.0);

        let ship_angle_difference = Vec2::angle_between(player_to_mouse, (player_trans.rotation * Vec3::Y).truncate());

        //Rotate towards position mouse is on
        if ship_angle_difference > 0.0 {
            player.delta_rotation += SPIN_ACCELERATION * (TWO_PI - ship_angle_difference.abs());
        } else

        if ship_angle_difference < 0.0 {
            player.delta_rotation -= SPIN_ACCELERATION * (TWO_PI - ship_angle_difference.abs());
        }

        // eprintln!("World coords: {}/{}", world_pos.x, world_pos.y);
    }



    player.delta_x = player.delta_x.clamp(-MAX_VELOCITY, MAX_VELOCITY);
    player.delta_y = player.delta_y.clamp(-MAX_VELOCITY, MAX_VELOCITY);
    player.delta_rotation = player.delta_rotation.clamp(-MAX_VELOCITY, MAX_VELOCITY);

    player_trans.translation.x += player.delta_x;
    player_trans.translation.y += player.delta_y;

    player_trans.translation.x = player_trans.translation.x.clamp(-320.0, 320.0);
    player_trans.translation.y = player_trans.translation.y.clamp(-320.0, 320.0);

    player_trans.rotate_z(player.delta_rotation.to_radians());

    // Decelerate
    player.delta_x *= DECLERATION;
    player.delta_y *= DECLERATION;
    player.delta_rotation *= SPIN_DECELERATION;

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
    keyboard_input: Res<Input<MouseButton>>,
    player_query: Query<(&mut Player, &mut Transform)>
)
{
    const BULLET_SPEED: f32 = 4.0;
    let (player, transform) = player_query.single();

    // why does this work? https://www.reddit.com/r/rust_gamedev/comments/rphgsf/calculating_bullet_x_and_y_position_based_off_of/
    let velocity = ((transform.rotation * Vec3::Y) * BULLET_SPEED) + Vec3::new(player.delta_x, player.delta_y, 0.0);

    // should be just pressed, but it's fun with keyboard_input.pressed()
    if keyboard_input.just_pressed(MouseButton::Left) {
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


