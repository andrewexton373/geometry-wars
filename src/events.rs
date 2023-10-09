use crate::hexbase::BuildingType;
use crate::recipe::Recipe;
use bevy::math::Vec2;
use bevy::prelude::Entity;
use bevy::prelude::Event;

#[derive(Event)]
pub struct AblateEvent(pub Entity, pub Vec2, pub Vec2);

#[derive(Event)]
pub struct CraftEvent(pub Recipe);

#[derive(Event)]
pub struct LaserEvent(pub bool, pub Vec2, pub Vec2);

#[derive(Event)]
pub struct EnginePowerEvent(pub f32);

#[derive(Event)]
pub struct BuildHexBuildingEvent(pub Entity, pub BuildingType);
