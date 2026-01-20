use bevy::prelude::{Entity, Event};

#[derive(Event)]
pub struct BatteryEvent {
    pub entity: Entity,
    pub event_type: BatteryEventType,
    pub amount: f32,
}

pub enum BatteryEventType {
    Drain,
    Charge,
}
