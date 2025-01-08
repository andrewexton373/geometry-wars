use bevy::prelude::*;

use crate::player_input::systems::player_movement_input_handling;

use super::resources::EmptyInventoryDepositTimer;
use super::systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here

        app.insert_resource(EmptyInventoryDepositTimer(None))
            .add_systems(Startup, spawn_player)
            .add_systems(
                Update,
                (
                    update_player_mass,
                    ship_rotate_towards_mouse.after(player_movement_input_handling),
                    player_fire_laser.after(ship_rotate_towards_mouse),
                    display_empty_ship_inventory_context_clue,
                    on_upgrade_event,
                ),
            );
    }
}
