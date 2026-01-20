use bevy::{
    ecs::{entity::Entity, event::Event},
    math::Vec2,
};

#[derive(Event)]
pub struct RCSThrustVectorEvent {
    pub entity: Entity,
    pub thrust_vector: Vec2,
}

#[derive(Event)]
pub struct RCSThrustSetPowerEvent(pub f32);
