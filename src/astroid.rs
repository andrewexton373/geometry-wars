use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::prelude as lyon;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use rand::Rng;
use std::f32::consts::{PI};
use crate::{ HitboxCircle, Player };
use crate::healthbar::Health;

pub struct AstroidPlugin;

#[derive(Component, Inspectable, Clone, Copy)]
pub struct Astroid {
    pub velocity: Vec2,
    pub size: AstroidSize,
    pub health: Health,
    pub hitbox: HitboxCircle
    // TODO: upon destruction, astroid should split into smaller asteroids
}

#[derive(Clone, Copy, Inspectable)]
pub enum AstroidSize {
    Small,
    Medium,
    Large
}

impl AstroidSize {
    fn radius(self) -> f32 {
        match self {
            Self::Small => 2.0,
            Self::Medium => 5.0,
            Self::Large => 10.0
        }
    }
}

#[derive(Component)]
pub struct AstroidSpawner {
    timer: Timer
}

impl Plugin for AstroidPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_startup_system(Self::spawn_astroids)
            .insert_resource(AstroidSpawner{ timer: Timer::from_seconds(0.10, false)})
            .add_system(Self::spawn_astroids_aimed_at_ship)
            .add_system(Self::despawn_far_astroids)
            // .add_system(Self::astroid_movement) // shouldn't have to do, handled by rapier2d
            // .add_system(Self::astroid_collision_check)
            .add_system(Self::handle_astroid_collision_event)
            .register_inspectable::<Astroid>();
    }
}

impl AstroidPlugin {
    fn spawn_astroids_aimed_at_ship(
        mut commands: Commands,
        player_query: Query<(&Player, &Transform)>,
        mut astroid_spawner: ResMut<AstroidSpawner>,
        time: Res<Time>
    ) {
        astroid_spawner.timer.tick(time.delta());

        if astroid_spawner.timer.finished() {
            astroid_spawner.timer.reset();

            let mut rng = rand::thread_rng();
            let (player, transform) = player_query.single();

            let player_position = transform.translation.truncate();

            let rand_x: f32 = rng.gen_range(-PI..PI);
            let rand_y: f32 = rng.gen_range(-PI..PI);
            let rand_direction = Vec2::new(rand_x, rand_y);

            const SPAWN_DISTANCE: f32 = 1000.0;
            let random_spawn_position = player_position + (rand_direction * SPAWN_DISTANCE);
            let direction_to_player = (player_position - random_spawn_position).normalize(); // maybe?

            Self::spawn_astroid(&mut commands, AstroidSize::Large, direction_to_player * crate::PIXELS_PER_METER, random_spawn_position);
        }
    }

    fn despawn_far_astroids(
        mut commands: Commands,
        mut astroid_query: Query<(Entity, &mut Astroid, &mut Transform), With<Astroid>>,
        player_query: Query<(&Player, &Transform), (With<Player>, Without<Astroid>)>,
    ) {
        const DESPAWN_DISTANCE: f32 = 5000.0;
        let (player, transform) = player_query.single();
        let player_position = transform.translation.truncate();

        for (entity, astroid, transform) in astroid_query.iter() {
            let astroid_position = transform.translation.truncate();
            // println!("{}", player_position.distance(astroid_position));
            if player_position.distance(astroid_position) > DESPAWN_DISTANCE {
                // println!("DESPAWN: {}", player_position.distance(astroid_position));
                commands.entity(entity).despawn_recursive();
            }
        }
    }
    
