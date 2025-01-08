use bevy::{
    core::Name,
    ecs::{
        query::{With, Without},
        system::{Commands, Query},
    },
    prelude::Camera2d,
    render::camera::Camera,
    transform::components::Transform,
};

use super::components::{CameraTarget, GameCamera};

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((Name::new("GameCamera"), GameCamera, Camera2d));
}

pub fn camera_follows_target(
    mut camera_query: Query<(&Camera, &mut Transform), With<GameCamera>>,
    target_query: Query<&Transform, (With<CameraTarget>, Without<GameCamera>)>,
) {
    let (_camera, mut camera_trans) = camera_query.single_mut();
    for target_t in target_query.iter() {
        camera_trans.translation.x = target_t.translation.x;
        camera_trans.translation.y = target_t.translation.y;
    }
}
