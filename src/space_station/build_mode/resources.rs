use bevy::{asset::Handle, ecs::system::Resource, sprite::ColorMaterial};

#[derive(Resource, Default)]
pub struct BuildModeMaterials {
    pub buildable_hex_material: Handle<ColorMaterial>,
    pub mouse_hover_hex_material: Handle<ColorMaterial>,
    pub selected_hex_material: Handle<ColorMaterial>
}