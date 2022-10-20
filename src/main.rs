// #![feature(array_methods)]

use bevy::{prelude::*, diagnostic::{FrameTimeDiagnosticsPlugin, Diagnostics}, window::PresentMode, render::{
    camera::{Projection, ScalingMode},
    render_resource::WgpuFeatures,
    settings::WgpuSettings,
}};
use bevy_debug_text_overlay::{screen_print, OverlayPlugin};
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_hanabi::prelude::*;

mod game_ui;
mod widgets;
mod refinery;
mod factory;
use factory::FactoryPlugin;
use game_ui::GameUIPlugin;

mod player;
use player::{ PlayerPlugin, Player };

mod astroid;
use astroid::{AstroidPlugin};

mod projectile;
use projectile::{ProjectilePlugin};

mod crosshair;
use crosshair::CrosshairPlugin;

mod player_stats_bar;
use player_stats_bar::PlayerStatsBarPlugin;

mod base_station;
use base_station::BaseStationPlugin;

mod inventory;
use inventory::InventoryPlugin;
use refinery::RefineryPlugin;

// Defines the amount of time that should elapse between each physics step.
// const TIME_STEP: f32 = 1.0 / 60.0;

pub const PIXELS_PER_METER : f32 = 10.0;

const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);

pub const HEIGHT: f32 = 800.0;
pub const RESOLUTION: f32 = 16.0 / 9.0;

#[derive(Component)]
struct Collider;

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
struct FpsText;

#[derive(Component)]
pub struct GameCamera;

fn main() {
    let mut options = WgpuSettings::default();
    options
        .features
        .set(WgpuFeatures::VERTEX_WRITABLE_STORAGE, true);

    App::new()
        .insert_resource(options)
        .insert_resource(WindowDescriptor {
            title: "ASTROID MINER".to_string(),
            width: HEIGHT * RESOLUTION,
            height: HEIGHT,
            present_mode: PresentMode::AutoVsync,
            ..default()
        })
        .insert_resource(bevy::log::LogSettings {
            level: bevy::log::Level::WARN,
            filter: "bevy_hanabi=warn,spawn=trace".to_string(),
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(HanabiPlugin)
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
        .add_plugin(PlayerStatsBarPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(PIXELS_PER_METER))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(OverlayPlugin { font_size: 18.0, ..default() })
        .add_plugin(GameUIPlugin)
        .add_system(screen_print_debug_text)
        .run();
}

fn setup(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    commands.spawn_bundle(Camera2dBundle::default())
                            .insert(GameCamera)
                            .insert(Name::new("GameCamera"));

    rapier_config.gravity = Vec2::new(0.0, 0.0);

    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(1.0, 1.0, 1.0, 1.0));
    gradient.add_key(1.0, Vec4::new(1.0, 1.0, 1.0, 0.0));

    let spawner = Spawner::once(30.0.into(), false);
    let effect = effects.add(
        EffectAsset {
            name: "Impact".into(),
            capacity: 32768,
            spawner,
            ..Default::default()
        }
        .init(PositionSphereModifier {
            radius: 0.05 * crate::PIXELS_PER_METER,
            speed: (0.2 * crate::PIXELS_PER_METER).into(),
            dimension: ShapeDimension::Surface,
            ..Default::default()
        })
        .render(SizeOverLifetimeModifier {
            gradient: Gradient::constant(Vec2::splat(0.25 * crate::PIXELS_PER_METER)),
        })
        .render(ColorOverLifetimeModifier { gradient }),
    );

    // Spawn an instance of the particle effect, and override its Z layer to
    // be above the reference white square previously spawned.
    commands
        .spawn_bundle(ParticleEffectBundle {
            // Assign the Z layer so it appears in the egui inspector and can be modified at runtime
            effect: ParticleEffect::new(effect).with_z_layer_2d(Some(0.1)),
            ..default()
        })
        .insert(Name::new("effect:2d"));

    // commands
    //     .spawn_bundle(ParticleEffectBundle::new(effect).with_spawner(spawner))
    //     .insert(Name::new("particle-effect"));

}

fn camera_follows_player(
    mut camera_query: Query<(&Camera, &mut GlobalTransform), With<GameCamera>>,
    player_query: Query<&Transform, (With<Player>, Without<GameCamera>)>,
){
    let (_camera, mut camera_trans) = camera_query.single_mut();
    let player_trans = player_query.single();

        // TODO: seems sloppy, is there another way?
        let player_to_camera = camera_trans.translation() - player_trans.translation;
        let mut_trans = camera_trans.translation_mut();
        mut_trans.x -= player_to_camera.x;
        mut_trans.y -= player_to_camera.y;
}

fn screen_print_debug_text(
    diagnostics: Res<Diagnostics>,
) {
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(average) = fps.average() {
            // Update the value of the second section
            screen_print!(col: Color::WHITE, "fps: {average}");
        }
    }
}
