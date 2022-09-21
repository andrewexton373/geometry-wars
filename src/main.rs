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

mod astroid;
use astroid::{AstroidPlugin, Astroid, AstroidSize};

mod projectile;
use projectile::{ProjectilePlugin};

mod crosshair;
use crosshair::CrosshairPlugin;

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
struct Collider;

#[derive(Component, Inspectable)]
pub struct Health {
    current_health: f32,
    full_health: f32
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(ShapePlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(AstroidPlugin)
        .add_plugin(ProjectilePlugin)
        .add_plugin(CrosshairPlugin)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_system(camera_follows_player)
        .run();
}

fn setup(mut commands: Commands) {
    let camera = commands.spawn_bundle(Camera2dBundle::default()).id();
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
