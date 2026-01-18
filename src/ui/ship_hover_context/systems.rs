use bevy::prelude::*;
use bevy_egui::{
    egui::{Align2, Window},
    EguiContexts,
};

use crate::{
    events::CraftEvent,
    factory::Factory,
    hexgrid::components::BuildingType,
    hexgrid::events::BuildHexBuildingEvent,
    hexgrid::resources::PlayerHoveringBuilding,
    inventory::components::Inventory,
    refinery::{Refinery, SmeltEvent},
    space_station::components::SpaceStation,
    ui::helpers::progress_string,
};

pub fn ui_ship_hover_context(
    // mut egui_ctx: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut ctx: EguiContexts,
    player_hovering_building: Res<PlayerHoveringBuilding>,
    // player_query: Query<(&Player, &UpgradesComponent)>,
    inventory_query: Query<&Inventory, With<SpaceStation>>,
    factory_query: Query<&Factory>,
    refinery_query: Query<&Refinery>,
    mut craft_events: MessageWriter<CraftEvent>,
    mut smelt_events: MessageWriter<SmeltEvent>,
    // mut upgrade_events: EventWriter<UpgradeEvent>,
    mut build_event: MessageWriter<BuildHexBuildingEvent>,
) {
    //If player is not hovering over a building
    if player_hovering_building.0.is_none() {
        return;
    }

    let building = &player_hovering_building.0.as_ref().unwrap().1;

    Window::new("Ship Hovering Context")
        .anchor(
            Align2::RIGHT_BOTTOM,
            bevy_egui::egui::Vec2 { x: 0.0, y: 0.0 },
        )
        .show(ctx.ctx_mut().expect("Context Not Okay"), |ui| {
            ui.group(|ui| {
                ui.heading(format!("Ship Hovering Over {:?}", building));
                let inventory = inventory_query.single().expect("No Inventory");

                match building {
                    &BuildingType::None => {
                        ui.group(|ui| {
                            let buttons: Vec<_> = vec![
                                ("Storage", BuildingType::Storage),
                                ("Factory", BuildingType::Factory),
                                ("Refinery", BuildingType::Refinery),
                            ];

                            for button in buttons {
                                if ui.button(button.0).clicked() {
                                    println!("SEND EVENT");
                                    build_event.write(BuildHexBuildingEvent(
                                        player_hovering_building.0.unwrap().0,
                                        button.1,
                                    ));
                                }
                            }
                        });
                    }
                    BuildingType::Factory => {
                        ui.group(|ui| {
                            let factory = factory_query.single().expect("No Factory");

                            if let Some(recipe) = &factory.currently_processing {
                                ui.group(|ui| {
                                    ui.heading("Factory Processing:");
                                    ui.label(format!(
                                        "Currently Crafting: {:?}",
                                        recipe.item_created
                                    ));
                                    ui.label(format!(
                                        "Time Remaining: {:.1} sec",
                                        factory.remaining_processing_time
                                    ));
                                    ui.label(progress_string(
                                        (recipe.time_required - factory.remaining_processing_time)
                                            / recipe.time_required,
                                    ));
                                });
                            }

                            ui.heading("Factory Recipes:");
                            for recipe in factory.recipes.iter() {
                                ui.group(|ui| {
                                    ui.vertical(|ui| {
                                        ui.horizontal(|ui| {
                                            ui.label(format!("{:?}", recipe.item_created));

                                            ui.label(format!(
                                                "Requires: {:?}",
                                                recipe.items_required
                                            ));
                                            if inventory.has_items(recipe.items_required.clone()) {
                                                ui.label("👍");
                                            }
                                        });

                                        ui.horizontal(|ui| {
                                            ui.label(format!(
                                                "Time Required: {:.1} sec",
                                                recipe.time_required
                                            ));

                                            if ui.button("Craft").clicked() {
                                                craft_events.write(CraftEvent(recipe.clone()));
                                            }
                                        })
                                    });
                                });
                            }
                        });
                    }
                    BuildingType::Refinery => {
                        ui.group(|ui| {
                            let refinery = refinery_query.single().expect("No Refinery");

                            if let Some(recipe) = &refinery.currently_processing {
                                ui.group(|ui| {
                                    ui.heading("Refinery Processing:");

                                    ui.label(format!("Currently Refining: {:?}", recipe));
                                    ui.label(format!(
                                        "Time Remaining: {:.1} sec",
                                        refinery.remaining_processing_time
                                    ));
                                    ui.label(progress_string(
                                        (recipe.time_required - refinery.remaining_processing_time)
                                            / recipe.time_required,
                                    ));
                                });
                            }

                            ui.heading("Refine Raw Ores:");

                            for recipe in refinery.recipes.iter() {
                                ui.group(|ui| {
                                    ui.vertical(|ui| {
                                        ui.horizontal(|ui| {
                                            ui.label(format!("{:?}", recipe.item_created));
                                            ui.label(format!(
                                                "Requires: {:?}",
                                                recipe.items_required
                                            ));
                                            if inventory.has_items(recipe.items_required.clone()) {
                                                ui.label("👍");
                                            }
                                        });

                                        ui.horizontal(|ui| {
                                            ui.label(format!(
                                                "Time Required: {:.1} sec",
                                                recipe.time_required
                                            ));

                                            if ui.button("Smelt").clicked() {
                                                smelt_events.write(SmeltEvent(recipe.clone()));
                                            }
                                        })
                                    });
                                });
                            }
                        });
                    }
                    BuildingType::Storage => {
                        let inventory = inventory_query.single().expect("No Inventory");

                        ui.group(|ui| {
                            ui.heading("Base Station Inventory:");
                            ui.vertical(|ui| {
                                for item in inventory.items.clone() {
                                    ui.label(format!("{:?}", item));
                                }
                            });
                        });
                    }
                }
            });
        });
}
