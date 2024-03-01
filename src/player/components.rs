use bevy::prelude::Component;

use crate::rcs::components::RCSBooster;

// use crate::battery::components::Battery;
// use crate::health::components::Health;

#[derive(Component, Default)]
pub struct Player {
    // pub battery: Battery,
    pub rcs_booster: RCSBooster,
}

impl Player {
    pub fn new() -> Player {
        Player {
            // battery: Battery::new(),
            rcs_booster: RCSBooster::new(), // upgrades: UpgradesComponent::new()
        }
    }
}
