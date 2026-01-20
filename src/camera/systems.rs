use avian2d::prelude::{LinearVelocity, RigidBody, Sensor};
use bevy::{
    camera::Camera,
    ecs::{
        name::Name,
        query::{With, Without},
        system::{Commands, Query},
    },
    prelude::Camera2d,
    transform::components::Transform,
};

use super::components::{CameraTarget, GameCamera};

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("GameCamera"),
        GameCamera,
        RigidBody::Dynamic,
        Sensor,
        LinearVelocity::default(),
        Camera2d,
    ));
}

pub fn camera_follows_target(
    mut camera_query: Query<(&Camera, &mut Transform, &mut LinearVelocity), With<GameCamera>>,
    target_query: Query<&Transform, (With<CameraTarget>, Without<GameCamera>)>,
) {
    let (_camera, camera_trans, mut lin_vel) = camera_query.single_mut().expect("No Camera");
    for target_t in target_query.iter() {
        // TODO: I'd like to do it this way , but it introduces a bug.
        let vec_to_target = target_t.translation.truncate() - camera_trans.translation.truncate();
        *lin_vel = LinearVelocity(vec_to_target.as_dvec2() * vec_to_target.length() as f64);

        // camera_trans.translation.x = target_t.translation.x;
        // camera_trans.translation.y = target_t.translation.y;
    }
}
