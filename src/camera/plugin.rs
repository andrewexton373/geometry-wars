use avian2d::prelude::PhysicsSet;
use bevy::{
    app::{App, Plugin, PostUpdate, Startup}, color::{palettes::css::BLACK, Color}, ecs::schedule::IntoSystemConfigs, prelude::ClearColor, transform::TransformSystem
};

use super::systems::{camera_follows_target, setup_camera};

pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera).add_systems(
            PostUpdate,
            camera_follows_target
                .after(PhysicsSet::Sync)
                .before(TransformSystem::TransformPropagate),
        )
        .insert_resource(ClearColor(Color::from(BLACK)));
    }
}
