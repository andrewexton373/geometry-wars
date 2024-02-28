use bevy::{app::{App, Plugin, Update}, ecs::schedule::{common_conditions::in_state, IntoSystemConfigs, OnExit}};

use crate::AppState;

use super::systems::{handle_build_mode_enter, handle_build_mode_exit, highlight_build_locations};

pub struct BuildModePlugin;

impl Plugin for BuildModePlugin {
    fn build(&self, app: &mut App) {            
        app
            .add_systems(Update, (
                highlight_build_locations.run_if(in_state(AppState::BuildMode)),
                handle_build_mode_enter.run_if(in_state(AppState::InGame)),
                handle_build_mode_exit.run_if(in_state(AppState::BuildMode))
            ));


    }
}

