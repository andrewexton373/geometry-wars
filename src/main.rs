use bevy::math::vec2;
use bevy_stat_bars::*;
use bevy::{prelude::*, transform};
use bevy_prototype_lyon::prelude::*;
use bevy_inspector_egui::{InspectorPlugin, Inspectable, RegisterInspectable};
use bevy_inspector_egui::WorldInspectorPlugin;

mod player;
use player::{ PlayerPlugin, Player };

mod astroid;
use astroid::{AstroidPlugin};

mod projectile;
use projectile::{ProjectilePlugin};

mod crosshair;
use crosshair::CrosshairPlugin;

mod healthbar;
use healthbar::HealthBarPlugin;

// Defines the amount of time that should elapse between each physics step.
const TIME_STEP: f32 = 1.0 / 60.0;

const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);

#[derive(Component, Inspectable, Reflect, Default)]
#[reflect(Component)]
pub struct HitboxCircle {
    pub radius: f32
}

#[derive(Component)]
struct Collider;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(ShapePlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(AstroidPlugin)
        .add_plugin(ProjectilePlugin)
        .add_plugin(CrosshairPlugin)
        .add_plugin(StatBarsPlugin)
        .add_plugin(HealthBarPlugin)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_system(camera_follows_player)
        .run();
}

fn setup(mut commands: Commands) {
    let camera = commands.spawn_bundle(Camera2dBundle::default()).id();
    HealthBarPlugin::attach_player_health_bar(&mut commands, camera);
}

fn camera_follows_player(
    mut camera_query: Query<(&Camera, &mut GlobalTransform), (With<Camera>)>,
    mut player_query: Query<(&Transform), (With<Player>, Without<Camera>)>,
){
    let (camera, mut camera_trans) = camera_query.single_mut().into();
    let (player_trans) = player_query.single();

        // TODO: seems sloppy, is there another way?
        let player_to_camera = camera_trans.translation() - player_trans.translation;
        let mut_trans = camera_trans.translation_mut();
        mut_trans.x -= player_to_camera.x;
        mut_trans.y -= player_to_camera.y;
}
