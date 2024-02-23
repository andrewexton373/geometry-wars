use bevy::{ecs::system::Resource, time::Timer};

#[derive(Resource)]
pub struct InventoryFullNotificationTimer(pub Option<Timer>);

#[derive(Resource)]
pub struct AsteroidSpawner {
    pub timer: Timer,
}
