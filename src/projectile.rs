// use crate::{
//     astroid::Astroid,
//     particles::ProjectileImpactParticles,
// };
// use bevy::prelude::*;
// use bevy_rapier2d::prelude::*;
// use crate::astroid_plugin::AstroidPlugin;
//
// #[derive(Component)]
// pub struct Projectile {
//     pub timer: Timer,
// }
//
// pub struct ProjectilePlugin;
//
// impl Plugin for ProjectilePlugin {
//     fn build(&self, app: &mut App) {
//         app.add_system(Self::handle_projectile_collision_event);
//     }
// }
//
// impl ProjectilePlugin {
// pub fn spawn_projectile(commands: &mut Commands, position: Vec2, player_velocity: Vec2) {
//     const BULLET_SPEED: f32 = 6.0;

//     let projectile_shape = lyon::shapes::Circle {
//         radius: PROJECTILE_RADIUS * crate::PIXELS_PER_METER,
//         ..lyon::shapes::Circle::default()
//     };

//     let spawn_position = position + player_velocity * 10.0;
//     let bullet_velocity = player_velocity * BULLET_SPEED * crate::PIXELS_PER_METER;

//     commands
//         .spawn((
//             Projectile {
//                 timer: Timer::from_seconds(5.0, TimerMode::Once),
//             },
//             ShapeBundle {
//                 path: GeometryBuilder::build_as(&projectile_shape),
//                 transform: Transform::from_translation(spawn_position.extend(0.0)),
//                 ..default()
//             },
//             Fill::color(Color::RED),
//             RigidBody::Dynamic,
//             Velocity {
//                 linvel: bullet_velocity * crate::PIXELS_PER_METER,
//                 angvel: 0.0,
//             },
//             Sleeping::disabled(),
//             Ccd::enabled(),
//             Collider::ball(projectile_shape.radius),
//             ActiveEvents::COLLISION_EVENTS,
//             Restitution::coefficient(0.01),
//         ));
// }

//     fn handle_projectile_collision_event(
//         mut astroid_query: Query<(Entity, &Astroid, &Transform, &Velocity), With<Astroid>>,
//         projectile_query: Query<(Entity, &Projectile, &Velocity), With<Projectile>>,
//         mut commands: Commands,
//         rapier_context: Res<RapierContext>,
//     ) {
//         for (projectile_ent, _projectile, projectile_velocity) in projectile_query.iter() {
//             for (astroid_ent, astroid, astroid_transform, _astroid_velocity) in
//                 astroid_query.iter_mut()
//             {
//                 if let Some(contact_pair_view) =
//                     rapier_context.contact_pair(projectile_ent, astroid_ent)
//                 {
//                     for manifold in contact_pair_view.manifolds() {
//                         // Read the solver contacts.
//
//                         for solver_contact in manifold.solver_contacts() {
//                             // Keep in mind that all the solver contact data are expressed in world-space.
//                             println!("PROJECTILE COLLISION WITH ASTROID");
//
//                             let (mut effect, mut effect_translation) = effect.single_mut();
//
//                             effect_translation.translation =
//                                 (solver_contact.point() * crate::PIXELS_PER_METER).extend(200.0);
//                             effect.maybe_spawner().unwrap().reset();
//
//                             AstroidPlugin::split_astroid(
//                                 &mut commands,
//                                 astroid_ent,
//                                 astroid,
//                                 astroid_transform.translation.truncate(),
//                                 projectile_velocity,
//                             );
//                             commands.entity(projectile_ent).despawn_recursive();
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }
