use bevy::prelude::*;

use super::{events::FireProjectileEvent, systems::{handle_fire_projectile_events, handle_projectile_collision_event}};

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<FireProjectileEvent>()
            .add_systems(Update, (
                handle_projectile_collision_event,
                handle_fire_projectile_events
            ));
    }
}