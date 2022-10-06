use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::prelude as lyon;
use crate::{astroid::{AstroidPlugin, Astroid}};

const PROJECTILE_RADIUS: f32 = 0.5;

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
        const BULLET_SPEED: f32 = 6.0;

        let projectile_shape = lyon::shapes::Circle {
            radius: PROJECTILE_RADIUS * crate::PIXELS_PER_METER,
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
        mut commands: Commands,
    ) {
        for contact_event in contact_events.iter() {
            for (projectile_ent, _projectile, projectile_velocity) in projectile_query.iter() {
                for (astroid_ent, astroid, astroid_transform, _astroid_velocity) in astroid_query.iter_mut() {
                
                    if let CollisionEvent::Started(h1, h2, _event_flag) = contact_event {

                        if (projectile_ent == *h2 && astroid_ent == *h1) ||
                            (projectile_ent == *h1 && astroid_ent == *h2) {

                            println!("PROJECTILE COLLISION WITH ASTROID");
                            AstroidPlugin::split_astroid(&mut commands, astroid_ent, astroid, astroid_transform.translation.truncate(), projectile_velocity);
                            commands.entity(projectile_ent).despawn_recursive();

    
                        }
    
                    }
                
                }
                
            }
            
        }

    }
    
}