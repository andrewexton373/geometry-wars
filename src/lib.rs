mod ai;
mod asteroid;
mod asteroid_field;
mod background;
mod battery;
mod camera;
mod collectible;
mod crosshair;
mod events;
mod factory;
mod health;
mod hexgrid;
mod inventory;
mod item_producer;
mod items;
mod laser;
mod particles;
mod player;
mod player_input;
mod projectile;
mod rcs;
mod recipe;
mod refinery;
mod sector;
mod space_station;
mod ui;
mod upgrades;

use asteroid_field::AsteroidFieldPlugin;
use avian2d::{
    prelude::{Gravity, PhysicsDebugPlugin, PhysicsLayer},
    PhysicsPlugins,
};
use bevy::prelude::*;
use bevy_hanabi::HanabiPlugin;

use background::plugin::BackgroundPlugin;
use sector::SectorPlugin;

use crate::crosshair::plugin::CrosshairPlugin;
use crate::laser::plugin::LaserPlugin;
use asteroid::plugin::AsteroidPlugin;
use battery::plugin::BatteryPlugin;
use camera::plugin::GameCameraPlugin;
use collectible::plugin::CollectiblesPlugin;
use factory::FactoryPlugin;
use health::plugin::HealthPlugin;
use hexgrid::plugin::HexBasePlugin;
use inventory::plugin::InventoryPlugin;
use particles::plugin::ParticlePlugin;
use player::plugin::PlayerPlugin;
use player_input::plugin::PlayerInputPlugin;
use projectile::plugin::ProjectilePlugin;
use rcs::plugin::RCSPlugin;
use refinery::RefineryPlugin;
use space_station::plugin::SpaceStationPlugin;
use ui::plugin::GameUIPlugin;
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
            // PerfUiPlugin,
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
            CollectiblesPlugin,
            // AiPlugin,
            ProjectilePlugin,
            BackgroundPlugin,
            SectorPlugin,
            AsteroidFieldPlugin,
        ))
        // .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::new(100))
        // .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin {
        //     max_history_length: 100,
        // })
        // .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
        .add_systems(Startup, setup)
        .insert_resource(Gravity::ZERO)
        .init_state::<AppState>();
    }
}

fn setup(mut commands: Commands) {
    // commands.spawn(PerfUiAllEntries::default());
}
