use bevy::ecs::component::Component;

#[derive(Component)]
pub struct StarfieldBackground;

#[derive(Component, Debug, Copy, Clone)]
pub struct Sector {
    pub i: i32,
    pub j: i32,
    pub sector_size: f32,
}

#[derive(Component, Debug, Copy, Clone)]
pub struct Layer(pub u8);
