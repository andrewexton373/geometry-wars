use bevy::prelude::*;

use bevy_tweening::TweeningPlugin;

use super::events::AblateEvent;
use super::resources::{AsteroidSpawner, InventoryFullNotificationTimer};
use super::systems::*;

pub struct AsteroidPlugin;

pub const LASER_DAMAGE: f32 = 250.0;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TweeningPlugin)
            .insert_resource(AsteroidSpawner {
                timer: Timer::from_seconds(1.0, TimerMode::Once),
            })
            .insert_resource(InventoryFullNotificationTimer(None))
            .add_event::<AblateEvent>()
            .add_systems(
                Update,
                (
                    spawn_asteroids_aimed_at_ship,
                    despawn_far_asteroids,
                    handle_asteroid_collision_event,
                    ablate_asteroids,
                    split_asteroids_over_split_ratio,
                    remove_post_animation_text,
                    display_inventory_full_context_clue,
                    update_collectible_material_color,
                ),
            );
    }
}
