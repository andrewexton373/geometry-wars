use bevy::ecs::component::Component;

#[derive(Component, Debug, Clone, Copy)]
pub enum BuildingType {
    None,
    Factory,
    Refinery,
    Storage,
}

#[derive(Component)]
pub struct Building(pub BuildingType);

// #[derive(Component)]
// struct HoveredHex;

#[derive(Component)]
pub struct HexTile;
