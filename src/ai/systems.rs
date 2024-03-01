use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_prototype_lyon::{draw::Fill, entity::ShapeBundle, geometry::GeometryBuilder, prelude::tess::geom::Translation, shapes};
use bevy_xpbd_2d::components::{Collider, RigidBody};
use big_brain::{actions::{ActionState, Steps}, pickers::FirstToScore, scorers::Score, thinker::{ActionSpan, Actor, ScorerSpan, Thinker}};
use rand::Rng;

use crate::{player::{self, components::Player}, rcs::{components::RCSBooster, events::RCSThrustVectorEvent}};

use super::components::{Attack, Hostile, Hostility, MoveTowardsPlayer, Position};

pub fn init_entities(mut cmd: Commands) {

    let move_towards_player = Steps::build()
        .label("MoveTowardsPlayer")
        .step(MoveTowardsPlayer {speed: 10000000000.0});

    let thinker = Thinker::build()
        .label("Ai Thinker")
        .picker(FirstToScore { threshold: 0.8 })
        // .when(Hostile, Attack {
        //     until: 70.0,
        //     per_second: 5.0
        // })
        .when(Hostile, move_towards_player);

    let mut rng = rand::thread_rng();


    for _ in 0..20 {

        let rand = rng.gen::<f32>() * 2.0 * PI;
        let random_dir = Vec2::new(f32::cos(rand), f32::sin(rand));

        cmd.spawn((
            ShapeBundle {
                path: GeometryBuilder::new().add(&shapes::Circle::default()).build(),
                spatial: SpatialBundle {
                    transform: Transform {
                        translation: (random_dir * 1000.0).extend(0.0),
                        scale: Vec2::new(10.0, 10.0).extend(1.0),
                        ..default()
                     },
                    ..default()
                },
                ..default()
            },
            RCSBooster::new(),
            RigidBody::Dynamic,
            Collider::ball(1.0),
            Fill::color(Color::RED),
            Hostility::new(75.0, 2.0),
            thinker.clone()
        ));
    }

    
}

// ACTIONS
pub fn attack_action_system(
    time: Res<Time>,
    mut hostilities: Query<&mut Hostility>,
    mut query: Query<(&Actor, &mut ActionState, &Attack, &ActionSpan)>
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
                },
                ActionState::Executing => {
                    trace!("Attacking player...");
                    hostility.hostility -= attack.per_second * (time.delta().as_micros() as f32 / 1_000_000.0);
                    if hostility.hostility <= attack.until {
                        debug!("Done Attacking, Getting some Rest");
                        *state = ActionState::Success;
                    }
                },
                ActionState::Cancelled => {
                    debug!("Attack Action was cancelled!");
                    *state = ActionState::Failure;
                },
                _ => {}
            }


        }


    }
}

pub const MAX_DISTANCE: f32 = 100.0;

pub fn move_towards_player_action_system(
    time: Res<Time>,
    player_q: Query<(Entity, &GlobalTransform), With<Player>>,
    mut positions: Query<&mut GlobalTransform, Without<Player>>,
    mut action_query: Query<(&Actor, &mut ActionState, &MoveTowardsPlayer, &ActionSpan)>,
    mut thrust_events: EventWriter<RCSThrustVectorEvent>
) {
    for (actor, mut action_state, move_to, span) in &mut action_query {
        let _guard = span.span().enter();

        match *action_state {
            ActionState::Requested => {
                debug!("Let's move towards the player!");
                *action_state = ActionState::Executing;
            },
            ActionState::Executing => {
                let mut actor_position = positions.get_mut(actor.0).expect("actor has no position");
                trace!("Actor position: {:?}", actor_position);

                let (player_ent, player_position) = player_q.single();
                let delta = player_position.translation() - actor_position.translation();
                let distance = delta.length();

                if distance > MAX_DISTANCE {
                    trace!("Thrusting Closer.");

                    let step_size = time.delta_seconds() * move_to.speed;
                    let step = delta.normalize() * step_size.min(distance);

                    thrust_events.send(RCSThrustVectorEvent {
                        entity: actor.0,
                        thrust_vector: step.truncate()
                    });
                    
                }

            },
            ActionState::Cancelled => {
                *action_state = ActionState::Failure;
            },
            ActionState::Success => {
                debug!("We got there!");
                *action_state = ActionState::Success;
            },
            ActionState::Failure => {
                
            },
            _ => {}
        }
    }
}

pub fn hostility_system(time:Res<Time>, mut hostilities: Query<&mut Hostility>) {
    for mut hostility in &mut hostilities {
        hostility.hostility += hostility.per_second * (time.delta().as_micros() as f32 / 1_000_000.0);
        hostility.hostility = hostility.hostility.clamp(0.0, 100.0);
        trace!("Hostility: {}", hostility.hostility);
    }
}


pub fn hostility_scorer_system(
    hostilities: Query<&Hostility>,
    mut query: Query<(&Actor, &mut Score, &ScorerSpan), With<Hostile>>
) {
    for (Actor(actor), mut score, span) in query.iter_mut(){
        if let Ok(hostility) = hostilities.get(*actor) {
            score.set(hostility.hostility / 100.0);

            if hostility.hostility >= 80.0 {
                span.span().in_scope(|| {
                    debug!("Hostility above threshold! Score: {}", hostility.hostility / 100.0)
                });
            }
        }
    }
}