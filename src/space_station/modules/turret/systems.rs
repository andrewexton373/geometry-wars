use bevy::{ecs::{entity::Entity, query::With, system::Query}, hierarchy::Parent, math::{Quat, Vec2, Vec3}, transform::components::{GlobalTransform, Transform}};

use crate::ai::components::Enemy;

use super::components::Turret;

pub fn update_turret_weapons(
    enemies: Query<(Entity, &GlobalTransform), With<Enemy>>,
    mut turrets: Query<(&Parent, &mut Transform, &GlobalTransform), With<Turret>>,
) {

    for (_, mut turret_transform, gt) in turrets.iter_mut() {
        
        let mut nearest_enemy: Option<(Entity, f32, Vec2)> = None;

        for (enemy_entitiy, enemy_gt) in enemies.iter() {

            let distance = (enemy_gt.translation().truncate() - turret_transform.translation.truncate()).length();

            if nearest_enemy.is_none() ||  distance < nearest_enemy.unwrap().1 {
                nearest_enemy = Some((enemy_entitiy, distance, enemy_gt.translation().truncate()));
            }

        }

        if let Some(nearest_enemy) = nearest_enemy {
            let trajectory = (nearest_enemy.2.extend(0.0) - gt.translation()).normalize();
            let rotate_to_enemy = Quat::from_rotation_arc(Vec3::X, trajectory);
            turret_transform.rotation = rotate_to_enemy;
        }

    }
}