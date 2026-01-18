use avian2d::prelude::ConstantForce;
use bevy::{
    ecs::{message::MessageReader, system::Query},
    prelude::{Commands, Transform, With, Without},
};
use bevy_hanabi::prelude::*;

use crate::{
    battery::events::DrainBatteryEvent, particles::components::PlayerShipTrailParticles,
    player::components::Player, PIXELS_PER_METER,
};

use super::{
    components::RCSBooster,
    events::{RCSThrustPowerEvent, RCSThrustVectorEvent},
};

pub fn handle_set_thrust_power_events(
    mut engine_events: MessageReader<RCSThrustPowerEvent>,
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
    mut events: MessageReader<RCSThrustVectorEvent>,
    mut commands: Commands,
    mut entity_query: Query<
        (&RCSBooster, &Transform, &mut ConstantForce),
        (With<RCSBooster>, Without<PlayerShipTrailParticles>),
    >,
    mut engine_effect: Query<
        (
            &mut EffectProperties,
            // &mut Effect,
            &mut Transform,
        ),
        With<PlayerShipTrailParticles>,
    >,
) {
    for evt in events.read() {
        if let Ok((booster, transform, mut external_force)) = entity_query.get_mut(evt.entity) {
            let thrust_scale = 1000.0 * PIXELS_PER_METER as f32;
            let thrust_vector = evt.thrust_vector * booster.power_level * thrust_scale;
            *external_force = ConstantForce::new(thrust_vector.x.into(), thrust_vector.y.into());
            // external_force.set(thrust_vector.as_dvec2());
            // external_force.persistent = false;

            let energy_spent = thrust_vector.length() / 5000000.0; // TODO: magic number

            commands.trigger(DrainBatteryEvent {
                entity: evt.entity,
                drain: energy_spent,
            });

            // Note: On first frame where the effect spawns, EffectSpawner is spawned during
            // PostUpdate, so will not be available yet. Ignore for a frame if so.
            let Ok((
                mut properties,
                //  mut initializers,
                mut effect_transform,
            )) = engine_effect.single_mut()
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
            // initializers.reset();
        }
    }
}
