use bevy::{ecs::event::Event, transform::components::Transform};

#[derive(Event)]
pub struct DamageIndicatorEvent {
    pub damage: f32,
    pub traslation: Transform,
}
