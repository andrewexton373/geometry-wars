use bevy::math::vec2;
use bevy::{prelude::*, transform};
use bevy::render::camera::RenderTarget;
use bevy_prototype_lyon::prelude::*;
use bevy_prototype_lyon::shapes::Line;
use bevy_inspector_egui::{InspectorPlugin, Inspectable, RegisterInspectable};
use bevy_inspector_egui::WorldInspectorPlugin;
use rand::Rng;

// Defines the amount of time that should elapse between each physics step.
const TIME_STEP: f32 = 1.0 / 60.0;

const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);
const PLAYER_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);

const PLAYER_SIZE: Vec3 = Vec3::new(100.0, 100.0, 0.0);

const PI: f32 = 3.14159;
const TWO_PI: f32 = 2.0 * PI;


#[derive(Component, Inspectable)]
struct Player {
    delta_x: f32,
    delta_y: f32,
    delta_rotation: f32,
    hitbox: HitboxCircle
}

impl Player {
    fn new() -> Player {
        Player { 
            delta_x: 0.0,
            delta_y: 0.0,
            delta_rotation: 0.0,
            hitbox: HitboxCircle { radius: 5.0 }
        }
    }
}

#[derive(Component, Inspectable)]
struct HitboxCircle {
    radius: f32
}

#[derive(Component)]
struct Projectile {
    velocity: Vec2,
    timer: Timer,
    hitbox: HitboxCircle
}

#[derive(Component)]
struct Crosshair {}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .register_inspectable::<Player>()
        .add_plugin(ShapePlugin)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_system(player_movement)
        .add_system(camera_follows_player)
        .add_system(player_fire_weapon)
        .add_system(projectile_movement)
        .add_system(projectile_collision_check)
        .add_startup_system(spawn_astroids)
        .add_system(astroid_movement)
        .run();
}

fn setup(mut commands: Commands) {
    let camera = commands.spawn_bundle(Camera2dBundle::default()).id();

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
        ));

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

fn camera_follows_player(
    mut camera_query: Query<(&Camera, &mut Transform), (With<Camera>)>,
    mut player_query: Query<(&Transform), (With<Player>, Without<Camera>)>,
){
    let (camera, mut camera_trans) = camera_query.single_mut().into();
    let (player_trans) = player_query.single();

        // TODO: Set Camera X,Y to Player X,Y
        let player_to_camera = camera_trans.translation - player_trans.translation;
        camera_trans.translation -= player_to_camera;
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
        let ship_angle_difference = Vec2::angle_between(player_to_mouse, (player_trans.rotation * Vec3::Y).truncate());

        //Rotate towards position mouse is on
        if ship_angle_difference > 0.0 {
            player.delta_rotation += SPIN_ACCELERATION * (TWO_PI - ship_angle_difference.abs());
        } else

        if ship_angle_difference < 0.0 {
            player.delta_rotation -= SPIN_ACCELERATION * (TWO_PI - ship_angle_difference.abs());
        }

        // Draw Crosshair
        {
            let line = shapes::Line (
                player_trans.translation.truncate(),
                world_pos,
            );
    
            *path = ShapePath::build_as(&line);
        }
        
        // eprintln!("World coords: {}/{}", world_pos.x, world_pos.y);
    }

    player.delta_x = player.delta_x.clamp(-MAX_VELOCITY, MAX_VELOCITY);
    player.delta_y = player.delta_y.clamp(-MAX_VELOCITY, MAX_VELOCITY);
    player.delta_rotation = player.delta_rotation.clamp(-MAX_VELOCITY, MAX_VELOCITY);

    player_trans.translation.x += player.delta_x;
    player_trans.translation.y += player.delta_y;

    // player_trans.translation.x = player_trans.translation.x.clamp(-320.0, 320.0);
    // player_trans.translation.y = player_trans.translation.y.clamp(-320.0, 320.0);

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
                timer: Timer::from_seconds(5.0, false),
                hitbox: HitboxCircle { radius: 2.0 }
            })
            .insert_bundle(GeometryBuilder::build_as(
                &player_shape,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::DARK_GRAY),
                    outline_mode: StrokeMode::new(Color::RED, 1.0),
                },
                transform.clone(),
            ))
            .insert(Collider);
    }
}

