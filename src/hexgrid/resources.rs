use bevy::{
    asset::Handle,
    ecs::{entity::Entity, system::Resource},
    sprite::ColorMaterial,
    utils::HashMap,
};
use hexx::{Hex, HexLayout};

use super::components::BuildingType;

#[derive(Debug, Resource)]
pub struct HexGridMap {
    pub layout: HexLayout,
    pub entities: HashMap<Hex, Entity>,
    pub selected_material: Handle<ColorMaterial>,
    pub mouse_hover_material: Handle<ColorMaterial>,
    pub ship_hover_material: Handle<ColorMaterial>,
    pub ring_material: Handle<ColorMaterial>,
    pub default_material: Handle<ColorMaterial>,
    pub factory_material: Handle<ColorMaterial>,
    pub refinery_material: Handle<ColorMaterial>,
    pub storage_material: Handle<ColorMaterial>,
}

#[derive(Debug, Default, Resource)]
pub struct HighlightedHexes {
    pub selected: Hex,
    pub ship_hover: Hex,
    pub ring: Vec<Hex>,
    pub line: Vec<Hex>,
}

#[derive(Resource, Default)]
pub struct PlayerHoveringBuilding(pub(crate) Option<(Entity, BuildingType)>);


#[derive(Resource, Default, Debug)]
pub struct SelectedHex {
    pub selected_hex: Option<Hex>,
    pub entity: Option<Entity>
}

#[derive(Resource, Default)]
pub struct MouseHoverHex {
    pub hover_hex: Option<Hex>,
    pub entity: Option<Entity>
}