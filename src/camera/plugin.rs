use bevy::{
    app::{App, Plugin, PostUpdate, Startup},
    ecs::schedule::IntoSystemConfigs,
    transform::TransformSystem,
};
use bevy_xpbd_2d::PhysicsSet;

use super::systems::{camera_follows_target, setup_camera};

pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera).add_systems(
            PostUpdate,
            camera_follows_target
                .after(PhysicsSet::Sync)
                .before(TransformSystem::TransformPropagate),
        );
    }
}
