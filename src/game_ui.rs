use std::fmt::format;

use bevy_egui::{egui::{self, Pos2, Align2, Vec2}, EguiContexts, EguiPlugin};

use bevy_rapier2d::prelude::Velocity;
use bevy::{prelude::*, utils::HashSet};

use crate::{
    base_station::BaseStation,
    factory::{Factory, CraftEvent},
    inventory::{Inventory, InventoryItem},
    player::{Player, ShipInformation},
    refinery::{Refinery, SmeltEvent}, upgrades::{UpgradeType, UpgradesComponent, UpgradeEvent},
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
            // .add_system(Self::ui_station_inventory)
            // .add_system(Self::ui_crafting_menu)
            // .add_system(Self::ui_refinery_menu)
            .add_system(Self::ui_station_menu)
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

    fn ui_station_menu(
        mut contexts: EguiContexts,
        cc_res: Res<ContextClues>,
        player_query: Query<(&Player, &UpgradesComponent)>,
        inventory_query: Query<&Inventory, With<BaseStation>>,
        factory_query: Query<&Factory>,
        refinery_query: Query<&Refinery>,
        mut craft_events: EventWriter<CraftEvent>,
        mut smelt_events: EventWriter<SmeltEvent>,
        mut upgrade_events: EventWriter<UpgradeEvent>,
    ) {
        let inventory = inventory_query.single() else {
            return;
        };

        let cc = cc_res.0.clone();
        egui::SidePanel::right("BaseStation Contextual Menu").show_animated(contexts.ctx_mut(), cc.contains(&ContextClue::NearBaseStation),|ui| {

            ui.heading("Base Station Inventory:");
            ui.vertical(|ui| {
                for item in inventory.items.clone() {
                    ui.label(format!("{:?}", item));
                }
            });


            let refinery = refinery_query.single() else { return; };

            match &refinery.currently_processing {
                Some(recipe) => {
                    ui.heading("Refinery Processing:");

                    ui.label(format!("Currently Refining: {:?}", recipe));
                    ui.label(format!("Time Remaining: {}", refinery.remaining_processing_time));
                    ui.label(Self::progress_string( (recipe.time_required - refinery.remaining_processing_time) / recipe.time_required));
                },
                None => {}
            }
    
            ui.heading("Refine Raw Ores:");

            for recipe in &refinery.recipes {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(format!("{:?}", recipe.item_created));
                        ui.label(format!("Requires: {:?}", recipe.items_required));
                        
                    });

                    ui.horizontal(|ui| {
                        ui.label(format!("Time Required: {}", recipe.time_required));

                        if ui.button("Smelt").clicked() {
                            smelt_events.send(SmeltEvent(recipe.clone()));
                        }
                    })
                });
            }

            let factory = factory_query.single() else { return; };

            match &factory.currently_processing {
                Some(recipe) => {
                    ui.heading("Factory Processing:");
                    ui.label(format!("Currently Crafting: {:?}", recipe.item_created));
                    ui.label(format!("Time Remaining: {}", factory.remaining_processing_time));
                    ui.label(Self::progress_string( (recipe.time_required - factory.remaining_processing_time) / recipe.time_required));
                },
                None => {}
            }
    
            ui.heading("Factory Recipes:");
            for recipe in &factory.recipes {

                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(format!("{:?}", recipe.item_created));
                        ui.label(format!("Requires: {:?}", recipe.items_required));
                        
                    });

                    ui.horizontal(|ui| {
                        ui.label(format!("Time Required: {}", recipe.time_required));

                        if ui.button("Craft").clicked() {
                            craft_events.send(CraftEvent(recipe.clone()));
                        }
                    })
                });

            }

            let (_, upgrades) = player_query.single();

            ui.heading("Ship Upgrades:");

            for upgrade in &upgrades.upgrades {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(format!("{:?}", upgrade));
                        ui.label(format!("Requires: {:?}", upgrade.requirements()));
                        
                    });

                    ui.horizontal(|ui| {
                        // ui.label(format!("Time Required: {}", recipe.time_required));

                        if ui.button("Upgrade").clicked() {
                            upgrade_events.send(UpgradeEvent(upgrade.clone()));
                        }
                    })
                });
            }
        
        });

    }
    
    fn ui_ship_information(
        mut contexts: EguiContexts,
        mut player_query: Query<(&mut Player, &mut Velocity)>,
    ) {
        let player = player_query.single() else { return; };

        let top_left = Pos2::new(0.0, 0.0);
    
        egui::Window::new("Ship Information").anchor(Align2::LEFT_TOP, Vec2::ZERO).show(contexts.ctx_mut(), |ui| {
            ui.label(format!("Health: {:.2}%", player.0.health.current()));
            let health_percent = player.0.health.current() / 100.0;
            ui.label(Self::progress_string(health_percent));
    
            ui.label(format!("Battery: {:.2}KWh", player.0.battery.current()));
            let battery_percent = player.0.battery.current() / 1000.0;
            ui.label(Self::progress_string(battery_percent));
    
            ui.label(format!("Speed: {:.2}", player.1.linvel.length()));
            // ui.label(format!("Direction: {:.2}", player.1.linvel.angle_between(Vec2::X)));
        });
    }
    
    fn ui_ship_inventory(
        mut contexts: EguiContexts,
        inventory_query: Query<&Inventory, With<Player>>,
        window_query: Query<&Window>
    ) {
        let inventory = inventory_query.single() else {
            return;
        };

        let window = window_query.single();
    
        egui::Window::new("Ship Inventory").anchor(Align2::LEFT_BOTTOM, Vec2::ZERO).show(contexts.ctx_mut(), |ui| {
            let inventory_capacity_percent = (1.0 - inventory.remaining_capacity() / inventory.capacity.maximum) * 100.0;
            ui.label(format!("Capacity: {:.2}%", inventory_capacity_percent));
            ui.label(Self::progress_string(inventory_capacity_percent / 100.0));
    
            ui.label("Contents:");
            ui.vertical(|ui| {
                for item in inventory.items.clone() {
                    ui.label(format!("{:?}", item));
                }
            });
        });
    }
    
    fn ui_context_clue(
        mut contexts: EguiContexts,
        context_clues_res: Res<ContextClues>,
        window_query: Query<&Window>
    ) {
        let cc = &context_clues_res.0;
        let window = window_query.single();
    
        if !cc.is_empty() {
            egui::Window::new("Context Clue").anchor(Align2::CENTER_BOTTOM, Vec2::new(0.0, 100.0)).show(contexts.ctx_mut(), |ui| {
                ui.vertical(|ui| {
                    for clue in cc {
                        ui.label(format!("{}", clue.text()));
                    }
                });
            });
        }    
    }
}
