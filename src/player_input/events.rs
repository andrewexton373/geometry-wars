use bevy::prelude::Event;

#[derive(Event)]
pub struct EnginePowerEvent(pub f32);