pub(crate) mod asteroid;
pub(crate) mod battery;
pub(crate) mod collectible;
pub(crate) mod crosshair;
pub(crate) mod engine;
pub(crate) mod events;
pub(crate) mod factory;
pub(crate) mod health;
pub(crate) mod hexbase;
pub(crate) mod inventory;
pub(crate) mod item_producer;
pub(crate) mod laser;
pub(crate) mod particles;
pub(crate) mod player;
pub(crate) mod player_input;
pub(crate) mod projectile;
pub(crate) mod recipe;
pub(crate) mod refinery;
pub(crate) mod space_station;
pub(crate) mod ui;
pub(crate) mod upgrades;
pub(crate) mod items;

// #![feature(array_methods)]

use bevy_debug_text_overlay::{screen_print, OverlayPlugin};

use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_particle_systems::ParticleSystemPlugin;
use bevy_prototype_lyon::prelude::*;
use bevy_xpbd_2d::prelude::*;

use engine::EnginePlugin;
use factory::FactoryPlugin;
use inventory::plugin::InventoryPlugin;
use particles::plugin::ParticlePlugin;
use player::{components::Player, plugin::PlayerPlugin};
use player_input::plugin::PlayerInputPlugin;
use ui::plugin::GameUIPlugin;
// use projectile::ProjectilePlugin;
use crate::crosshair::plugin::CrosshairPlugin;
use crate::laser::plugin::LaserPlugin;
use asteroid::plugin::AsteroidPlugin;
use hexbase::HexBasePlugin;
use refinery::RefineryPlugin;
use space_station::plugin::SpaceStationPlugin;

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
            SpaceStationPlugin,
            RefineryPlugin,
            FactoryPlugin,
            AsteroidPlugin,
            LaserPlugin,
            CrosshairPlugin,
            GameUIPlugin,
            ParticlePlugin,
            HexBasePlugin,
        ))
        .insert_resource(Gravity::ZERO)
        .add_systems(Startup, (setup,))
        .add_systems(Update, (camera_follows_player, screen_print_debug_text))
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
