use bevy::math::Vec2;
use bevy::prelude::Resource;

#[derive(Resource)]
pub struct MouseWorldPosition(pub(crate) Vec2);

#[derive(Resource)]
pub struct MouseScreenPosition(pub(crate) Vec2);
