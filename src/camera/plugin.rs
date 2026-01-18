use avian2d::prelude::PhysicsSystems;
use bevy::{
    app::{App, Plugin, PostUpdate, Startup},
    color::{palettes::css::BLACK, Color},
    ecs::schedule::IntoScheduleConfigs,
    prelude::ClearColor,
    transform::TransformSystems,
};

use super::systems::{camera_follows_target, setup_camera};

pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(
                PostUpdate,
                camera_follows_target
                    .after(PhysicsSystems::Last)
                    .before(TransformSystems::Propagate),
            )
            .insert_resource(ClearColor(Color::from(BLACK)));
    }
}
