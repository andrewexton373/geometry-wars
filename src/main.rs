pub(crate) mod ai;
pub(crate) mod asteroid;
pub(crate) mod background;
pub(crate) mod battery;
pub(crate) mod camera;
pub(crate) mod collectible;
pub(crate) mod crosshair;
pub(crate) mod events;
pub(crate) mod factory;
pub(crate) mod health;
pub(crate) mod hexgrid;
pub(crate) mod inventory;
pub(crate) mod item_producer;
pub(crate) mod items;
pub(crate) mod laser;
pub(crate) mod particles;
pub(crate) mod player;
pub(crate) mod player_input;
pub(crate) mod projectile;
pub(crate) mod rcs;
pub(crate) mod recipe;
pub(crate) mod refinery;
pub(crate) mod space_station;
pub(crate) mod ui;
pub(crate) mod upgrades;

// #![feature(array_methods)]

use ai::plugin::AiPlugin;
use background::plugin::BackgroundPlugin;
// use bevy_debug_text_overlay::{screen_print, OverlayPlugin};

use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, prelude::*
};
// use bevy_particle_systems::ParticleSystemPlugin;
use bevy_prototype_lyon::prelude::*;
use avian2d::prelude::*;

use camera::plugin::GameCameraPlugin;
use factory::FactoryPlugin;
use inventory::plugin::InventoryPlugin;
use particles::plugin::ParticlePlugin;
use player::plugin::PlayerPlugin;
use player_input::plugin::PlayerInputPlugin;
use projectile::plugin::ProjectilePlugin;
use rcs::plugin::RCSPlugin;
use ui::plugin::GameUIPlugin;
// use projectile::ProjectilePlugin;
use crate::crosshair::plugin::CrosshairPlugin;
use crate::laser::plugin::LaserPlugin;
use asteroid::plugin::AsteroidPlugin;
use battery::plugin::BatteryPlugin;
use health::plugin::HealthPlugin;
use hexgrid::plugin::HexBasePlugin;
use refinery::RefineryPlugin;
use space_station::plugin::SpaceStationPlugin;
use upgrades::plugin::UpgradesPlugin;

// Defines the amount of time that should elapse between each physics step.
// const TIME_STEP: f32 = 1.0 / 60.0;

pub const PIXELS_PER_METER: f64 = 10.0;

// const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);

pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const HEIGHT: f32 = 800.0;
pub const WIDTH: f32 = HEIGHT * RESOLUTION;

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum AppState {
    #[default]
    InGame,
    Paused,
    BuildMode,
}

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
            ShapePlugin,
            // ParticleSystemPlugin,
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
         ))
        .add_plugins((
            HexBasePlugin,
            PlayerPlugin,
            UpgradesPlugin,
            RCSPlugin,
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
            GameCameraPlugin,
        ))
        .add_plugins((
            HealthPlugin,
            BatteryPlugin,
            AiPlugin,
            ProjectilePlugin,
            BackgroundPlugin,
        ))
        .insert_resource(Gravity::ZERO)
        .init_state::<AppState>()
        .run();
}

fn screen_print_debug_text(diagnostics: Res<DiagnosticsStore>) {
    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(average) = fps.average() {
            // screen_print!(col: bevy_render::color::Color::WHITE, "fps: {average}");

        }
    }
}
