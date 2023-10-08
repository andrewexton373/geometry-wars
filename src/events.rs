use crate::hexbase::BuildingType;
use crate::recipe::Recipe;
use bevy::math::Vec2;
use bevy::prelude::Entity;

pub struct AblateEvent(pub Entity, pub Vec2, pub Vec2);

pub struct CraftEvent(pub Recipe);

pub struct LaserEvent(pub bool, pub Vec2, pub Vec2);

pub struct EnginePowerEvent(pub f32);

pub struct BuildHexBuildingEvent(pub Entity, pub BuildingType);
