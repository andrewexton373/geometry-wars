use bevy::ecs::{entity::Entity, event::Event, message::Message};

use crate::space_station::modules::components::SpaceStationModuleType;

#[derive(Event, Message)]
pub struct BuildSpaceStationModuleEvent {
    pub entity: Entity,
    pub module_type: SpaceStationModuleType,
}
