// #![feature(array_methods)]

use bevy_debug_text_overlay::{screen_print, OverlayPlugin};

use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_particle_systems::ParticleSystemPlugin;
use bevy_prototype_lyon::prelude::*;
use bevy_xpbd_2d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crosshair::CrosshairPlugin;
use engine::EnginePlugin;
use factory::FactoryPlugin;
use game_ui::GameUIPlugin;
use inventory::InventoryPlugin;
use laser::LaserPlugin;
use particles::ParticlePlugin;
use player::{Player, PlayerPlugin};
use player_input::PlayerInputPlugin;
// use projectile::ProjectilePlugin;
use astroid_plugin::AstroidPlugin;
use base_station::BaseStationPlugin;
use hexbase::HexBasePlugin;
use refinery::RefineryPlugin;

mod astroid;
mod astroid_composition;
mod astroid_material;
mod astroid_plugin;
mod astroid_size;
mod base_station;
mod battery;
mod crosshair;
mod engine;
mod events;
mod factory;
mod game_ui;
mod health;
mod hexbase;
mod inventory;
mod item_producer;
mod laser;
mod particles;
mod player;
mod player_input;
mod projectile;
mod recipe;
mod refinery;
mod upgrades;

// Defines the amount of time that should elapse between each physics step.
// const TIME_STEP: f32 = 1.0 / 60.0;

pub const PIXELS_PER_METER: f32 = 10.0;

// const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);

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
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Geometry Wars"),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            // WorldInspectorPlugin::new(),

            OverlayPlugin {
                font_size: 24.0,
                ..default()
            },
            ShapePlugin,
            ParticleSystemPlugin::default(),
            PhysicsPlugins::default(),
            // Enables debug rendering
            PhysicsDebugPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
        ))
        .add_plugins((
            PlayerPlugin,
            EnginePlugin,
            PlayerInputPlugin,
            InventoryPlugin,
            BaseStationPlugin,
            RefineryPlugin,
            FactoryPlugin,
            AstroidPlugin,
            LaserPlugin,
            CrosshairPlugin,
            GameUIPlugin,
            ParticlePlugin,
            HexBasePlugin,
        ))
        .insert_resource(Gravity::ZERO)
        .add_systems(Startup, (
            setup,
        ))
        .add_systems(Update, (
            camera_follows_player,
            screen_print_debug_text
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    // mut rapier_config: ResMut<RapierConfiguration>
) {
    commands.spawn((
        GameCamera,
        Camera2dBundle::default(),
        Name::new("GameCamera"),
    ));

    // rapier_config.gravity = Vec2::new(0.0, 0.0);
}

fn camera_follows_player(
    mut camera_query: Query<(&Camera, &mut Transform), With<GameCamera>>,
    player_query: Query<&Transform, (With<Player>, Without<GameCamera>)>,
) {
    let (_camera, mut camera_trans) = camera_query.single_mut();
    let player_trans = player_query.single();

    camera_trans.translation.x = player_trans.translation.x;
    camera_trans.translation.y = player_trans.translation.y;
}

fn screen_print_debug_text(diagnostics: Res<DiagnosticsStore>) {
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(average) = fps.average() {
            screen_print!(col: Color::WHITE, "fps: {average}");
        }
    }
}
