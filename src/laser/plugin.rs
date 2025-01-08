use crate::player::systems::ship_rotate_towards_mouse;
use crate::player_input::systems::player_movement_input_handling;

use super::events::LaserEvent;
use super::systems::{fire_laser_raycasting, setup_laser};
use bevy::prelude::*;

pub struct LaserPlugin;

impl Plugin for LaserPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LaserEvent>()
            .add_systems(Startup, setup_laser)
            .add_systems(
                Update,
                fire_laser_raycasting
                    .after(player_movement_input_handling)
                    .after(ship_rotate_towards_mouse),
            );
    }
}
