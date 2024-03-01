use bevy::ecs::{entity::Entity, event::Event};

use super::components::BuildingType;

#[derive(Event)]
pub struct BuildHexBuildingEvent(pub Entity, pub BuildingType);
