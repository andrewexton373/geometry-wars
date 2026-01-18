use bevy::{
    ecs::message::Message,
    prelude::{Entity, Event},
};

#[derive(Event, Message)]
pub struct DrainBatteryEvent {
    pub entity: Entity,
    pub drain: f32,
}

#[derive(Event, Message)]
pub struct ChargeBatteryEvent {
    pub entity: Entity,
    pub charge: f32,
}
