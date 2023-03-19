use bevy_egui::{egui::{self, Align2, Vec2}, EguiContexts, EguiPlugin};

use bevy_rapier2d::prelude::{Velocity, RapierContext, QueryFilter};
use bevy::{prelude::{*}, utils::HashSet, render::view::window, input::mouse};
use egui_dnd::{DragDropUi, utils::shift_vec};

use crate::{
    base_station::BaseStation,
    factory::{Factory, CraftEvent},
    inventory::{Inventory, InventoryItem},
    player::{Player},
    refinery::{Refinery, SmeltEvent}, upgrades::{UpgradeType, UpgradesComponent, UpgradeEvent},
    GameCamera
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
            .add_system(Self::ui_station_menu)
            .add_system(Self::ui_context_clue)
            .add_system(Self::dnd_ship_inventory)
            .add_system(Self::ui_mouse_hover_context);
    }
}

impl GameUIPlugin {
    fn dnd_ship_inventory(
        mut dnd: Local<DND>,
        mut contexts: EguiContexts,
        mut inventory_query: Query<(&Player, &mut Inventory)>,
    ) {
        egui::Window::new("DND Ship Inventory").auto_sized().anchor(Align2::LEFT_BOTTOM, Vec2::ZERO).show(contexts.ctx_mut(), |ui| {

            let (_, mut inventory) = inventory_query.single_mut();

            let inventory_capacity_percent = (1.0 - inventory.remaining_capacity().0 / inventory.capacity.maximum.0) * 100.0;
            ui.label(format!("Capacity: {:.2}%", inventory_capacity_percent));
            ui.label(Self::progress_string(inventory_capacity_percent / 100.0));

            ui.group(|ui| {

                let response =
                // make sure this is called in a vertical layout.
                // Horizontal sorting is not supported yet.
                
                dnd.0.ui::<InventoryItem>(ui, inventory.items.clone().iter_mut(), |item, ui, handle| {
                    
                    handle.ui(ui, item, |ui| {
                        ui.group(|ui| {
                            ui.label(format!("{:?}", item));
                        });
                        
                    });

                });

            // After the drag is complete, we get a response containing the old index of the
            // dragged item, as well as the index it was moved to. You can use the
            // shift_vec function as a helper if you store your items in a Vec.
            if let Some(response) = response.completed {
                shift_vec(response.from, response.to, &mut inventory.items);
            }

            });

            
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

            ui.group(|ui| {
                ui.heading("Base Station Inventory:");
                ui.vertical(|ui| {
                    for item in inventory.items.clone() {
                        ui.label(format!("{:?}", item));
                    }
                });
            });
            

            ui.group(|ui| {
                let refinery = refinery_query.single();

                match &refinery.currently_processing {
                    Some(recipe) => {

                        ui.group(|ui| {
                            ui.heading("Refinery Processing:");

                            ui.label(format!("Currently Refining: {:?}", recipe));
                            ui.label(format!("Time Remaining: {:.1} sec", refinery.remaining_processing_time));
                            ui.label(Self::progress_string( (recipe.time_required - refinery.remaining_processing_time) / recipe.time_required));
                        });

                    },
                    None => {}
                }
        
                ui.heading("Refine Raw Ores:");

                for recipe in &refinery.recipes {
                    ui.group(|ui| {
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
                    });

                }
            });

            ui.group(|ui| {

                let factory = factory_query.single();

                match &factory.currently_processing {
                    Some(recipe) => {

                        ui.group(|ui| {
                            ui.heading("Factory Processing:");
                            ui.label(format!("Currently Crafting: {:?}", recipe.item_created));
                            ui.label(format!("Time Remaining: {:.1} sec", factory.remaining_processing_time));
                            ui.label(Self::progress_string( (recipe.time_required - factory.remaining_processing_time) / recipe.time_required));
                        });

                    },
                    None => {}
                }
        
                ui.heading("Factory Recipes:");
                for recipe in &factory.recipes {

                    ui.group(|ui| {
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
                    });

                }

            });
            
            ui.group(|ui| {

                let (_, upgrades) = player_query.single();

                ui.heading("Ship Upgrades:");
    
                for upgrade in &upgrades.upgrades {

                    ui.group(|ui| {
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
                    });

                }

            });

        });

    }
    
    fn ui_ship_information(
        mut contexts: EguiContexts,
        player_query: Query<(&mut Player, &mut Velocity)>,
    ) {
        let (player, velocity) = player_query.single();
    
        egui::Window::new("Ship Information").anchor(Align2::LEFT_TOP, Vec2::ZERO).show(contexts.ctx_mut(), |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.horizontal(|ui| {
                    ui.group(|ui| {
                        ui.label(format!("Health: {:.2}%", player.health.current()));
                        let health_percent = player.health.current() / 100.0;
                        ui.label(Self::progress_string(health_percent));
                    });
        
                    ui.group(|ui| {
                        ui.label(format!("Battery: {:.2}KWh", player.battery.current()));
                        let battery_percent = player.battery.current() / 1000.0;
                        ui.label(Self::progress_string(battery_percent));
                    });
                    
                });
    
                ui.horizontal(|ui| {
                    ui.add(egui::Slider::new(&mut player.engine.power_level.clone(), 0.0..=100.0).text("Engine Power"));
                    ui.label(format!("Speed: {:.2}", velocity.linvel.length()));
                    // ui.label(format!("Direction: {:.2}", player.1.linvel.angle_between(Vec2::X)));
                }); 
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

    pub fn ui_mouse_hover_context(
        mut contexts: EguiContexts,
        window_query: Query<&Window>,
        camera_q: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
        rapier_context: Res<RapierContext>,
        ent_query: Query<(Entity, &Name)>
    ) {

        let window = window_query.single();
        let (camera, camera_transform) = camera_q.single();

        
            if let Some(world_position) = window.cursor_position()
                .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                .map(|ray| ray.origin.truncate())
            {
               
                egui::Window::new("Mouse Context").anchor(Align2::CENTER_TOP, Vec2::ZERO).show(contexts.ctx_mut(), |ui| {
                
                    // Raycast Mouse Position Into Viewport

                    let ray_pos = world_position;
                    let ray_dir = bevy::prelude::Vec2::Y;

                    let max_toi = 0.001;
                    let solid = true; // i think?
                    let filter = QueryFilter::default();

                    if let Some((entity, intersection)) = rapier_context.cast_ray_and_get_normal (
                        ray_pos, ray_dir, max_toi, solid, filter
                    ) {
                        let hit_point = intersection.point;
                        let hit_normal = intersection.normal;
                        ui.group(|ui| {
                            ui.label(format!("X:{:.4} Y:{:.4}", world_position.x, world_position.y));
                        });

                        if let Ok((entity, name)) = ent_query.get(entity) {
                            ui.group(|ui| {
                                ui.label(format!("{}", name));
                            });
                        };

                    }

                });
            };

    }
}
