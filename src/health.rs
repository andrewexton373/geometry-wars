// use crate::{widgets::station_menu::UpgradeLevel, upgrades::Upgradeable};
use bevy::prelude::*;

use crate::upgrades::components::{UpgradeLevel, Upgradeable};

#[derive(Component, Default, Clone, Debug)]
pub struct Health {
    pub current: f32,
    pub maximum: f32,
    pub upgrade_level: UpgradeLevel,
}

impl Health {
    pub fn new() -> Self {
        Self {
            current: 100.0,
            maximum: 100.0,
            upgrade_level: UpgradeLevel::Level0,
        }
    }

    pub fn current(&self) -> f32 {
        self.current
    }

    pub fn current_percent(&self) -> f32 {
        self.current / self.maximum
    }

    pub fn set_current(&mut self, updated: f32) {
        self.current = updated.clamp(0.0, self.maximum());
    }

    pub fn maximum(&self) -> f32 {
        self.maximum * self.upgrade_effect()
    }
}

impl Upgradeable for Health {
    fn set_upgrade_level(&mut self, upgrade_level: UpgradeLevel) {
        self.upgrade_level = upgrade_level;
    }

    fn upgrade_effect(&self) -> f32 {
        match self.upgrade_level {
            UpgradeLevel::Level0 => 1.0,
            UpgradeLevel::Level1 => 1.5,
            UpgradeLevel::Level2 => 2.0,
            UpgradeLevel::Level3 => 3.0,
            UpgradeLevel::Level4 => 4.0,
            UpgradeLevel::MaxLevel => 5.0,
        }
    }
}
