use bevy::{core::Name, ecs::{entity::Entity, event::EventWriter, query::With, system::{Query, Res}}, render::camera::Camera, transform::components::GlobalTransform};
use bevy_egui::{egui::{Align2, Slider, Vec2, Window}, EguiContexts};
use bevy_xpbd_2d::{components::LinearVelocity, plugins::spatial_query::{SpatialQuery, SpatialQueryFilter}};

use crate::{asteroid::components::Asteroid, events::{BuildHexBuildingEvent, CraftEvent}, factory::Factory, hexbase::{BuildingType, PlayerHoveringBuilding}, inventory::Inventory, player::components::Player, refinery::{Refinery, SmeltEvent}, space_station::components::SpaceStation, upgrades::{UpgradeEvent, UpgradesComponent}, GameCamera};

use super::{context_clue::resources::ContextClues, helpers::progress_string};



pub fn ui_station_menu(
    mut ctx: EguiContexts,
    cc_res: Res<ContextClues>,
    player_query: Query<(&Player, &UpgradesComponent)>,
    inventory_query: Query<&Inventory, With<SpaceStation>>,
    factory_query: Query<&Factory>,
    refinery_query: Query<&Refinery>,
    mut craft_events: EventWriter<CraftEvent>,
    mut smelt_events: EventWriter<SmeltEvent>,
    mut upgrade_events: EventWriter<UpgradeEvent>,
) {
    Window::new("BaseStation Information")
        .anchor(Align2::RIGHT_BOTTOM, Vec2 { x: 0.0, y: 0.0 })
        .show(ctx.ctx_mut(), |ui| {
            ui.group(|ui| {
                let (_, upgrades) = player_query.single();

                ui.heading("Ship Upgrades:");

                for upgrade in &upgrades.upgrades {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.label(format!("{:?}", upgrade));
                                if ui.button("Upgrade").clicked() {
                                    upgrade_events.send(UpgradeEvent(*upgrade));
                                }
                            });

                            ui.vertical(|ui| {
                                ui.label("Requires: ");
                                if upgrade.requirements().is_some() {
                                    for requirement in
                                        upgrade.requirements().unwrap().requirements
                                    {
                                        // TODO: This seems unnecessairly convoluted..
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

pub fn ui_ship_information(
    player_query: Query<(&mut Player, &mut LinearVelocity)>,
    mut ctx: EguiContexts,
) {
    let (player, velocity) = player_query.single();

    Window::new("Ship Information")
        .anchor(
            Align2::LEFT_TOP,
            Vec2 { x: 0.0, y: 0.0 },
        )
        .show(ctx.ctx_mut(), |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.horizontal(|ui| {
                    ui.group(|ui| {
                        ui.label(format!(
                            "Health: {:.2}%",
                            player.health.current_percent() * 100.0
                        ));
                        let health_percent = player.health.current_percent();
                        ui.label(progress_string(health_percent));
                    });

                    ui.group(|ui| {
                        ui.label(format!("Battery: {:.2}KWh", player.battery.current()));
                        let battery_percent = player.battery.current() / 1000.0;
                        ui.label(progress_string(battery_percent));
                    });
                });

                ui.horizontal(|ui| {
                    ui.add(
                    Slider::new(&mut player.engine.power_level.clone(), 0.0..=100.0)
                                .text("Engine Power"),
                    );
                    ui.label(format!("Speed: {:.2}", velocity.0.length()));

                    let direction_radians = velocity.0.angle_between(hexx::Vec2::X);

                    // TODO: procedure to convert direction_radians to cardinal directions (nice to have)
                    ui.label(format!("Direction: {:.2}", direction_radians));
                });
            });
        });
}



pub fn ui_ship_hover_context(
    // mut egui_ctx: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut ctx: EguiContexts,
    player_hovering_building: Res<PlayerHoveringBuilding>,
    // player_query: Query<(&Player, &UpgradesComponent)>,
    inventory_query: Query<&Inventory, With<SpaceStation>>,
    factory_query: Query<&Factory>,
    refinery_query: Query<&Refinery>,
    mut craft_events: EventWriter<CraftEvent>,
    mut smelt_events: EventWriter<SmeltEvent>,
    // mut upgrade_events: EventWriter<UpgradeEvent>,
    mut build_event: EventWriter<BuildHexBuildingEvent>,
) {
    if player_hovering_building.0.is_some() {
        let building = &player_hovering_building.0.as_ref().unwrap().1;

        Window::new("Ship Hovering Context")
            .anchor(
                Align2::RIGHT_BOTTOM,
                bevy_inspector_egui::egui::Vec2 { x: 0.0, y: 0.0 },
            )
            .show(ctx.ctx_mut(), |ui| {
                ui.group(|ui| {
                    ui.heading(format!("Ship Hovering Over {:?}", building));
                    let inventory = inventory_query.single();

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
                                        build_event.send(BuildHexBuildingEvent(
                                            player_hovering_building.0.unwrap().0,
                                            button.1,
                                        ));
                                    }
                                }
                            });
                        }
                        BuildingType::Factory => {
                            ui.group(|ui| {
                                let factory = factory_query.single();

                                match &factory.currently_processing {
                                    Some(recipe) => {
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
                                                (recipe.time_required
                                                    - factory.remaining_processing_time)
                                                    / recipe.time_required,
                                            ));
                                        });
                                    }
                                    None => {}
                                }

                                ui.heading("Factory Recipes:");
                                for recipe in &factory.recipes {
                                    ui.group(|ui| {
                                        ui.vertical(|ui| {
                                            ui.horizontal(|ui| {
                                                ui.label(format!("{:?}", recipe.item_created));

                                                ui.label(format!(
                                                    "Requires: {:?}",
                                                    recipe.items_required
                                                ));
                                                if inventory
                                                    .has_items(recipe.items_required.clone())
                                                {
                                                    ui.label("👍");
                                                }
                                            });

                                            ui.horizontal(|ui| {
                                                ui.label(format!(
                                                    "Time Required: {:.1} sec",
                                                    recipe.time_required
                                                ));

                                                if ui.button("Craft").clicked() {
                                                    craft_events
                                                        .send(CraftEvent(recipe.clone()));
                                                }
                                            })
                                        });
                                    });
                                }
                            });
                        }
                        BuildingType::Refinery => {
                            ui.group(|ui| {
                                let refinery = refinery_query.single();

                                match &refinery.currently_processing {
                                    Some(recipe) => {
                                        ui.group(|ui| {
                                            ui.heading("Refinery Processing:");

                                            ui.label(format!(
                                                "Currently Refining: {:?}",
                                                recipe
                                            ));
                                            ui.label(format!(
                                                "Time Remaining: {:.1} sec",
                                                refinery.remaining_processing_time
                                            ));
                                            ui.label(progress_string(
                                                (recipe.time_required
                                                    - refinery.remaining_processing_time)
                                                    / recipe.time_required,
                                            ));
                                        });
                                    }
                                    None => {}
                                }

                                ui.heading("Refine Raw Ores:");

                                for recipe in &refinery.recipes {
                                    ui.group(|ui| {
                                        ui.vertical(|ui| {
                                            ui.horizontal(|ui| {
                                                ui.label(format!("{:?}", recipe.item_created));
                                                ui.label(format!(
                                                    "Requires: {:?}",
                                                    recipe.items_required
                                                ));
                                                if inventory
                                                    .has_items(recipe.items_required.clone())
                                                {
                                                    ui.label("👍");
                                                }
                                            });

                                            ui.horizontal(|ui| {
                                                ui.label(format!(
                                                    "Time Required: {:.1} sec",
                                                    recipe.time_required
                                                ));

                                                if ui.button("Smelt").clicked() {
                                                    smelt_events
                                                        .send(SmeltEvent(recipe.clone()));
                                                }
                                            })
                                        });
                                    });
                                }
                            });
                        }
                        BuildingType::Storage => {
                            let inventory = inventory_query.single();

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
}

pub fn ui_mouse_hover_context(
    // mut egui_ctx_query: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut ctx: EguiContexts,
    window_query: Query<&bevy::window::Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    // rapier_context: Res<RapierContext>,
    ent_query: Query<(Entity, &Name, Option<&Asteroid>)>,
    spatial_q: SpatialQuery,
) {
    let window = window_query.single();
    let (camera, camera_transform) = camera_q.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        Window::new("Mouse Context")
            .anchor(
                Align2::CENTER_TOP,
                bevy_inspector_egui::egui::Vec2 { x: 0.0, y: 0.0 },
            )
            .show(ctx.ctx_mut(), |ui| {
                ui.group(|ui| {
                    ui.label(format!(
                        "X:{:.2} Y:{:.2}",
                        world_position.x, world_position.y
                    ));
                });

                // Raycast Mouse Position Into Viewport

                let ray_pos = world_position;
                let ray_dir = bevy::prelude::Vec2::Y;

                let max_toi = 0.001;
                let solid = true; // i think?

                if let Some(ray_hit) = spatial_q.cast_ray(
                    ray_pos,
                    ray_dir,
                    max_toi,
                    true,
                    SpatialQueryFilter::default(),
                ) {
                    if let Ok((_ent, name, asteroid)) = ent_query.get(ray_hit.entity) {
                        ui.group(|ui| {
                            ui.heading(format!("{}", name));

                            if let Some(asteroid) = asteroid {
                                ui.label(format!(
                                    "Health: {:.2}%",
                                    asteroid.health.current_percent() * 100.0
                                ));
                                let health_percent = asteroid.health.current_percent();
                                ui.label(progress_string(health_percent));

                                ui.label("Composition:");
                                ui.label(format!("{:?}", asteroid.composition));
                            }
                        });
                    };
                }
            });
    };
}