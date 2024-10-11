use bevy::{
    app::{App, Plugin, PreUpdate, Update},
    ecs::schedule::IntoSystemConfigs, time::{Timer, TimerMode},
};
use big_brain::{BigBrainPlugin, BigBrainSet};

use super::{resources::EnemySpawnTimer, systems::{
    attack_action_system, despawn_dead_enemies, hostility_scorer_system, hostility_system,
    move_towards_player_action_system, spawn_enemies,
}};

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<EnemySpawnTimer>()
            .insert_resource(EnemySpawnTimer {
                timer: Timer::from_seconds(100.0, TimerMode::Repeating)
            })
            .add_plugins(BigBrainPlugin::new(PreUpdate))
            .add_systems(Update, (hostility_system, spawn_enemies, despawn_dead_enemies))
            .add_systems(
                PreUpdate,
                (
                    attack_action_system,
                    hostility_scorer_system,
                    move_towards_player_action_system,
                )
                    .in_set(BigBrainSet::Actions),
            );
    }
}
