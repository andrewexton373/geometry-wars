use bevy::prelude::{App, Plugin, Update};

use super::{
    events::{DamageEvent, RepairEvent},
    systems::{handle_damage_events, handle_repair_events},
};

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageEvent>()
            .add_event::<RepairEvent>()
            .add_systems(Update, (handle_damage_events, handle_repair_events));
    }
}
