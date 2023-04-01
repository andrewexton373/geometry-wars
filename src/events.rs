use bevy::prelude::Entity;
use bevy::math::Vec2;
use crate::hexbase::BuildingType;
use crate::recipe::Recipe;

pub struct AblateEvent(pub Entity, pub Vec2, pub Vec2);

pub struct CraftEvent(pub Recipe);

pub struct LaserEvent(pub bool, pub Vec2, pub Vec2);

pub struct EnginePowerEvent(pub f32);

pub struct BuildHexBuildingEvent(pub Entity, pub BuildingType);