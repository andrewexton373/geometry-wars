use bevy::prelude::*;

use crate::AppState;

use super::systems::ui_build_mode;

pub struct BuildModeUIPlugin;

impl Plugin for BuildModeUIPlugin {
    fn build(&self, app: &mut App) {
        app
            // .init_resource::<PlayerHoveringBuilding>()
            .add_systems(Update, ui_build_mode.run_if(in_state(AppState::BuildMode)));
    }
}
