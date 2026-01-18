use bevy::{ecs::message::Message, prelude::Event};

use super::components::UpgradeType;

#[derive(Event, Message)]
pub struct UpgradeEvent(pub UpgradeType);
