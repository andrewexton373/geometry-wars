use bevy::ecs::{change_detection::DetectChangesMut, entity::Entity, event::EventReader, system::{adapter::dbg, Query}};
use bevy_xpbd_2d::components::ExternalForce;

use crate::player::components::Player;

use super::{components::RCSBooster, events::{RCSThrustPowerEvent, RCSThrustVectorEvent}};

pub fn handle_set_thrust_power_events(
    mut engine_events: EventReader<RCSThrustPowerEvent>,
    mut player_query: Query<&mut Player>,
) {
    for mut player in player_query.iter_mut() {
        for event in engine_events.read() {
            let delta = event.0;
            player.rcs_booster.delta_power_level(delta);
        }
    }
}

pub fn handle_thrust_events(
    mut thrust_vector_events: EventReader<RCSThrustVectorEvent>,
    mut entity_query: Query<(&RCSBooster, &mut ExternalForce)>
) {
    for evt in thrust_vector_events.read() {
        if let Ok((booster, mut external_force)) = entity_query.get_mut(evt.entity) {
            let thrust_vector = evt.thrust_vector * booster.power_level;
            external_force.set_force(thrust_vector);
        }
    }
}