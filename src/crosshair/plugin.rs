use bevy::prelude::*;

use super::systems::*;

pub struct CrosshairPlugin;

impl Plugin for CrosshairPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_crosshair);
        app.add_systems(Update, draw_crosshair);
    }
}
