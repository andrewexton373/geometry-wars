use super::{resources::ContextClues, systems::ui_context_clue};
use bevy::{platform::collections::HashSet, prelude::*};
use bevy_egui::EguiPrimaryContextPass;

pub struct ContextCluePlugin;

impl Plugin for ContextCluePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(ContextClues(HashSet::new()))
            .add_systems(EguiPrimaryContextPass, (ui_context_clue,));
    }
}
