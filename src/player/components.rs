use bevy::prelude::Component;

// use crate::battery::components::Battery;
use crate::engine::Engine;
// use crate::health::components::Health;

#[derive(Component, Default)]
pub struct Player {
    // pub battery: Battery,
    pub engine: Engine,
}

impl Player {
    pub fn new() -> Player {
        Player {
            // battery: Battery::new(),
            engine: Engine::new(), // upgrades: UpgradesComponent::new()
        }
    }
}
