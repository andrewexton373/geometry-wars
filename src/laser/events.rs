use bevy::ecs::message::Message;
use bevy::math::Vec2;
use bevy::prelude::Event;

#[derive(Event, Message)]
pub struct LaserEvent(pub bool, pub Vec2, pub Vec2);
