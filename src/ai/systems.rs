use std::{borrow::BorrowMut, f32::consts::PI};

use avian2d::prelude::*;
use bevy::{color::palettes::css::RED, prelude::*};

use rand::Rng;

use crate::{health::components::Health, rcs::components::RCSBooster};

use super::{components::Enemy, resources::EnemySpawnTimer};

pub fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_time: ResMut<EnemySpawnTimer>,
    keys: Res<ButtonInput<KeyCode>>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    spawn_time.timer.tick(time.delta());

    if spawn_time.timer.is_finished() || keys.just_pressed(KeyCode::F1) {
        spawn_enemy(commands.borrow_mut(), meshes, materials);
        spawn_time.timer.reset();
    }
}

pub fn spawn_enemy(
    cmd: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::rng();

    let rand = rng.random::<f32>() * 2.0 * PI;
    let random_dir = Vec2::new(f32::cos(rand), f32::sin(rand));

    cmd.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(RED))),
        Transform {
            translation: (random_dir * 1000.0).extend(0.0),
            scale: Vec2::new(10.0, 10.0).extend(1.0),
            ..default()
        },
        RCSBooster::new(),
        RigidBody::Dynamic,
        Collider::circle(1.0),
        Health::new(),
        LinearVelocity::ZERO,
        Name::new("Enemy"),
        Enemy,
    ));
}

pub fn despawn_dead_enemies(mut commands: Commands, enemies: Query<(Entity, &Enemy, &Health)>) {
    for (entity, _, health) in enemies.iter() {
        if health.current <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
