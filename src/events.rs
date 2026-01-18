use crate::recipe::Recipe;
use bevy::{ecs::message::Message, prelude::Event};

#[derive(Event, Message)]
pub struct CraftEvent(pub Recipe);
