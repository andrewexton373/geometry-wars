use bevy::prelude::*;

use crate::player::components::Player;
use crate::ui::context_clue::resources::{ContextClue, ContextClues};

use super::components::Battery;
use super::events::{ChargeBatteryEvent, DrainBatteryEvent};

pub fn handle_drain_battery_events(
    mut events: EventReader<DrainBatteryEvent>,
    mut battery_q: Query<&mut Battery>,
) {
    for evt in events.read() {
        if let Ok(mut battery) = battery_q.get_mut(evt.entity) {
            battery.drain_battery(evt.drain);
        }
    }
}

pub fn handle_charge_battery_events(
    mut events: EventReader<ChargeBatteryEvent>,
    mut battery_q: Query<&mut Battery>,
) {
    for evt in events.read() {
        if let Ok(mut battery) = battery_q.get_mut(evt.entity) {
            battery.charge_battery(evt.charge);
        }
    }
}

pub fn trickle_charge(
    mut commands: Commands,
    battery: Query<Entity, (With<Player>, With<Battery>)>,
) {
    if let Ok(entity) = battery.get_single() {
        commands.send_event(ChargeBatteryEvent {
            entity,
            charge: 0.01,
        });
    }
}

pub fn ship_battery_is_empty_context_clue(
    mut context_clues_res: ResMut<ContextClues>,
    mut battery_q: Query<&Battery, With<Player>>,
) {
    if let Ok(battery) = battery_q.get_single_mut() {
        if battery.is_empty() {
            context_clues_res.0.insert(ContextClue::ShipFuelEmpty);
        } else {
            context_clues_res.0.remove(&ContextClue::ShipFuelEmpty);
        }
    }
}
