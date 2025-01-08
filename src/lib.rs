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

use std::default;

use avian2d::{
    prelude::{Gravity, PhysicsDebugPlugin, PhysicsLayer},
    PhysicsPlugins,
};
use bevy::prelude::*;
use bevy_hanabi::HanabiPlugin;

use ai::plugin::AiPlugin;
use background::plugin::BackgroundPlugin;

// use bevy_particle_systems::ParticleSystemPlugin;
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

pub const PIXELS_PER_METER: f64 = 10.0;

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

#[derive(Default, PhysicsLayer)]
pub enum GameLayer {
    #[default]
    Default,
    Collectible,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Geometry Wars"),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            HanabiPlugin,
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
        .init_state::<AppState>();
    }
}
