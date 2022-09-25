use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::prelude as lyon;
use std::f32::consts::PI;
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
            // .add_system(Self::projectile_movement)
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
            .insert(ActiveHooks::FILTER_CONTACT_PAIRS)
            .insert(Restitution::coefficient(0.01));
    }

    // fn projectile_movement(
    //     mut commands: Commands,
    //     mut projectile_query: Query<(Entity, &mut Projectile, &mut Transform)>,
    //     time: Res<Time>
    // )
    // {
    //     for (ent, mut projectile, mut transform) in projectile_query.iter_mut() {
    //         transform.translation.x += projectile.velocity.x;
    //         transform.translation.y += projectile.velocity.y;
    
    //         projectile.timer.tick(time.delta());
    
    //         if projectile.timer.finished() {
    //             commands.entity(ent).despawn_recursive();
    //         }
    //     }
    // }

    fn handle_projectile_collision_event(
        mut contact_events: EventReader<CollisionEvent>,
        astroid_query: Query<(Entity, &Astroid), With<Astroid>>,
        projectile_query: Query<(Entity, &Projectile), With<Projectile>>,
        time: Res<Time>,
        mut commands: Commands,
    ) {
        for contact_event in contact_events.iter() {
            if let CollisionEvent::Started(h1, h2, _event_flag) = contact_event {

                //FIXME: there's got to be a better way...
                // h1 is project, despawn it.
                // if projectile_query.contains(*h1) && astroid_query.contains(*h2) {
                //     println!("PROJECTILE COLLISION WITH ASTROID");
                //     commands.entity(*h1).despawn_recursive();
                // }

                // h2 is projectile, despawn it.
                // why does this work, but the above implementation doesn't???
                if projectile_query.contains(*h2) && astroid_query.contains(*h1) {
                    println!("PROJECTILE COLLISION WITH ASTROID");
                    commands.entity(*h2).despawn_recursive();
                }
            }
        }

    }
    
    // fn projectile_collision_check(
    //     mut commands: Commands,
    //     projectile_query: Query<(Entity, &Projectile, &Transform), With<Projectile>>,
    //     collider_query: Query<(Entity, &Transform, Option<&Astroid>), With<Collider>>
    // ){
    //     let mut rng = rand::thread_rng();
    
    //     for (projectile_ent, projectile, projectile_transform) in projectile_query.iter() {
    //         for (ent, ent_transform, maybe_astroid) in &collider_query {
    
    //             match maybe_astroid {
    //                 Some(astroid) => {
    //                     if Vec2::distance(
    //                         projectile_transform.translation.truncate(),
    //                         ent_transform.translation.truncate())
    //                          < PROJECTILE_RADIUS + astroid.hitbox.radius
    //                     {
    //                         let split_angle = rng.gen_range(0.0..PI/4.0);
                            
    //                         let right_velocity = projectile.velocity.rotate(Vec2::from_angle(split_angle)) * 0.5;
    //                         let left_velocity = projectile.velocity.rotate(Vec2::from_angle(-split_angle)) * 0.5;
    
    //                         match &astroid.size {
    //                             AstroidSize::Small => {
    //                             },
    //                             AstroidSize::Medium => {
    //                                 AstroidPlugin::spawn_astroid(&mut commands, AstroidSize::Small, right_velocity, ent_transform.translation.truncate());
    //                                 AstroidPlugin::spawn_astroid(&mut commands, AstroidSize::Small, left_velocity, ent_transform.translation.truncate());
    
    //                             },
    //                             AstroidSize::Large => {
    //                                 AstroidPlugin::spawn_astroid(&mut commands, AstroidSize::Medium, right_velocity,ent_transform.translation.truncate());
    //                                 AstroidPlugin::spawn_astroid(&mut commands, AstroidSize::Medium, left_velocity, ent_transform.translation.truncate());
    //                             }
    //                         }
    
    //                         commands.entity(projectile_ent).despawn_recursive();
    //                         commands.entity(ent).despawn_recursive();
    //                         return;
    //                     }
    //                 },
    //                 None => {
    
    //                 }
    //             }
    //         }
    //     }
    
    // }
}