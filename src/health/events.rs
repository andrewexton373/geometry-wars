use bevy::{
    ecs::message::Message,
    prelude::{Entity, Event},
};

#[derive(Event, Message)]
pub struct DamageEvent {
    pub entity: Entity,
    pub damage: f32,
}

#[derive(Event, Message)]
pub struct RepairEvent {
    pub entity: Entity,
    pub repair: f32,
}
