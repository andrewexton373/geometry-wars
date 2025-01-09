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
pub(crate) mod sector;
pub(crate) mod space_station;
pub(crate) mod ui;
pub(crate) mod upgrades;

use avian2d::{
    prelude::{Gravity, PhysicsDebugPlugin, PhysicsLayer},
    PhysicsPlugins,
};
use bevy::prelude::*;
use bevy_hanabi::HanabiPlugin;

use ai::plugin::AiPlugin;
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
use iyes_perf_ui::prelude::*;
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
            PerfUiPlugin,
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
            AiPlugin,
            ProjectilePlugin,
            BackgroundPlugin,
            SectorPlugin,
        ))
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
        .add_systems(Startup, setup)
        .insert_resource(Gravity::ZERO)
        .init_state::<AppState>();
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(PerfUiAllEntries::default());
}
