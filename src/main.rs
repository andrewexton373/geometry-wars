use bevy_stat_bars::*;
use bevy::{prelude::*, diagnostic::{FrameTimeDiagnosticsPlugin, Diagnostics}};
use bevy_debug_text_overlay::{screen_print, OverlayPlugin};
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::prelude::*;
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

mod base_station;
use base_station::BaseStationPlugin;

// Defines the amount of time that should elapse between each physics step.
const TIME_STEP: f32 = 1.0 / 60.0;

pub const PIXELS_PER_METER : f32 = 10.0;

const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);

#[derive(Component)]
struct Collider;

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
struct FpsText;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(ShapePlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(BaseStationPlugin)
        .add_plugin(AstroidPlugin)
        .add_plugin(ProjectilePlugin)
        .add_plugin(CrosshairPlugin)
        .add_plugin(StatBarsPlugin)
        .add_plugin(HealthBarPlugin)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_system(camera_follows_player)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(PIXELS_PER_METER))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(OverlayPlugin { font_size: 18.0, ..default() })
        .add_system(screen_print_debug_text)
        .run();
}

#[derive(Component)]
pub struct InventoryText;

fn setup(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
    asset_server: Res<AssetServer>
) {
    let camera = commands.spawn_bundle(Camera2dBundle::default()).id();
    HealthBarPlugin::attach_player_health_bar(&mut commands, camera);
    rapier_config.gravity = Vec2::new(0.0, 0.0);
}

fn camera_follows_player(
    mut camera_query: Query<(&Camera, &mut GlobalTransform), With<Camera>>,
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
){
    let (_camera, mut camera_trans) = camera_query.single_mut().into();
    let player_trans = player_query.single();

        // TODO: seems sloppy, is there another way?
        let player_to_camera = camera_trans.translation() - player_trans.translation;
        let mut_trans = camera_trans.translation_mut();
        mut_trans.x -= player_to_camera.x;
        mut_trans.y -= player_to_camera.y;
}

fn screen_print_debug_text(
    diagnostics: Res<Diagnostics>,
    player_query: Query<&Player>
) {
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(average) = fps.average() {
            // Update the value of the second section
            screen_print!(col: Color::WHITE, "fps: {average}");
        }
    }

    let player = player_query.single();
    let inventory = &player.inventory;
    screen_print!(col: Color::LIME_GREEN, "inventory: {inventory:?}");
}
