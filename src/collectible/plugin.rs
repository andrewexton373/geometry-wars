use bevy::prelude::*;
use bevy_tweening::TweeningPlugin;

use super::systems::gravitate_collectibles_towards_player_ship;

pub struct CollectiblesPlugin;

impl Plugin for CollectiblesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TweeningPlugin)
            .add_systems(Update, (gravitate_collectibles_towards_player_ship));
    }
}
