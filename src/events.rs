use crate::recipe::Recipe;
use bevy::prelude::Entity;
use bevy::prelude::Event;

#[derive(Event)]
pub struct CraftEvent(pub Recipe);
