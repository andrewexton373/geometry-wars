// #![feature(array_methods)]

use bevy_debug_text_overlay::{screen_print, OverlayPlugin};
use bevy_egui::{egui, EguiContexts, EguiPlugin};

use astroid::AstroidPlugin;
use base_station::{BaseStationPlugin, BaseStation};
use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
    window::PresentMode,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use crosshair::CrosshairPlugin;
use factory::{FactoryPlugin, Factory, CraftEvent};
use game_ui::{GameUIPlugin, ContextClues};
use inventory::{InventoryPlugin, Inventory};
use particles::ParticlePlugin;
use player::{PlayerPlugin, Player, ShipInformation};
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
        // .add_plugins(DefaultPlugins.set(WindowPlugin {
        //     window: WindowDescriptor {
        //         title: "ASTROID MINER".to_string(),
        //         width: HEIGHT * RESOLUTION,
        //         height: HEIGHT,
        //         present_mode: PresentMode::AutoVsync,
        //       ..default()
        //     },
        //     ..default()
        //   }))
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
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
        // .add_system(ui_example_system)
        .add_system(ui_ship_information)
        .add_system(ui_ship_inventory)
        .add_system(ui_station_inventory)
        .add_system(ui_crafting_menu)
        .add_system(ui_context_clue)
        .run();
}

fn setup(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    commands
        .spawn((
            Camera2dBundle::default(),
            GameCamera,
            Name::new("GameCamera"),
        ));

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
    // let mut_trans = camera_trans.translation_mut();
    // mut_trans.x -= player_to_camera.x;
    // mut_trans.y -= player_to_camera.y;

    camera_trans.translation().x -= player_to_camera.x;
    camera_trans.translation().y -= player_to_camera.y;

}

fn screen_print_debug_text(diagnostics: Res<Diagnostics>) {
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(average) = fps.average() {
            screen_print!(col: Color::WHITE, "fps: {average}");
        }
    }
}

fn progress_string(progress: f32) -> String {
    let progress_bar_len = 10;

    return format!(
        "{}",
        (0..progress_bar_len)
            .map(|i| {
                let percent = i as f32 / progress_bar_len as f32;
                if percent < progress {
                    '◼'
                } else {
                    '◻'
                }
            })
            .collect::<String>()
    );
}

fn ui_ship_information(
    mut contexts: EguiContexts,
    mut player_query: Query<(&mut Player, &mut Velocity)>,
) {
    let player = player_query.single() else { return; };

    egui::Window::new("Ship Information").show(contexts.ctx_mut(), |ui| {
        ui.label(format!("Health: {:.2}%", player.0.health.current()));
        let health_percent = player.0.health.current() / 100.0;
        ui.label(progress_string(health_percent));

        ui.label(format!("Battery: {:.2}KWh", player.0.battery.current()));
        let battery_percent = player.0.battery.current() / 1000.0;
        ui.label(progress_string(battery_percent));

        ui.label(format!("Speed: {:.2}", player.1.linvel.length()));
        ui.label(format!("Direction: {:.2}", player.1.linvel.angle_between(Vec2::X)));
    });
}

fn ui_ship_inventory(
    mut contexts: EguiContexts,
    inventory_query: Query<&Inventory, With<Player>>,
) {
    let inventory = inventory_query.single() else {
        return;
    };

    egui::Window::new("Ship Inventory").show(contexts.ctx_mut(), |ui| {
        let inventory_capacity_percent = (1.0 - inventory.remaining_capacity() / inventory.capacity.maximum) * 100.0;
        ui.label(format!("Capacity: {:.2}%", inventory_capacity_percent));
        ui.label(progress_string(inventory_capacity_percent));

        ui.label("Contents:");
        ui.vertical(|ui| {
            for item in inventory.items.clone() {
                ui.label(format!("ITEM: {:?}", item));
            }
        });
    });
}

fn ui_station_inventory(
    mut contexts: EguiContexts,
    inventory_query: Query<&Inventory, With<BaseStation>>,
) {
    let inventory = inventory_query.single() else {
        return;
    };

    egui::Window::new("Station Inventory").show(contexts.ctx_mut(), |ui| {
        ui.label("Contents:");
        ui.vertical(|ui| {
            for item in inventory.items.clone() {
                ui.label(format!("ITEM: {:?}", item));
            }
        });
    });
}

fn ui_crafting_menu(
    mut contexts: EguiContexts,
    factory_query: Query<&Factory>,
    mut events: EventWriter<CraftEvent>
) {

    let factory = factory_query.single() else { return; };

    egui::Window::new("Crafting Menu").show(contexts.ctx_mut(), |ui| {

        match &factory.currently_processing {
            Some(recipe) => {
                ui.label(format!("Currently Crafting: {:?}", recipe));
                ui.label(progress_string( (recipe.time_required - factory.remaining_processing_time) / recipe.time_required));
            },
            None => {}
        }

        ui.label("Recipes:");
        for recipe in &factory.recipes {
            ui.label(format!("{:?}", recipe));
            if ui.button("Craft").clicked() {
                events.send(CraftEvent(recipe.clone()));
            }
        }

    });
}

fn ui_context_clue(
    mut contexts: EguiContexts,
    context_clues_res: Res<ContextClues>,
) {
    let cc = &context_clues_res.0;

    if !cc.is_empty() {
        egui::Window::new("Context Clue").show(contexts.ctx_mut(), |ui| {
            ui.vertical(|ui| {
                for clue in cc {
                    ui.label(format!("{}", clue.text()));
                }
            });
        });
    }    
}
