use bevy::app::{App, Plugin, Startup, Update};

use super::systems::{camera_follows_target, setup_camera};

pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_camera)
            .add_systems(Update, (
                camera_follows_target
            ));
    }
}