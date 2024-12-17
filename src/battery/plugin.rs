use bevy::prelude::*;

use super::events::{ChargeBatteryEvent, DrainBatteryEvent};
use super::systems::{handle_charge_battery_events, handle_drain_battery_events};

pub struct BatteryPlugin;

impl Plugin for BatteryPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ChargeBatteryEvent>()
            .add_event::<DrainBatteryEvent>()
            .add_observer(handle_charge_battery_events)
            .add_observer(handle_drain_battery_events);
    }
}
