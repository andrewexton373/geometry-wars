use bevy::{
    ecs::{entity::Entity, event::Event, message::Message},
    math::Vec2,
};

#[derive(Event, Message)]
pub struct RCSThrustVectorEvent {
    pub entity: Entity,
    pub thrust_vector: Vec2,
}

#[derive(Event, Message)]
pub struct RCSThrustPowerEvent(pub f32);
