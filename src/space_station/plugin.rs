use bevy::prelude::*;

use crate::hexgrid::systems::setup_hex_grid;

use super::build_mode::plugin::BuildModePlugin;
use super::guide_arrow::plugin::GuideArrowPlugin;
use super::resources::CanDeposit;

use super::systems::{
    color_space_station_modules, handle_space_station_collision_event, init_space_station_core, init_space_station_module_material_map, repel_asteroids_from_space_station, spawn_space_station
};

pub struct SpaceStationPlugin;

impl Plugin for SpaceStationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((GuideArrowPlugin, BuildModePlugin))
            .insert_resource(CanDeposit(true))
            .add_systems(Startup, (
                init_space_station_module_material_map,
                spawn_space_station,
                init_space_station_core.after(setup_hex_grid)
            ))
            .add_systems(
                Update,
                (
                    repel_asteroids_from_space_station,
                    handle_space_station_collision_event,
                    color_space_station_modules
                ),
            );
    }
}
