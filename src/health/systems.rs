use bevy::{ecs::system::Query, prelude::EventReader};

use super::{
    components::Health,
    events::{DamageEvent, RepairEvent},
};

pub fn handle_damage_events(
    mut damage_events: EventReader<DamageEvent>,
    mut entity_q: Query<&mut Health>,
) {
    for evt in damage_events.read() {
        if let Ok(mut health) = entity_q.get_mut(evt.entity) {
            health.take_damage(evt.damage);
        }
    }
}

pub fn handle_repair_events(
    mut damage_events: EventReader<RepairEvent>,
    mut entity_q: Query<&mut Health>,
) {
    for evt in damage_events.read() {
        if let Ok(mut health) = entity_q.get_mut(evt.entity) {
            health.repair_damage(evt.repair);
        }
    }
}
