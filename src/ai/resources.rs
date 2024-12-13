use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}
