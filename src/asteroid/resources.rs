use bevy::{ecs::resource::Resource, time::Timer};

#[derive(Resource)]
pub struct InventoryFullNotificationTimer(pub Option<Timer>);

#[derive(Resource)]
pub struct AsteroidSpawner {
    pub timer: Timer,
}
