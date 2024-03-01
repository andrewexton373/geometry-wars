use bevy::prelude::{Entity, Event};

#[derive(Event)]
pub struct DrainBatteryEvent {
    pub entity: Entity,
    pub drain: f32,
}

#[derive(Event)]
pub struct ChargeBatteryEvent {
    pub entity: Entity,
    pub charge: f32,
}
