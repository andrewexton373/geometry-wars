use bevy::prelude::Component;

use crate::battery::Battery;
use crate::engine::Engine;
use crate::health::Health;

#[derive(Component, Default)]
pub struct Player {
    pub health: Health,
    pub battery: Battery,
    pub engine: Engine,
}

impl Player {
    pub fn new() -> Player {
        Player {
            health: Health::new(),
            battery: Battery::new(),
            engine: Engine::new(), // upgrades: UpgradesComponent::new()
        }
    }

    // TODO: Should probably use components and systems for these "effects", use a component for Damage(DamageValue), and respective systems to watch and apply to parent health, etc.
    // TODO Move these to health and battery respectively...
    pub fn take_damage(&mut self, damage: f32) {
        let modified_health = self.health.current() - damage;
        self.health.set_current(modified_health);
    }

    pub fn repair_damage(&mut self, amount: f32) {
        let updated_health = self.health.current() + amount;
        self.health.set_current(updated_health);
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
