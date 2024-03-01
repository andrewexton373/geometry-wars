use bevy::prelude::*;

use super::systems::*;

pub struct CrosshairPlugin;

impl Plugin for CrosshairPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_crosshair, spawn_pointer));
        app.add_systems(PostUpdate, (draw_crosshair, update_pointer));
    }
}
