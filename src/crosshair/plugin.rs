use bevy::prelude::*;

use super::systems::*;

pub struct CrosshairPlugin;

impl Plugin for CrosshairPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_gizmo_group::<CrosshairGizmos>()
            .add_systems(Startup, crosshair_gizmo_config)
            .add_systems(PostUpdate, draw_crosshair);
    }
}
