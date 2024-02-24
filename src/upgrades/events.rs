use bevy::prelude::Event;

use super::components::UpgradeType;

#[derive(Event)]
pub struct UpgradeEvent(pub UpgradeType);