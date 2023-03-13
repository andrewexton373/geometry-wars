use bevy_egui::{egui, EguiContexts, EguiPlugin};

use bevy_rapier2d::prelude::Velocity;
use bevy::{prelude::*, utils::HashSet};

use crate::{
    base_station::BaseStation,
    factory::{Factory, CraftEvent},
    inventory::{Inventory, InventoryItem},
    player::{Player, ShipInformation},
    refinery::{Refinery, SmeltEvent}, upgrades::UpgradeType,
};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct UIItems {
    pub ship_inventory_items: Vec<InventoryItem>,
    pub station_inventory_items: Vec<InventoryItem>,
    pub refinery: Refinery,
    pub factory: Factory,
    pub remaining_refinery_time: f32,
    pub context_clues: HashSet<ContextClue>,
    pub ship_info: ShipInformation,
    pub upgrades: Vec<UpgradeType>,
}

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ContextClue {
    #[default]
    NearBaseStation,
    CargoBayFull,
    ShipFuelEmpty,
    ShipInventoryEmpty,
}

impl ContextClue {
    pub fn text(&self) -> String {
        match *self {
            ContextClue::NearBaseStation => "Near Base Station, Deposit Collected Ore with SPACE.",
            ContextClue::CargoBayFull => {
                "The Player's Ship Cargo Bay is Full. Deposit Ore at Base Station."
            }
            ContextClue::ShipFuelEmpty => "The Player's Ship Fuel Tank is Empty!",
            ContextClue::ShipInventoryEmpty => "The Player's Ship Inventory is Empty!",
            _ => "Missing Context Clue Note.",
        }
        .to_string()
    }
}

#[derive(Resource)]
pub struct ContextClues(pub HashSet<ContextClue>);

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
            .add_plugin(EguiPlugin)
            .insert_resource(ContextClues(HashSet::new()))
            .add_system(Self::ui_ship_information)
            .add_system(Self::ui_ship_inventory)
            .add_system(Self::ui_station_inventory)
            .add_system(Self::ui_crafting_menu)
            .add_system(Self::ui_refinery_menu)
            .add_system(Self::ui_context_clue);
    }
}

impl GameUIPlugin {
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
            ui.label(Self::progress_string(health_percent));
    
            ui.label(format!("Battery: {:.2}KWh", player.0.battery.current()));
            let battery_percent = player.0.battery.current() / 1000.0;
            ui.label(Self::progress_string(battery_percent));
    
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
            ui.label(Self::progress_string(inventory_capacity_percent));
    
            ui.label("Contents:");
            ui.vertical(|ui| {
                for item in inventory.items.clone() {
                    ui.label(format!("{:?}", item));
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
                    ui.label(format!("{:?}", item));
                }
            });
        });
    }
    
    fn ui_refinery_menu(
        mut contexts: EguiContexts,
        refinery_query: Query<&Refinery>,
        mut events: EventWriter<SmeltEvent>
    ) {
        let refinery = refinery_query.single() else { return; };
    
        egui::Window::new("Refinery Menu").show(contexts.ctx_mut(), |ui| {
    
            match &refinery.currently_processing {
                Some(recipe) => {
                    ui.label(format!("Currently Crafting: {:?}", recipe));
                    ui.label(Self::progress_string( (recipe.time_required - refinery.remaining_processing_time) / recipe.time_required));
                },
                None => {}
            }
    
            ui.label("Raw Ores:");
            for recipe in &refinery.recipes {
                ui.label(format!("{:?}", recipe));
                if ui.button("Smelt").clicked() {
                    events.send(SmeltEvent(recipe.clone()));
                }
            }
    
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
                    ui.label(Self::progress_string( (recipe.time_required - factory.remaining_processing_time) / recipe.time_required));
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
}
