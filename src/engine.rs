use bevy::prelude::*;

use crate::events::EnginePowerEvent;
use crate::player::components::Player;

pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, Self::handle_player_input_events);
    }
}

impl EnginePlugin {
    pub fn handle_player_input_events(
        mut engine_events: EventReader<EnginePowerEvent>,
        mut player_query: Query<&mut Player>,
    ) {
        for mut player in player_query.iter_mut() {
            for event in engine_events.read() {
                let delta = event.0;
                player.engine.delta_power_level(delta);
            }
        }
    }
}

#[derive(Component, Default)]
pub struct Engine {
    pub power_level: f32,
}

impl Engine {
    pub fn new() -> Self {
        Engine { power_level: 100.0 }
    }

    pub fn set_power_level(&mut self, power_level: f32) {
        self.power_level = num::clamp(power_level, 0.0, 100.0);
    }

    pub fn delta_power_level(&mut self, delta: f32) {
        self.set_power_level(self.power_level + delta);
    }
}
