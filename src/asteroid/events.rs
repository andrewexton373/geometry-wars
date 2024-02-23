use bevy::{
    math::Vec2,
    prelude::{Entity, Event},
};

#[derive(Event)]
pub struct AblateEvent(pub Entity, pub Vec2, pub Vec2);
