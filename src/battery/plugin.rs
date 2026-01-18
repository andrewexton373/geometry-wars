use bevy::prelude::*;

use super::events::{ChargeBatteryEvent, DrainBatteryEvent};
use super::systems::{
    handle_charge_battery_events, handle_drain_battery_events, ship_battery_is_empty_context_clue,
    trickle_charge,
};

pub struct BatteryPlugin;

impl Plugin for BatteryPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ChargeBatteryEvent>()
            .add_message::<DrainBatteryEvent>()
            .add_systems(
                Update,
                (
                    ship_battery_is_empty_context_clue,
                    trickle_charge,
                    handle_charge_battery_events,
                    handle_drain_battery_events,
                ),
            );
    }
}
