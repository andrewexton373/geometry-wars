use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use rand::Rng;
use std::f32::consts::{PI};
use crate::{ HitboxCircle, Collider, Player };
use crate::healthbar::Health;

pub struct AstroidPlugin;

#[derive(Component, Inspectable)]
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
            Self::Small => 8.0,
            Self::Medium => 14.0,
            Self::Large => 20.0
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
            .add_system(Self::astroid_movement)
            .add_system(Self::astroid_collision_check)
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

            Self::spawn_astroid(&mut commands, AstroidSize::Large, direction_to_player, random_spawn_position);
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

    fn spawn_static_astroids(
        mut commands: Commands
    ){
        let mut rng = rand::thread_rng();
    
        for i in 0..15 {
            let random_postion = Vec2 {x: rng.gen_range(-300.0..300.0), y: rng.gen_range(-300.0..300.0)};
            Self::spawn_astroid(&mut commands, AstroidSize::Large, Vec2 { x: 0.0, y: 0.0 }, random_postion);
        }
    
    }
    
    pub fn spawn_astroid(
        commands: &mut Commands,
        size: AstroidSize,
        velocity: Vec2,
        position: Vec2
    ) {
    
        let astroid_shape: shapes::RegularPolygon;
        match size {
            AstroidSize::Small => {
                astroid_shape = shapes::RegularPolygon {
                    sides: 3,
                    ..shapes::RegularPolygon::default()
                };
            },
            AstroidSize::Medium => {
                astroid_shape = shapes::RegularPolygon {
                    sides: 4,
                    ..shapes::RegularPolygon::default()
                };
            },
            AstroidSize::Large => {
                astroid_shape = shapes::RegularPolygon {
                    sides: 7,
                    ..shapes::RegularPolygon::default()
                };
            }
        }
    
        commands.spawn()
            .insert(Astroid {
                velocity: velocity,
                size: size,
                health: Health {current: 50.0, maximum: 100.0},
                hitbox: HitboxCircle { radius: size.radius() }
            })
            .insert_bundle(GeometryBuilder::build_as(
                &astroid_shape,
                DrawMode::Fill(FillMode::color(Color::RED)),
                // DrawMode::Outlined {
                //     fill_mode: FillMode::color(Color::DARK_GRAY),
                //     outline_mode: StrokeMode::new(Color::WHITE, 1.0),
                // },
                Transform {
                    translation: position.extend(0.0),
                    scale: Vec3::new(size.radius(), size.radius(), 0.0),
                    ..default()
                }
            ))
            .insert(Collider);
            
    }
    
    fn astroid_movement(
        mut astroid_query: Query<(&mut Astroid,&mut Transform)>,
        time: Res<Time>
    ){
    
        for (mut astroid, mut transform) in astroid_query.iter_mut() {
            transform.translation.x += astroid.velocity.x;
            transform.translation.y += astroid.velocity.y;
    
            // projectile.timer.tick(time.delta());
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