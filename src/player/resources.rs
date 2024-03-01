use bevy::prelude::{Resource, Timer};

#[derive(Resource)]
pub struct EmptyInventoryDepositTimer(pub Option<Timer>);
