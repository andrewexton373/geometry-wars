use bevy::prelude::*;
use bevy_xpbd_2d::components::LinearVelocity;

use super::components::Collectible;

use crate::player::components::Player;


pub fn gravitate_collectibles_towards_player_ship(
    mut collectible_query: Query<(Entity, &Collectible, &Transform, &mut LinearVelocity)>,
    player_query: Query<(Entity, &Player, &Transform), With<Player>>,
) {
    const MAX_GRAVITATION_DISTANCE: f32 = 30.0 * crate::PIXELS_PER_METER;
    let (_player_ent, _player, player_transform) = player_query.single();

    for (_ent, _collectible, collectible_tranform, mut velocity) in
        collectible_query.iter_mut()
    {
        let distance_to_player_from_collectible = player_transform
            .translation
            .truncate()
            .distance(collectible_tranform.translation.truncate());
        if distance_to_player_from_collectible < MAX_GRAVITATION_DISTANCE {
            let percent_distance_from_max =
                distance_to_player_from_collectible / MAX_GRAVITATION_DISTANCE;
            let direction_to_player_from_collectible =
                (player_transform.translation.truncate()
                    - collectible_tranform.translation.truncate())
                .normalize();
            let gravitation_factor = 1.0 - percent_distance_from_max;
            velocity.0 += direction_to_player_from_collectible
                * gravitation_factor
                * 5.0
                * crate::PIXELS_PER_METER;
        }
    }
}