use bevy::prelude::*;

use super::{
    events::{RCSThrustPowerEvent, RCSThrustVectorEvent},
    systems::{handle_set_thrust_power_events, handle_thrust_events},
};

pub struct RCSPlugin;

impl Plugin for RCSPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RCSThrustPowerEvent>()
            .add_event::<RCSThrustVectorEvent>()
            .add_systems(
                Update,
                (handle_set_thrust_power_events),
            )
            .add_observer(handle_thrust_events);
    }
}
