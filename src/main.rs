use bevy::math::vec2;
use bevy::{prelude::*, transform};
use bevy::render::camera::RenderTarget;
use bevy_prototype_lyon::prelude::*;
use bevy_prototype_lyon::shapes::Line;
use bevy_inspector_egui::{InspectorPlugin, Inspectable, RegisterInspectable};
use bevy_inspector_egui::WorldInspectorPlugin;
use rand::Rng;

mod player;
use player::{ PlayerPlugin, Player };

// Defines the amount of time that should elapse between each physics step.
const TIME_STEP: f32 = 1.0 / 60.0;

const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);
const PLAYER_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);

const PLAYER_SIZE: Vec3 = Vec3::new(100.0, 100.0, 0.0);

pub const PI: f32 = 3.14159;
pub const TWO_PI: f32 = 2.0 * PI;

#[derive(Component, Inspectable)]
pub struct HitboxCircle {
    pub radius: f32
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
        .add_plugin(PlayerPlugin)
        .register_inspectable::<Player>()
        .add_plugin(ShapePlugin)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_system(camera_follows_player)
        .add_system(projectile_movement)
        .add_system(projectile_collision_check)
        .add_startup_system(spawn_astroids)
        .add_system(astroid_movement)
        .run();
}

fn setup(mut commands: Commands) {
    let camera = commands.spawn_bundle(Camera2dBundle::default()).id();
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