use bevy::prelude::Res;
use bevy_egui::{egui::{Align2, Vec2, Window}, EguiContexts};

use crate::player_input::resources::MouseScreenPosition;

pub fn ui_mouse_coordinates(
    mut ctx: EguiContexts,
    mouse_screen_position: Res<MouseScreenPosition>,
) {
    Window::new("Mouse Coordinates")
        .anchor(Align2::RIGHT_TOP, Vec2::ZERO)
        .title_bar(false)
        .resizable(false)
        .show(ctx.ctx_mut(), |ui| {
            ui.group(|ui| {
                ui.label(format!(
                    "X:{:.2} Y:{:.2}",
                    mouse_screen_position.0.x, mouse_screen_position.0.y
                ));
            });

        });
}