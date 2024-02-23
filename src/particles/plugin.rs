use bevy::app::{Plugin, App, Startup, Update};

use super::systems::{
    setup_projectile_impact_particle_system,
    setup_ship_asteroid_impact_particle_system,
    setup_player_ship_trail_particle_system
};

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                setup_projectile_impact_particle_system,
                setup_ship_asteroid_impact_particle_system,
            ),
        )
        .add_systems(Update, setup_player_ship_trail_particle_system);
    }
}