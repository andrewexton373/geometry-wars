use bevy::ecs::{entity::Entity, event::Event};

use crate::space_station::modules::components::SpaceStationModuleType;


#[derive(Event)]
pub struct BuildSpaceStationModuleEvent {
    entity: Entity,
    module_type: SpaceStationModuleType
}