use bevy::{
    ecs::{event::Event, message::Message},
    transform::components::Transform,
};

#[derive(Event, Message)]
pub struct DamageIndicatorEvent {
    pub damage: f32,
    pub traslation: Transform,
}
