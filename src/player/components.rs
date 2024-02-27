use bevy::prelude::Component;

use crate::battery::Battery;
use crate::engine::Engine;
use crate::health::components::Health;

#[derive(Component, Default)]
pub struct Player {
    pub battery: Battery,
    pub engine: Engine,
}

impl Player {
    pub fn new() -> Player {
        Player {
            battery: Battery::new(),
            engine: Engine::new(), // upgrades: UpgradesComponent::new()
        }
    }

    pub fn drain_battery(&mut self, amount: f32) {
        let updated_capacity = self.battery.current() - amount;
        self.battery.set_current(updated_capacity);
    }

    pub fn charge_battery(&mut self, amount: f32) {
        let updated_capacity = self.battery.current() + amount;
        self.battery.set_current(updated_capacity);
    }
}
