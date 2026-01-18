use bevy::ecs::system::Res;
use bevy_egui::{
    egui::{self, Align2, Window},
    EguiContexts,
};

use super::resources::ContextClues;

pub fn ui_context_clue(mut contexts: EguiContexts, context_clues_res: Res<ContextClues>) {
    let cc = &context_clues_res.0;
    if cc.is_empty() {
        return;
    }

    let ctx = match contexts.ctx_mut() {
        Ok(ctx) => ctx,
        Err(_) => return, // or log + return
    };

    egui::Window::new("Context Clue")
        .title_bar(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_BOTTOM, egui::vec2(0.0, -100.0))
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                for clue in cc.iter() {
                    ui.label(clue.text());
                }
            });

            // If you need “available rect”, do it HERE:
            // let rect = ui.available_rect_before_wrap();
        });
}
