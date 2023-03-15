use bevy_egui::{egui::{self, Align2, Vec2}, EguiContexts, EguiPlugin};

use bevy_rapier2d::prelude::Velocity;
use bevy::{prelude::*, utils::HashSet};
use egui_dnd::{DragDropUi, utils::shift_vec};

use crate::{
    base_station::BaseStation,
    factory::{Factory, CraftEvent},
    inventory::{Inventory, InventoryItem},
    player::{Player},
    refinery::{Refinery, SmeltEvent}, upgrades::{UpgradeType, UpgradesComponent, UpgradeEvent}, player_input::EnginePowerEvent,
};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct UIItems {
    pub ship_inventory_items: Vec<InventoryItem>,
    pub station_inventory_items: Vec<InventoryItem>,
    pub refinery: Refinery,
    pub factory: Factory,
    pub remaining_refinery_time: f32,
    pub context_clues: HashSet<ContextClue>,
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
            // _ => "Missing Context Clue Note.",
        }
        .to_string()
    }
}

#[derive(Resource)]
pub struct ContextClues(pub HashSet<ContextClue>);

#[derive(Hash, Clone)]
struct ItemType {
    name: String,
}

pub struct DND(DragDropUi, Vec<ItemType>);

impl Default for DND {

    fn default() -> Self {
        Self(DragDropUi::default(), ["iron", "silver", "gold"]
                        .iter()
                        .map(|name| ItemType {
                            name: name.to_string(),
                        })
                        .collect())
    }

}

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
            .add_plugin(EguiPlugin)
            .insert_resource(ContextClues(HashSet::new()))
            .add_system(Self::ui_ship_information)
            .add_system(Self::ui_ship_inventory)
            .add_system(Self::ui_station_menu)
            .add_system(Self::ui_context_clue)
            .add_system(Self::dnd);
    }
}

impl GameUIPlugin {
    fn dnd(
        mut dnd: Local<DND>,
        mut contexts: EguiContexts,
    ) {
        egui::Window::new("DND").show(contexts.ctx_mut(), |ui| {

            let mut items = dnd.1.clone();

            let response =
                // make sure this is called in a vertical layout.
                // Horizontal sorting is not supported yet.
                dnd.0.ui::<ItemType>(ui, items.iter_mut(), |item, ui, handle| {
                    ui.horizontal(|ui| {
                        // Anything in the handle can be used to drag the item
                        handle.ui(ui, item, |ui| {
                            ui.label(&item.name);
                        });
                    });
                });

            // After the drag is complete, we get a response containing the old index of the
            // dragged item, as well as the index it was moved to. You can use the
            // shift_vec function as a helper if you store your items in a Vec.
            if let Some(response) = response.completed {
                shift_vec(response.from, response.to, &mut dnd.1);
            }
        });
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
        let inventory = inventory_query.single();

        let cc = cc_res.0.clone();
        egui::SidePanel::right("BaseStation Contextual Menu").show_animated(contexts.ctx_mut(), cc.contains(&ContextClue::NearBaseStation),|ui| {

            ui.heading("Base Station Inventory:");
            ui.vertical(|ui| {
                for item in inventory.items.clone() {
                    ui.label(format!("{:?}", item));
                }
            });


            let refinery = refinery_query.single();

            match &refinery.currently_processing {
                Some(recipe) => {
                    ui.heading("Refinery Processing:");

                    ui.label(format!("Currently Refining: {:?}", recipe));
                    ui.label(format!("Time Remaining: {:.1} sec", refinery.remaining_processing_time));
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
                        if inventory.has_items(recipe.items_required.clone()) { ui.label("👍");}
                    });

                    ui.horizontal(|ui| {
                        ui.label(format!("Time Required: {:.1} sec", recipe.time_required));

                        if ui.button("Smelt").clicked() {
                            smelt_events.send(SmeltEvent(recipe.clone()));
                        }
                    })
                });
            }

            let factory = factory_query.single();

            match &factory.currently_processing {
                Some(recipe) => {
                    ui.heading("Factory Processing:");
                    ui.label(format!("Currently Crafting: {:?}", recipe.item_created));
                    ui.label(format!("Time Remaining: {:.1} sec", factory.remaining_processing_time));
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
                        if inventory.has_items(recipe.items_required.clone()) { ui.label("👍");}
                        
                    });

                    ui.horizontal(|ui| {
                        ui.label(format!("Time Required: {:.1} sec", recipe.time_required));

                        if ui.button("Craft").clicked() {
                            craft_events.send(CraftEvent(recipe.clone()));
                        }
                    })
                });

            }

            let (_, upgrades) = player_query.single();

            ui.heading("Ship Upgrades:");

            for upgrade in &upgrades.upgrades {
                    ui.horizontal(|ui| {

                        ui.vertical(|ui| {

                            ui.label(format!("{:?}", upgrade));
                            if ui.button("Upgrade").clicked() {
                                upgrade_events.send(UpgradeEvent(upgrade.clone()));
                            }
                        });
                        
                        ui.vertical(|ui| {
                            ui.label("Requires: ");
                            if upgrade.requirements().is_some() {
                                for requirement in upgrade.requirements().unwrap().requirements { // TODO: This seems unnecessairly convoluted..
                                    ui.label(format!("{:?}", requirement));
        
                                }
                            }
                        });
            
                    });

            }
        
        });

    }
    
    fn ui_ship_information(
        mut engine_power: Local<f32>,
        mut contexts: EguiContexts,
        player_query: Query<(&mut Player, &mut Velocity)>,
        mut engine_power_events: EventReader<EnginePowerEvent>,
    ) {
        let player = player_query.single();

        for event in engine_power_events.iter() {
            *engine_power = num::clamp(engine_power.clone() + event.0, 0.0, 100.0);
        }
    
        egui::Window::new("Ship Information").anchor(Align2::LEFT_TOP, Vec2::ZERO).show(contexts.ctx_mut(), |ui| {
            ui.label(format!("Health: {:.2}%", player.0.health.current()));
            let health_percent = player.0.health.current() / 100.0;
            ui.label(Self::progress_string(health_percent));
    
            ui.label(format!("Battery: {:.2}KWh", player.0.battery.current()));
            let battery_percent = player.0.battery.current() / 1000.0;
            ui.label(Self::progress_string(battery_percent));
    
            ui.label(format!("Speed: {:.2}", player.1.linvel.length()));
            ui.add(egui::Slider::new(&mut engine_power.clone(), 0.0..=100.0).text("Engine Power"));
            // ui.label(format!("Direction: {:.2}", player.1.linvel.angle_between(Vec2::X)));
        });
    }
    
    fn ui_ship_inventory(
        mut contexts: EguiContexts,
        inventory_query: Query<&Inventory, With<Player>>,
    ) {
        let inventory = inventory_query.single();
    
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
    ) {
        let cc = &context_clues_res.0;
    
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
