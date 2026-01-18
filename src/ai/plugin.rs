use bevy::{
    app::{App, Plugin, Update},
    time::{Timer, TimerMode},
};

use super::{
    resources::EnemySpawnTimer,
    systems::{despawn_dead_enemies, spawn_enemies},
};

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .insert_resource(EnemySpawnTimer {
                timer: Timer::from_seconds(100.0, TimerMode::Repeating),
            })
            .add_systems(Update, (spawn_enemies, despawn_dead_enemies));
    }
}
