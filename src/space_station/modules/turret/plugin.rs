use bevy::app::{App, Plugin, Update};

use super::systems::update_turret_weapons;

pub struct SpaceStationTurretPlugin;

impl Plugin for SpaceStationTurretPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_turret_weapons);
    }
}
