use bevy::ecs::{entity::Entity, event::Event, message::Message};

use super::components::BuildingType;

#[derive(Event, Message)]
pub struct BuildHexBuildingEvent(pub Entity, pub BuildingType);
