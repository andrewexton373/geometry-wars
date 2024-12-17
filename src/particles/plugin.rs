use bevy::app::{App, Plugin, Startup, Update};

use super::systems::{setup_player_ship_trail_particle_system, 
    setup_projectile_impact_particle_system};

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app 
            .add_observer(setup_projectile_impact_particle_system)
            .add_observer(setup_player_ship_trail_particle_system);
        // .add_systems(Update, setup_player_ship_trail_particle_system);
    }
}
