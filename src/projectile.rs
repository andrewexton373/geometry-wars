use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use rand::Rng;
use crate::{ HitboxCircle, Health, Collider, PI };
use crate::astroid::{AstroidPlugin, Astroid, AstroidSize};

#[derive(Component)]
pub struct Projectile {
    pub velocity: Vec2,
    pub timer: Timer,
    pub hitbox: HitboxCircle
}

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(Self::projectile_movement)
            .add_system(Self::projectile_collision_check);

    }
}

impl ProjectilePlugin {
    fn projectile_movement(
        mut commands: Commands,
        mut projectile_query: Query<(Entity, &mut Projectile, &mut Transform)>,
        time: Res<Time>
    )
    {
        for (ent, mut projectile, mut transform) in projectile_query.iter_mut() {
            transform.translation.x += projectile.velocity.x;
            transform.translation.y += projectile.velocity.y;
    
            projectile.timer.tick(time.delta());
    
            if projectile.timer.finished() {
                commands.entity(ent).despawn_recursive();
            }
        }
    }
    
    fn projectile_collision_check(
        mut commands: Commands,
        projectile_query: Query<(Entity, &Projectile, &Transform), With<Projectile>>,
        collider_query: Query<(Entity, &Transform, Option<&Astroid>), With<Collider>>
    ){
        let mut rng = rand::thread_rng();
    
        for (projectile_ent, projectile, projectile_transform) in projectile_query.iter() {
            for (ent, ent_transform, maybe_astroid) in &collider_query {
    
                match maybe_astroid {
                    Some(astroid) => {
                        if Vec2::distance(
                            projectile_transform.translation.truncate(),
                            ent_transform.translation.truncate())
                             < projectile.hitbox.radius + astroid.hitbox.radius
                        {
                            let split_angle = rng.gen_range(0.0..PI/4.0);
                            
                            let right_velocity = projectile.velocity.rotate(Vec2::from_angle(split_angle)) * 0.5;
                            let left_velocity = projectile.velocity.rotate(Vec2::from_angle(-split_angle)) * 0.5;
    
                            match &astroid.size {
                                AstroidSize::Small => {
                                },
                                AstroidSize::Medium => {
                                    AstroidPlugin::spawn_astroid(&mut commands, AstroidSize::Small, right_velocity, ent_transform.translation.truncate());
                                    AstroidPlugin::spawn_astroid(&mut commands, AstroidSize::Small, left_velocity, ent_transform.translation.truncate());
    
                                },
                                AstroidSize::Large => {
                                    AstroidPlugin::spawn_astroid(&mut commands, AstroidSize::Medium, right_velocity,ent_transform.translation.truncate());
                                    AstroidPlugin::spawn_astroid(&mut commands, AstroidSize::Medium, left_velocity, ent_transform.translation.truncate());
                                }
                            }
    
                            commands.entity(projectile_ent).despawn_recursive();
                            commands.entity(ent).despawn_recursive();
                            return;
                        }
                    },
                    None => {
    
                    }
                }
            }
        }
    
    }
}