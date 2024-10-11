pub const SPACE_STATION_SIZE: f32 = 20.0;

use bevy::{
    asset::Handle,
    ecs::entity::Entity,
    prelude::Resource,
    sprite::ColorMaterial,
};

use crate::hexgrid::components::BuildingType;

#[derive(Resource)]
pub struct CanDeposit(pub bool);

#[derive(Resource, Default)]
pub struct PlayerHoveringSpaceStationModule(pub(crate) Option<(Entity, BuildingType)>);

#[derive(Resource)]
pub struct SpaceStationModuleMaterialMap {
    pub core_material: Handle<ColorMaterial>,
    pub fabrication_material: Handle<ColorMaterial>,
    pub storage_material: Handle<ColorMaterial>,
    pub turret_material: Handle<ColorMaterial>,
    pub buildable_material: Handle<ColorMaterial>,
}
