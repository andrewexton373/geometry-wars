use crate::hexbase::BuildingType;
use crate::recipe::Recipe;
use bevy::prelude::Entity;
use bevy::prelude::Event;

#[derive(Event)]
pub struct CraftEvent(pub Recipe);

#[derive(Event)]
pub struct BuildHexBuildingEvent(pub Entity, pub BuildingType);
