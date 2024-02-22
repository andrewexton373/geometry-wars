use bevy::prelude::*;

use super::resources::EmptyInventoryDepositTimer;
use crate::upgrades::UpgradeEvent;
use super::systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here

        app
            .insert_resource(EmptyInventoryDepositTimer(None))
            .add_event::<UpgradeEvent>()
            .add_systems(Startup, spawn_player)
            .add_systems(Update, (
                update_player_mass,
                player_movement.after(update_player_mass),
                ship_rotate_towards_mouse.after(player_movement),
                player_fire_laser.after(ship_rotate_towards_mouse),
                player_camera_control,
                player_deposit_control,
                gravitate_collectibles,
                trickle_charge,
                ship_battery_is_empty_context_clue,
                display_empty_ship_inventory_context_clue,
                on_upgrade_event
            ));
    }
}