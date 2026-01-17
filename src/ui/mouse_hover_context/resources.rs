use bevy::ecs::{entity::Entity, resource::Resource};

#[derive(Resource, Default)]
pub struct MouseHoverContext(pub Option<Entity>);
