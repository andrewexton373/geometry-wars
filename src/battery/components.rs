use bevy::prelude::*;

use crate::upgrades::components::{UpgradeLevel, Upgradeable};

// use crate::{widgets::station_menu::UpgradeLevel, upgrades::Upgradeable};

#[derive(Component, Default, Clone, Debug)]
pub struct Battery {
    _current_capacity: f32,
    _maximum_capacity: f32,
    _upgrade_level: UpgradeLevel,
}

impl Battery {
    pub fn new() -> Self {
        Self {
            _current_capacity: 1000.0,
            _maximum_capacity: 1000.0,
            _upgrade_level: UpgradeLevel::Level0,
        }
    }

    pub fn drain_battery(&mut self, amount: f32) {
        self.set_current(self.current() - amount);
    }

    pub fn charge_battery(&mut self, amount: f32) {
        self.set_current(self.current() + amount);
    }

    pub fn current(&self) -> f32 {
        self._current_capacity
    }

    pub fn set_current(&mut self, updated: f32) {
        self._current_capacity = updated.clamp(0.0, self.maximum());
    }

    pub fn maximum(&self) -> f32 {
        self._maximum_capacity * self.upgrade_effect()
    }

    pub fn is_empty(&self) -> bool {
        self._current_capacity <= 0.0
    }
}

impl Upgradeable for Battery {
    fn set_upgrade_level(&mut self, upgrade_level: UpgradeLevel) {
        self._upgrade_level = upgrade_level;
    }

    fn upgrade_effect(&self) -> f32 {
        match self._upgrade_level {
            UpgradeLevel::Level0 => 1.0,
            UpgradeLevel::Level1 => 1.5,
            UpgradeLevel::Level2 => 2.0,
            UpgradeLevel::Level3 => 3.0,
            UpgradeLevel::Level4 => 4.0,
            UpgradeLevel::MaxLevel => 5.0,
        }
    }
}
