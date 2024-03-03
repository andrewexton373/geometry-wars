use bevy::app::{App, Plugin, Startup, Update};

use super::systems::{generate_visible_sectors, init_starfield, parallax_layers};

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (
                init_starfield,
            ))
            .add_systems(Update, (
               generate_visible_sectors,
                parallax_layers
            ));
    }
}