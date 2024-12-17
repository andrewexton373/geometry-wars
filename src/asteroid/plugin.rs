use bevy::prelude::*;

use super::events::{AblateEvent, SpawnAsteroidEvent, SplitAsteroidEvent};
use super::resources::{AsteroidSpawner, InventoryFullNotificationTimer};
use super::systems::*;

pub struct AsteroidPlugin;

pub const LASER_DAMAGE: f32 = 250.0;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AsteroidSpawner {
            timer: Timer::from_seconds(1.0, TimerMode::Once),
        })
        .insert_resource(InventoryFullNotificationTimer(None))
        .add_event::<AblateEvent>()
        .add_event::<SpawnAsteroidEvent>()
        .add_event::<SplitAsteroidEvent>()
        .add_systems(PreUpdate, (tag_small_asteroids_as_collectible,))
        .add_systems(
            Update,
            (
                spawn_asteroids_aimed_at_ship,
                // spawn_asteroid_events,
                despawn_far_asteroids,
                handle_asteroid_collision_event,
                // ablate_asteroids_events,
                split_asteroids_over_split_ratio,
                split_asteroid_events,
                display_inventory_full_context_clue,
                update_collectible_material_color,
                handle_collectible_collision_event,
            ),
        )
        .add_observer(handle_spawn_asteroid_events)
        .add_observer(ablate_asteroids_events);

        
    }
}
