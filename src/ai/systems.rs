use std::{borrow::BorrowMut, f32::consts::PI};

use avian2d::prelude::*;
use bevy::{color::palettes::css::RED, prelude::*};
use big_brain::{
    actions::{ActionState, Steps},
    pickers::FirstToScore,
    scorers::Score,
    thinker::{ActionSpan, Actor, ScorerSpan, Thinker},
};
use rand::Rng;

use crate::{
    health::components::Health,
    player::components::Player,
    projectile::events::FireProjectileEvent,
    rcs::{components::RCSBooster, events::RCSThrustVectorEvent},
};

use super::{
    components::{Attack, Enemy, Hostile, Hostility, MoveTowardsPlayer},
    resources::EnemySpawnTimer,
};

pub fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_time: ResMut<EnemySpawnTimer>,
    keys: Res<ButtonInput<KeyCode>>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    spawn_time.timer.tick(time.delta());

    if spawn_time.timer.finished() || keys.just_pressed(KeyCode::F1) {
        spawn_enemy(commands.borrow_mut(), meshes, materials);
        spawn_time.timer.reset();
    }
}

pub fn spawn_enemy(
    cmd: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let move_towards_player_and_attack = Steps::build()
        .label("MoveTowardsPlayerAndAttack")
        .step(MoveTowardsPlayer {
            speed: 10000000000.0,
        })
        .step(Attack {
            until: 5.0,
            per_second: 1.0,
        });

    let thinker = Thinker::build()
        .label("Ai Thinker")
        .picker(FirstToScore { threshold: 0.8 })
        .when(Hostile, move_towards_player_and_attack);

    let mut rng = rand::thread_rng();

    let rand = rng.gen::<f32>() * 2.0 * PI;
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
        Hostility::new(75.0, 2.0),
        thinker.clone(),
        Name::new("Enemy"),
        Enemy,
    ));
}

pub fn despawn_dead_enemies(mut commands: Commands, enemies: Query<(Entity, &Enemy, &Health)>) {
    for (entity, _, health) in enemies.iter() {
        if health.current <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

// ACTIONS
pub fn attack_action_system(
    time: Res<Time>,
    player_q: Query<&GlobalTransform, With<Player>>,
    positions: Query<&GlobalTransform, Without<Player>>,
    velocities: Query<&LinearVelocity>,
    mut hostilities: Query<&mut Hostility>,
    mut query: Query<(&Actor, &mut ActionState, &Attack, &ActionSpan)>,
    mut fire_projectile_events: EventWriter<FireProjectileEvent>,
) {
    for (Actor(actor), mut state, attack, span) in &mut query {
        // This sets up the tracing scope. Any `debug` calls here will be
        // spanned together in the output.
        let _guard = span.span().enter();

        if let Ok(mut hostility) = hostilities.get_mut(*actor) {
            match *state {
                ActionState::Requested => {
                    debug!("Time to attack player!");
                    *state = ActionState::Executing;
                }
                ActionState::Executing => {
                    trace!("Attacking player...");
                    hostility.hostility -=
                        attack.per_second * (time.delta().as_micros() as f32 / 1_000_000.0);

                    let player_gt = player_q.single();
                    let actor_gt = positions
                        .get(*actor)
                        .expect("actor does not have a global transform");
                    let actor_lin_vel = velocities
                        .get(*actor)
                        .expect("actor does not have linear velocity!");

                    let dir_to_player = (player_gt.translation() - actor_gt.translation())
                        .truncate()
                        .normalize();
                    fire_projectile_events.send(FireProjectileEvent {
                        entity: *actor,
                        projectile_trajectory: LinearVelocity(
                            actor_lin_vel.0
                                + (dir_to_player * 50.0 * crate::PIXELS_PER_METER as f32)
                                    .as_dvec2(),
                        ),
                    });

                    *state = ActionState::Success;

                    // if hostility.hostility <= attack.until {
                    //     debug!("Done Attacking, Getting some Rest");
                    //     *state = ActionState::Success;
                    // }
                }
                ActionState::Cancelled => {
                    debug!("Attack Action was cancelled!");
                    *state = ActionState::Failure;
                }
                _ => {}
            }
        }
    }
}

pub const MAX_DISTANCE: f32 = 500.0;

pub fn move_towards_player_action_system(
    time: Res<Time>,
    player_q: Query<(Entity, &GlobalTransform, &LinearVelocity), With<Player>>,
    mut enemies: Query<(&mut GlobalTransform, &LinearVelocity), Without<Player>>,
    mut action_query: Query<(&Actor, &mut ActionState, &MoveTowardsPlayer, &ActionSpan)>,
    mut thrust_events: EventWriter<RCSThrustVectorEvent>,
) {
    for (actor, mut action_state, move_to, span) in &mut action_query {
        let _guard = span.span().enter();

        match *action_state {
            ActionState::Requested => {
                debug!("Let's move towards the player!");
                *action_state = ActionState::Executing;
            }
            ActionState::Executing => {
                let (actor_position, actor_linear_velocity) =
                    enemies.get_mut(actor.0).expect("actor has no position");
                trace!("Actor position: {:?}", actor_position);

                let (player_ent, player_position, player_linear_velocity) = player_q.single();
                let delta =
                    (player_position.translation() - actor_position.translation()).truncate();
                let distance = delta.length();

                if distance > MAX_DISTANCE {
                    trace!("Thrusting Closer.");

                    let step_size = time.delta_secs() * move_to.speed;
                    let step = delta.normalize() * step_size.min(distance);

                    // Try and match player velocity?
                    let delta_velocity = actor_linear_velocity.xy() - player_linear_velocity.xy();

                    let mix = step.lerp(delta_velocity.as_vec2(), 1.0 / (distance - MAX_DISTANCE));

                    thrust_events.send(RCSThrustVectorEvent {
                        entity: actor.0,
                        thrust_vector: mix,
                    });
                } else {
                    *action_state = ActionState::Success;
                }
            }
            ActionState::Cancelled => {
                *action_state = ActionState::Failure;
            }
            ActionState::Success => {
                debug!("We got there!");
                *action_state = ActionState::Success;
            }
            ActionState::Failure => {}
            _ => {}
        }
    }
}

pub fn hostility_system(time: Res<Time>, mut hostilities: Query<&mut Hostility>) {
    for mut hostility in &mut hostilities {
        hostility.hostility +=
            hostility.per_second * (time.delta().as_micros() as f32 / 1_000_000.0);
        hostility.hostility = hostility.hostility.clamp(0.0, 100.0);
        trace!("Hostility: {}", hostility.hostility);
    }
}

pub fn hostility_scorer_system(
    hostilities: Query<&Hostility>,
    mut query: Query<(&Actor, &mut Score, &ScorerSpan), With<Hostile>>,
) {
    for (Actor(actor), mut score, span) in query.iter_mut() {
        if let Ok(hostility) = hostilities.get(*actor) {
            score.set(hostility.hostility / 100.0);

            if hostility.hostility >= 80.0 {
                span.span().in_scope(|| {
                    debug!(
                        "Hostility above threshold! Score: {}",
                        hostility.hostility / 100.0
                    )
                });
            }
        }
    }
}
