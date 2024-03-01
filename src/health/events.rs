use bevy::prelude::{Entity, Event};

#[derive(Event)]
pub struct DamageEvent {
    pub entity: Entity,
    pub damage: f32,
}

#[derive(Event)]
pub struct RepairEvent {
    pub entity: Entity,
    pub repair: f32,
}
