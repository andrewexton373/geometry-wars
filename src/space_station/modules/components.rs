use bevy::ecs::component::Component;

#[derive(Component)]
pub struct SpaceStationModule;




#[derive(Component, Debug, Clone, Copy)]
pub enum SpaceStationModuleType {
    None,
    Factory,
    Refinery,
    Storage,
    Turret
}

// impl SpaceStationModuleTrait for SpaceStationModuleType {

// }

// #[bevy_trait_query::queryable]
// pub trait SpaceStationModuleTrait {


// }