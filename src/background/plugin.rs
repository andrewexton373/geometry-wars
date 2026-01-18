use bevy::app::{App, Plugin, Startup, Update};

use super::systems::{
    destroy_background_on_sector_remove, generate_background_on_sector_add, init_starfield,
    parallax_layers,
};

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_starfield)
            .add_systems(Update, parallax_layers)
            .add_observer(generate_background_on_sector_add)
            .add_observer(destroy_background_on_sector_remove);
    }
}
