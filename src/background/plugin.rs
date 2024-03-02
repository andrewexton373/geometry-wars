use bevy::app::{App, Plugin, Startup, Update};

use super::systems::{init_starfield, update_visible_sectors};

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (
                init_starfield,
            ))
            .add_systems(Update, (
                update_visible_sectors
            ));
    }
}