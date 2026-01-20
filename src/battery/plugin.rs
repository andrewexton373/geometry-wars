use bevy::prelude::*;

use crate::battery::systems::on_battery_event;

use super::systems::{ship_battery_is_empty_context_clue, trickle_charge};

pub struct BatteryPlugin;

impl Plugin for BatteryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (ship_battery_is_empty_context_clue, trickle_charge))
            .add_observer(on_battery_event);
    }
}
