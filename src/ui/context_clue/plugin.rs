use bevy::{prelude::*, utils::HashSet};

use super::{resources::ContextClues, systems::ui_context_clue};

pub struct ContextCluePlugin;

impl Plugin for ContextCluePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(ContextClues(HashSet::new()))
            .add_systems(Update, (ui_context_clue,));
    }
}
