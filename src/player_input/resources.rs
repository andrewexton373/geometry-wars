use bevy::prelude::Resource;
use bevy::math::Vec2;

#[derive(Resource)]
pub struct MouseWorldPosition(pub(crate) Vec2);

#[derive(Resource)]
pub struct MouseScreenPosition(pub(crate) Vec2);