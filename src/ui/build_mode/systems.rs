
use bevy::prelude::*;
use bevy_egui::egui::Pos2;
use bevy_egui::{egui::Window, EguiContexts};

use crate::camera::components::GameCamera;
use crate::hexgrid::resources::SelectedHex;
use crate::space_station::build_mode::events::BuildSpaceStationModuleEvent;
use crate::space_station::modules::components::SpaceStationModuleType;

pub fn ui_build_mode(
    mut ctx: EguiContexts,
    selected: Res<SelectedHex>,
    entity_g_t_q: Query<&GlobalTransform, Without<GameCamera>>,
    camera: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    mut build_events: EventWriter<BuildSpaceStationModuleEvent>,
) {
    if let Some(selected) = selected.entity {
        if let Ok(gt) = entity_g_t_q.get(selected) {
            if let Ok((camera, camera_gt)) = camera.get_single() {
                if let Some(computed_pos) = camera.world_to_viewport(camera_gt, gt.translation()).ok() {
                    let pos = Pos2 {
                        x: computed_pos.x,
                        y: computed_pos.y,
                    };

                    Window::new("BUILD MODE").auto_sized().fixed_pos(pos).show(
                        ctx.ctx_mut(),
                        |ui| {
                            ui.label("BUILD MODE");
                            ui.label(format!("SELECTED HEX: {:?}", selected));

                            ui.group(|ui| {
                                let buttons: Vec<_> = vec![
                                    ("Storage", SpaceStationModuleType::Storage),
                                    ("Factory", SpaceStationModuleType::Factory),
                                    ("Refinery", SpaceStationModuleType::Refinery),
                                    ("Core", SpaceStationModuleType::Core),
                                    ("Turret", SpaceStationModuleType::Turret),
                                ];

                                for button in buttons {
                                    if ui.button(button.0).clicked() {
                                        println!("SEND EVENT");
                                        build_events.send(BuildSpaceStationModuleEvent {
                                            entity: selected,
                                            module_type: button.1,
                                        });
                                    }
                                }
                            });
                        },
                    );
                }
            }
        }
    }
}
