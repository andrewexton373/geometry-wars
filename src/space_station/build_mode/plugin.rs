use bevy::{
    app::{App, Plugin, Startup, Update},
    ecs::schedule::{common_conditions::in_state, IntoSystemConfigs, OnExit},
};

use crate::{space_station::systems::color_space_station_modules, AppState};

use super::{
    events::BuildSpaceStationModuleEvent,
    resources::BuildModeMaterials,
    systems::{
        color_hexes, handle_build_events, handle_build_mode_enter, handle_build_mode_exit,
        highlight_build_locations, init_materials,
    },
};

pub struct BuildModePlugin;

impl Plugin for BuildModePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BuildModeMaterials>()
            .add_event::<BuildSpaceStationModuleEvent>()
            .add_systems(Startup, init_materials)
            .add_systems(
                Update,
                (
                    color_hexes
                        .after(color_space_station_modules)
                        .run_if(in_state(AppState::BuildMode)),
                    highlight_build_locations.run_if(in_state(AppState::BuildMode)),
                    handle_build_events.run_if(in_state(AppState::BuildMode)),
                    handle_build_mode_enter.run_if(in_state(AppState::InGame)),
                    handle_build_mode_exit.run_if(in_state(AppState::BuildMode)),
                ),
            );
    }
}
