use bevy::ecs::system::Res;
use bevy_egui::{
    egui::{self},
    EguiContexts,
};

use super::resources::ContextClues;

pub fn ui_context_clue(mut ctx: EguiContexts, context_clues_res: Res<ContextClues>) {
    let cc = &context_clues_res.0;
    if cc.is_empty() {
        return;
    }

    egui::Window::new("Context Clue")
        .title_bar(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_BOTTOM, egui::vec2(0.0, -100.0))
        .show(ctx.ctx_mut().expect("No Context"), |ui| {
            ui.vertical(|ui| {
                for clue in cc.iter() {
                    ui.label(clue.text());
                }
            });
        });
}
