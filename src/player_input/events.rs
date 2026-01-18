use bevy::{ecs::message::Message, prelude::Event};

#[derive(Event, Message)]
pub struct DepositInventoryEvent;
