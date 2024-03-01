use bevy::prelude::*;

#[derive(Component, Default)]
pub struct RCSBooster {
    pub power_level: f32,
}

impl RCSBooster {
    pub fn new() -> Self {
        RCSBooster { power_level: 100.0 }
    }

    pub fn set_power_level(&mut self, power_level: f32) {
        self.power_level = num::clamp(power_level, 0.0, 100.0);
    }

    pub fn delta_power_level(&mut self, delta: f32) {
        self.set_power_level(self.power_level + delta);
    }
}