    pub fn spawn_astroid(
        commands: &mut Commands,
        size: AstroidSize,
        velocity: Vec2,
        position: Vec2
    ) {
    
        let astroid_shape: lyon::shapes::RegularPolygon;
        match size {
            AstroidSize::Small => {
                astroid_shape = lyon::shapes::RegularPolygon {
                    sides: 3,
                    feature: lyon::RegularPolygonFeature::Radius(crate::PIXELS_PER_METER * size.radius()),
                    ..lyon::shapes::RegularPolygon::default()
                };
            },
            AstroidSize::Medium => {
                astroid_shape = lyon::shapes::RegularPolygon {
                    sides: 4,
                    feature: lyon::RegularPolygonFeature::Radius(crate::PIXELS_PER_METER * size.radius()),
                    ..lyon::shapes::RegularPolygon::default()
                };
            },
            AstroidSize::Large => {
                astroid_shape = lyon::shapes::RegularPolygon {
                    sides: 7,
                    feature: lyon::RegularPolygonFeature::Radius(crate::PIXELS_PER_METER * size.radius()),
                    ..lyon::shapes::RegularPolygon::default()
                };
            }
        }

        let astroid = Astroid {
            velocity: velocity,
            size: size,
            health: Health {current: 50.0, maximum: 100.0},
            hitbox: HitboxCircle { radius: size.radius() }
        };
    
        commands.spawn()
            .insert(astroid)
            .insert_bundle(lyon::GeometryBuilder::build_as(
                &astroid_shape,
                lyon::DrawMode::Fill(lyon::FillMode::color(Color::RED)),
                Default::default()
            ))
            // .insert(Collider) // do we still need? don't think so
            .insert(RigidBody::Dynamic)
            .insert(Velocity { linvel: astroid.velocity, angvel: 0.0 })
            .insert(Sleeping::disabled())
            .insert(Ccd::enabled())
            .insert(Collider::ball(crate::PIXELS_PER_METER * size.radius()))
            .insert(Transform::from_xyz(position.x, position.y, 0.0))
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(Restitution::coefficient(0.01));
            
    }

    fn handle_astroid_collision_event(
        astroid_query: Query<(Entity, &Astroid), With<Astroid>>,
        mut player_query: Query<(Entity, &mut Player), With<Player>>,
        time: Res<Time>,
        mut contact_events: EventReader<CollisionEvent>,
        mut commands: Commands
    ) {
        let (player_ent, mut player) = player_query.single_mut();

        for contact_event in contact_events.iter() {
            for (astroid_entity, astroid) in astroid_query.iter() {
                if let CollisionEvent::Started(h1, h2, _event_flag) = contact_event {
                    
                    // If player hit astroid
                    if player_ent == *h1 && astroid_entity == *h2 {
                        let timestamp_last_hit = time.seconds_since_startup();

                        match &astroid.size {
                            AstroidSize::Small => {
                                println!("Hit small Astroid");
                                // TODO: collect minerals?

                            },
                            AstroidSize::Medium => {
                                println!("Hit medium Astroid");
                                // commands.entity(ent).despawn_recursive();

                            },
                            AstroidSize::Large => {
                                println!("Hit large Astroid");
                                
                                commands.entity(player_ent)
                                    .insert(ExternalForce {
                                        force: Vec2::new(1000.0, 2000.0),
                                        torque: 100.0,
                                    });

                                player.take_damage(5.0);
                                // commands.entity(ent).despawn_recursive();

                            }
                        }
                        return;
                    }
                    
                }
            }
        }
    }

    fn astroid_collision_check(
        mut commands: Commands,
        mut player_query: Query<(Entity, &mut Player, &Transform), With<Player>>,
        collider_query: Query<(Entity, &Transform, Option<&Astroid>), With<Collider>>
    ){
        // let mut rng = rand::thread_rng();
    
        for (player_ent, mut player, player_transform) in player_query.iter_mut() {
            for (ent, ent_transform, maybe_astroid) in &collider_query {
    
                match maybe_astroid {
                    Some(astroid) => {
                        if Vec2::distance(
                            player_transform.translation.truncate(),
                            ent_transform.translation.truncate())
                             < player.hitbox.radius + astroid.hitbox.radius
                        {
                            // let split_angle = rng.gen_range(0.0..PI/4.0);

                            // let right_velocity = player.velocity.rotate(Vec2::from_angle(split_angle)) * 0.5;
                            // let left_velocity = player.velocity.rotate(Vec2::from_angle(-split_angle)) * 0.5;
    
                            match &astroid.size {
                                AstroidSize::Small => {
                                    println!("Hit small Astroid");
                                    // TODO: collect minerals?

                                },
                                AstroidSize::Medium => {
                                    println!("Hit medium Astroid");
                                    commands.entity(ent).despawn_recursive();

                                },
                                AstroidSize::Large => {
                                    println!("Hit large Astroid");
                                    player.take_damage(5.0);
                                    commands.entity(ent).despawn_recursive();

                                }
                            }
    
                            // commands.entity(player_ent).despawn_recursive();
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