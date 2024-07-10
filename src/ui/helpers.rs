use bevy::{
    ecs::system::{Query, ResMut},
    input::{keyboard::KeyCode, mouse::MouseButton, ButtonInput},
};
use bevy_egui::EguiContext;

pub fn progress_string(progress: f32) -> String {
    let progress_bar_len = 10;

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
}

// See: https://github.com/mvlabat/bevy_egui/issues/47
pub fn absorb_egui_inputs(
    mut contexts: bevy_egui::EguiContexts,
    mut mouse: ResMut<ButtonInput<MouseButton>>,
    // mut keyboard: ResMut<ButtonInput<KeyCode>>,
) {
    let ctx = contexts.ctx_mut();
    if ctx.is_pointer_over_area() {
        mouse.reset_all();
    }
}
