pub const SPACE_STATION_SIZE: f32 = 20.0;

use bevy::prelude::Resource;

#[derive(Resource)]
pub struct CanDeposit(pub bool);