fn projectile_collision_check(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Projectile, &Transform), With<Projectile>>,
    collider_query: Query<(Entity, &Transform, Option<&Astroid>), With<Collider>>
){
    let mut rng = rand::thread_rng();

    for (projectile_ent, projectile, projectile_transform) in projectile_query.iter() {
        for (ent, ent_transform, maybe_astroid) in &collider_query {

            match maybe_astroid {
                Some(astroid) => {
                    if Vec2::distance(
                        projectile_transform.translation.truncate(),
                        ent_transform.translation.truncate())
                         < projectile.hitbox.radius + astroid.hitbox.radius
                    {
                        let split_angle = rng.gen_range(0.0..PI/4.0);
                        
                        let right_velocity = projectile.velocity.rotate(Vec2::from_angle(split_angle)) * 0.5;
                        let left_velocity = projectile.velocity.rotate(Vec2::from_angle(-split_angle)) * 0.5;

                        match &astroid.size {
                            AstroidSize::Small => {
                            },
                            AstroidSize::Medium => {
                                spawn_astroid(&mut commands, AstroidSize::Small, right_velocity, ent_transform.translation.truncate());
                                spawn_astroid(&mut commands, AstroidSize::Small, left_velocity, ent_transform.translation.truncate());

                            },
                            AstroidSize::Large => {
                                spawn_astroid(&mut commands, AstroidSize::Medium, right_velocity,ent_transform.translation.truncate());
                                spawn_astroid(&mut commands, AstroidSize::Medium, left_velocity, ent_transform.translation.truncate());
                            }
                        }

                        commands.entity(projectile_ent).despawn_recursive();
                        commands.entity(ent).despawn_recursive();
                        return;
                    }
                },
                None => {

                }
            }
        }
    }

}



#[derive(Component)]
struct Collider;

#[derive(Component)]
struct Astroid {
    velocity: Vec2,
    size: AstroidSize,
    health: Health,
    hitbox: HitboxCircle
    // TODO: upon destruction, astroid should split into smaller asteroids
}

#[derive(Clone, Copy)]
enum AstroidSize {
    Small,
    Medium,
    Large
}

impl AstroidSize {
    fn radius(self) -> f32 {
        match self {
            Self::Small => 8.0,
            Self::Medium => 14.0,
            Self::Large => 20.0
        }
    }
}

#[derive(Component)]
struct Health {
    current_health: f32,
    full_health: f32
}

fn spawn_astroids(
    mut commands: Commands
){
    let mut rng = rand::thread_rng();

    for i in 0..15 {
        let random_postion = Vec2 {x: rng.gen_range(-300.0..300.0), y: rng.gen_range(-300.0..300.0)};
        spawn_astroid(&mut commands, AstroidSize::Large, Vec2 { x: 0.0, y: 0.0 }, random_postion);
    }

}

fn spawn_astroid(
    commands: &mut Commands,
    size: AstroidSize,
    velocity: Vec2,
    position: Vec2
) {

    let astroid_shape: shapes::RegularPolygon;
    match size {
        AstroidSize::Small => {
            astroid_shape = shapes::RegularPolygon {
                sides: 3,
                ..shapes::RegularPolygon::default()
            };
        },
        AstroidSize::Medium => {
            astroid_shape = shapes::RegularPolygon {
                sides: 4,
                ..shapes::RegularPolygon::default()
            };
        },
        AstroidSize::Large => {
            astroid_shape = shapes::RegularPolygon {
                sides: 7,
                ..shapes::RegularPolygon::default()
            };
        }
    }

    commands.spawn()
        .insert(Astroid {
            velocity: velocity,
            size: size,
            health: Health {current_health: 50.0, full_health: 100.0},
            hitbox: HitboxCircle { radius: size.radius() }
        })
        .insert_bundle(GeometryBuilder::build_as(
            &astroid_shape,
            DrawMode::Fill(FillMode::color(Color::RED)),
            // DrawMode::Outlined {
            //     fill_mode: FillMode::color(Color::DARK_GRAY),
            //     outline_mode: StrokeMode::new(Color::WHITE, 1.0),
            // },
            Transform {
                translation: position.extend(0.0),
                scale: Vec3::new(size.radius(), size.radius(), 0.0),
                ..default()
            }
        ))
        .insert(Collider);
        
}

fn astroid_movement(
    mut astroid_query: Query<(&mut Astroid,&mut Transform)>,
    time: Res<Time>
){

    for (mut astroid, mut transform) in astroid_query.iter_mut() {
        transform.translation.x += astroid.velocity.x;
        transform.translation.y += astroid.velocity.y;

        // projectile.timer.tick(time.delta());
    }
}