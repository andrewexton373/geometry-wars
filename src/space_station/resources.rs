pub const SPACE_STATION_SIZE: f32 = 20.0;

use bevy::{ecs::{component::Component, entity::Entity}, prelude::Resource};

use crate::hexbase::BuildingType;

#[derive(Resource)]
pub struct CanDeposit(pub bool);

#[derive(Resource, Default)]
pub struct PlayerHoveringSpaceStationModule(pub(crate) Option<(Entity, BuildingType)>);