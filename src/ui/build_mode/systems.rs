use std::fmt::format;

use bevy::prelude::*;
use bevy_egui::egui::panel::Side;
use bevy_egui::egui::CentralPanel;
use bevy_egui::egui::{Frame, Margin, Pos2, SidePanel};
use bevy_egui::{egui::Window, EguiContexts};

use crate::camera::components::GameCamera;
use crate::hexgrid::resources::SelectedHex;

pub fn ui_build_mode(
    mut ctx: EguiContexts,
    selected: Res<SelectedHex>,
    entity_g_t_q: Query<&GlobalTransform, Without<GameCamera>>,
    camera: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
) {
    if let Some(selected) = selected.entity {
        if let Ok(gt) = entity_g_t_q.get(selected) {
            if let Ok((camera, camera_gt)) = camera.get_single() {
                if let Some(computed_pos) = camera.world_to_viewport(camera_gt, gt.translation()) {
                    let pos = Pos2 {
                        x: computed_pos.x,
                        y: computed_pos.y,
                    };

                    Window::new("BUILD MODE").auto_sized().fixed_pos(pos).show(
                        ctx.ctx_mut(),
                        |ui| {
                            ui.label("BUILD MODE");
                            ui.label(format!("SELECTED HEX: {:?}", selected));
                        },
                    );
                }
            }
        }
    }
}
