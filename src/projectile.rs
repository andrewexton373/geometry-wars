use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::prelude::{self as lyon, DrawMode};
use rand::Rng;
use crate::{astroid::{AstroidPlugin, Astroid, AstroidSize}, PIXELS_PER_METER};

const PROJECTILE_RADIUS: f32 = 0.2;

#[derive(Component)]
pub struct Projectile {
    pub timer: Timer
}

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(Self::handle_projectile_collision_event);
    }
}

impl ProjectilePlugin {

    pub fn spawn_projectile(
        commands: &mut Commands,
        position: Vec2,
        player_velocity: Vec2
    ) {
        const BULLET_SPEED: f32 = 4.0;

        let projectile_shape = lyon::shapes::Circle {
            radius: 0.5 * crate::PIXELS_PER_METER,
            ..lyon::shapes::Circle::default()
        };

        let spawn_position = position + player_velocity * 10.0;
        let bullet_velocity = player_velocity * BULLET_SPEED * crate::PIXELS_PER_METER;
    
        commands.spawn()
            .insert(Projectile {
                timer: Timer::from_seconds(5.0, false),
            })
            .insert_bundle(lyon::GeometryBuilder::build_as(
                &projectile_shape,
                lyon::DrawMode::Fill(lyon::FillMode::color(Color::RED)),
                Default::default()
            ))
            .insert(RigidBody::Dynamic)
            .insert(Velocity { linvel: bullet_velocity * crate::PIXELS_PER_METER, angvel: 0.0 })
            .insert(Sleeping::disabled())
            .insert(Ccd::enabled())
            .insert(Collider::ball(projectile_shape.radius))
            .insert(Transform::from_translation(spawn_position.extend(0.0)))
            .insert(ActiveEvents::COLLISION_EVENTS)
            // .insert(ActiveHooks::FILTER_CONTACT_PAIRS)
            .insert(Restitution::coefficient(0.01));
    }

    fn handle_projectile_collision_event(
        mut contact_events: EventReader<CollisionEvent>,
        mut astroid_query: Query<(Entity, &Astroid, &Transform, &Velocity), With<Astroid>>,
        projectile_query: Query<(Entity, &Projectile, &Velocity), With<Projectile>>,
        time: Res<Time>,
        mut commands: Commands,
    ) {
        for contact_event in contact_events.iter() {
            for (projectile_ent, projectile, projectile_velocity) in projectile_query.iter() {
                for (astroid_ent, astroid, astroid_transform, astroid_velocity) in astroid_query.iter_mut() {
                
                    if let CollisionEvent::Started(h1, h2, _event_flag) = contact_event {

                        if (projectile_ent == *h2 && astroid_ent == *h1) ||
                            (projectile_ent == *h1 && astroid_ent == *h2) {
    
                            println!("PROJECTILE COLLISION WITH ASTROID");

                            let mut rng = rand::thread_rng();
                            let split_angle = rng.gen_range(0.0..PI/4.0);
                            
                            let right_velocity = projectile_velocity.linvel.rotate(Vec2::from_angle(split_angle)) * 0.5;
                            let left_velocity = projectile_velocity.linvel.rotate(Vec2::from_angle(-split_angle)) * 0.5;
    
                            match &astroid.size {
                                AstroidSize::Small => {
                                },
                                AstroidSize::Medium => {
                                    AstroidPlugin::spawn_astroid(&mut commands, AstroidSize::Small, right_velocity, astroid_transform.translation.truncate());
                                    AstroidPlugin::spawn_astroid(&mut commands, AstroidSize::Small, left_velocity, astroid_transform.translation.truncate());
                                    commands.entity(astroid_ent).despawn_recursive();
                                },
                                AstroidSize::Large => {
                                    AstroidPlugin::spawn_astroid(&mut commands, AstroidSize::Medium, right_velocity,astroid_transform.translation.truncate());
                                    AstroidPlugin::spawn_astroid(&mut commands, AstroidSize::Medium, left_velocity, astroid_transform.translation.truncate());
                                    commands.entity(astroid_ent).despawn_recursive();
                                }
                            }
    
                            commands.entity(projectile_ent).despawn_recursive();
    
                        }
    
                    }
                
                }
                
            }
            
        }

    }
    
}