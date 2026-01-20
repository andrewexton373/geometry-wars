use bevy::prelude::*;

use crate::battery::events::{BatteryEvent, BatteryEventType};
use crate::player::components::Player;
use crate::ui::context_clue::resources::{ContextClue, ContextClues};

use super::components::Battery;

pub fn on_battery_event(trigger: On<BatteryEvent>, mut battery_q: Query<&mut Battery>) {
    if let Ok(mut battery) = battery_q.get_mut(trigger.entity) {
        match trigger.event_type {
            BatteryEventType::Drain => {
                battery.drain_battery(trigger.amount);
            }
            BatteryEventType::Charge => {
                battery.charge_battery(trigger.amount);
            }
        }
    }
}

pub fn trickle_charge(
    mut commands: Commands,
    battery: Single<Entity, (With<Player>, With<Battery>)>,
) {
    let entity = battery.entity();
    commands.trigger(BatteryEvent {
        entity,
        event_type: BatteryEventType::Charge,
        amount: 0.01,
    });
}

pub fn ship_battery_is_empty_context_clue(
    mut context_clues_res: ResMut<ContextClues>,
    battery: Single<&Battery, With<Player>>,
) {
    if battery.is_empty() {
        context_clues_res.0.insert(ContextClue::ShipFuelEmpty);
    } else {
        context_clues_res.0.remove(&ContextClue::ShipFuelEmpty);
    }
}
