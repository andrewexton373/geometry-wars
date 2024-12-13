use avian2d::prelude::ExternalForce;
use bevy::ecs::{
    event::EventReader,
    system::Query,
};

use crate::player::components::Player;

use super::{
    components::RCSBooster,
    events::{RCSThrustPowerEvent, RCSThrustVectorEvent},
};

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
    mut entity_query: Query<(&RCSBooster, &mut ExternalForce)>,
) {
    for evt in thrust_vector_events.read() {
        // dbg!("EVENT: {} {}", evt.entity, evt.thrust_vector);
        if let Ok((booster, mut external_force)) = entity_query.get_mut(evt.entity) {
            let thrust_vector = evt.thrust_vector * booster.power_level;
            external_force.set_force(thrust_vector.as_dvec2());
            external_force.persistent = false;
        }
    }
}
