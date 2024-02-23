use bevy::prelude::*;

use super::resources::CanDeposit;
use super::guide_arrow::plugin::GuideArrowPlugin;

use super::systems::{
    handle_space_station_collision_event,
    repel_asteroids_from_space_station, spawn_space_station,
};

pub struct SpaceStationPlugin;

impl Plugin for SpaceStationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(GuideArrowPlugin)
            .insert_resource(CanDeposit(true))
            .add_systems(Startup, (spawn_space_station))
            .add_systems(
                Update,
                (
                    repel_asteroids_from_space_station,
                    handle_space_station_collision_event,
                ),
            );
    }
}

