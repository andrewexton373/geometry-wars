use bevy::ecs::{entity::Entity, system::Resource};

#[derive(Resource, Default)]
pub struct MouseHoverContext(pub Option<Entity>);