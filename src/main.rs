// #![feature(array_methods)]

use bevy_debug_text_overlay::{screen_print, OverlayPlugin};

use astroid::AstroidPlugin;
use base_station::BaseStationPlugin;
use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
    window::PresentMode,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use crosshair::CrosshairPlugin;
use factory::FactoryPlugin;
use game_ui::GameUIPlugin;
use inventory::InventoryPlugin;
use particles::ParticlePlugin;
use player::{PlayerPlugin, Player};
use projectile::ProjectilePlugin;
use refinery::RefineryPlugin;

mod upgrades;
mod battery;
mod factory;
mod game_ui;
mod health;
mod item_producer;
mod particles;
mod recipe;
mod refinery;
// mod widgets;
mod player;
mod astroid;
mod projectile;
mod crosshair;
mod player_stats_bar;
mod base_station;
mod inventory;

// Defines the amount of time that should elapse between each physics step.
// const TIME_STEP: f32 = 1.0 / 60.0;

pub const PIXELS_PER_METER: f32 = 10.0;

const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);

pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const HEIGHT: f32 = 800.0;
pub const WIDTH: f32 = HEIGHT * RESOLUTION;

#[derive(Component)]
struct Collider;

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
struct FpsText;

#[derive(Component)]
pub struct GameCamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "ASTROID MINER".to_string(),
                width: HEIGHT * RESOLUTION,
                height: HEIGHT,
                present_mode: PresentMode::AutoVsync,
              ..default()
            },
            ..default()
          }))
        .add_plugin(OverlayPlugin { font_size: 32.0, ..default() })
        // .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(ShapePlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(InventoryPlugin)
        .add_plugin(BaseStationPlugin)
        .add_plugin(RefineryPlugin)
        .add_plugin(FactoryPlugin)
        .add_plugin(AstroidPlugin)
        .add_plugin(ProjectilePlugin)
        .add_plugin(CrosshairPlugin)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_system(camera_follows_player)
        // .add_plugin(PlayerStatsBarPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            PIXELS_PER_METER,
        ))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(GameUIPlugin)
        .add_plugin(ParticlePlugin)
        .add_system(screen_print_debug_text)
        .run();
}

fn setup(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(GameCamera)
        .insert(Name::new("GameCamera"));

    rapier_config.gravity = Vec2::new(0.0, 0.0);
}

fn camera_follows_player(
    mut camera_query: Query<(&Camera, &mut GlobalTransform), With<GameCamera>>,
    player_query: Query<&Transform, (With<Player>, Without<GameCamera>)>,
) {
    let (_camera, mut camera_trans) = camera_query.single_mut();
    let player_trans = player_query.single();

    // TODO: seems sloppy, is there another way?
    let player_to_camera = camera_trans.translation() - player_trans.translation;
    let mut_trans = camera_trans.translation_mut();
    mut_trans.x -= player_to_camera.x;
    mut_trans.y -= player_to_camera.y;
}

fn screen_print_debug_text(diagnostics: Res<Diagnostics>) {
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(average) = fps.average() {
            screen_print!(col: Color::WHITE, "fps: {average}");
        }
    }
}
