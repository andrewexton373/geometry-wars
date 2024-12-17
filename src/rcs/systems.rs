use avian2d::prelude::ExternalForce;
use bevy::{ecs::{event::EventReader, system::Query}, log::info, prelude::{Transform, With, Without}};
use bevy_hanabi::prelude::*;

use crate::{particles::components::PlayerShipTrailParticles, player::components::Player};

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
    mut entity_query: Query<(&RCSBooster, &Transform, &mut ExternalForce), (With<RCSBooster>, Without<PlayerShipTrailParticles>)>,
    mut engine_effect: Query<
        (
            &mut EffectProperties,
            &mut EffectInitializers,
            &mut Transform,
        ),
        With<PlayerShipTrailParticles>
    >,
) {
    for evt in thrust_vector_events.read() {
        // dbg!("EVENT: {} {}", evt.entity, evt.thrust_vector);
        if let Ok((booster, transform, mut external_force)) = entity_query.get_mut(evt.entity) {
            let thrust_vector = evt.thrust_vector * booster.power_level;
            external_force.set_force(thrust_vector.as_dvec2());
            external_force.persistent = false;

            // Note: On first frame where the effect spawns, EffectSpawner is spawned during
            // PostUpdate, so will not be available yet. Ignore for a frame if so.
            let Ok((mut properties, mut initializers, mut effect_transform)) = engine_effect.get_single_mut()
            else {
                return;
            };

            // This isn't the most accurate place to spawn the particle effect,
            // but this is just for demonstration, so whatever.
            effect_transform.translation = transform.translation;

            // Set the collision normal
            let normal = -thrust_vector.normalize();
            // info!("Thrust: n={:?}", thrust_vector);
            properties.set("thrust_vector", normal.extend(0.).into());

            // Spawn the particles
            initializers.reset();

        }
    }
}
