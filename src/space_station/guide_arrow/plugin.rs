use bevy::prelude::*;

use super::systems::{guide_player_to_space_station, spawn_player_base_guide_arrow};

pub struct GuideArrowPlugin;

impl Plugin for GuideArrowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_player_base_guide_arrow))
            .add_systems(Update, (guide_player_to_space_station,));
    }
}
