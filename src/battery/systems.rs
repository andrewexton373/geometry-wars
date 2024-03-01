use bevy::prelude::{EventReader, Query};

use super::components::Battery;
use super::events::{ChargeBatteryEvent, DrainBatteryEvent};

pub fn handle_drain_battery_events(
    mut battery_q: Query<&mut Battery>,
    mut events: EventReader<DrainBatteryEvent>,
) {
    for evt in events.read() {
        if let Ok(mut battery) = battery_q.get_mut(evt.entity) {
            battery.drain_battery(evt.drain);
        }
    }
}

pub fn handle_charge_battery_events(
    mut battery_q: Query<&mut Battery>,
    mut events: EventReader<ChargeBatteryEvent>,
) {
    for evt in events.read() {
        if let Ok(mut battery) = battery_q.get_mut(evt.entity) {
            battery.charge_battery(evt.charge);
        }
    }
}
