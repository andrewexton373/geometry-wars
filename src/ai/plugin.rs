use bevy::{app::{App, Plugin, PreUpdate, Startup, Update}, ecs::schedule::IntoSystemConfigs};
use big_brain::{BigBrainPlugin, BigBrainSet};

use super::systems::{attack_action_system, hostility_scorer_system, hostility_system, init_entities, move_towards_player_action_system};

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(BigBrainPlugin::new(PreUpdate))
            .add_systems(Startup, init_entities)
            .add_systems(Update, hostility_system)
            .add_systems(PreUpdate, (
                attack_action_system,
                hostility_scorer_system,
                move_towards_player_action_system
            ).in_set(BigBrainSet::Actions));
    }
}