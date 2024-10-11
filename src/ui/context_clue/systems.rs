use bevy::ecs::system::Res;
use bevy_egui::{
    egui::{Align2, Window},
    EguiContexts,
};

use super::resources::ContextClues;

pub fn ui_context_clue(mut ctx: EguiContexts, context_clues_res: Res<ContextClues>) {
    let cc = &context_clues_res.0;
    if cc.is_empty() {
        return;
    };

    Window::new("Context Clue")
        .title_bar(false)
        .resizable(false)
        .anchor(
            Align2::CENTER_BOTTOM,
            bevy_egui::egui::Vec2 { x: 0.0, y: 100.0 },
        )
        .show(ctx.ctx_mut(), |ui| {
            ui.vertical(|ui| {
                for clue in cc {
                    ui.label(clue.text());
                }
            });
        });
}
