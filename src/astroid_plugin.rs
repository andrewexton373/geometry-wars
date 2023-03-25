use bevy::prelude::{Color, Commands, Component, default, Entity, EventReader, GlobalTransform, Query, Res, ResMut, Resource, Text, Text2dBundle, TextStyle, Time, Timer, TimerMode, Transform, With, Without};
use bevy::math::Vec2;
use bevy::app::{App, Plugin};
use bevy_tweening::{Animator, EaseFunction, RepeatCount, Tween, TweenCompleted, TweeningPlugin};
use std::f32::consts::PI;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::geometry::GeometryBuilder;
use bevy_prototype_lyon::draw::Fill;
use bevy_rapier2d::dynamics::{Ccd, MassProperties, ReadMassProperties, RigidBody, Sleeping, Velocity};
use bevy_rapier2d::geometry::{ActiveEvents, Collider, Restitution};
use bevy::core::Name;
use bevy_prototype_lyon::prelude::FillOptions;
use bevy_rapier2d::plugin::RapierContext;
// use bevy_hanabi::ParticleEffect;
use ordered_float::OrderedFloat;
use bevy::asset::AssetServer;
use bevy_tweening::lens::TextColorLens;
use rand::Rng;
use bevy::hierarchy::DespawnRecursiveExt;
use bevy_rapier2d::na::Translation;
use crate::astroid::Astroid;
use crate::astroid_composition::AstroidComposition;
use crate::astroid_material::AstroidMaterial;
use crate::astroid_size::{AstroidSize, Collectible};
use crate::base_station::BaseStation;
use crate::events::AblateEvent;
use crate::game_ui::{ContextClue, ContextClues};
use crate::inventory::{Amount, Inventory, InventoryItem};
// use crate::particles::ShipAstroidImpactParticles;
use crate::PIXELS_PER_METER;
use crate::player::Player;

pub struct AstroidPlugin;

#[derive(Resource)]
pub struct InventoryFullNotificationTimer(pub Option<Timer>);

#[derive(Component)]
pub struct Splittable(f32);

#[derive(Component, Resource)]
pub struct AstroidSpawner {
    timer: Timer,
}

const LASER_DAMAGE: f32 = 250.0;


impl Plugin for AstroidPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(TweeningPlugin)
            .insert_resource(AstroidSpawner {
                timer: Timer::from_seconds(0.25, TimerMode::Once),
            })
            .insert_resource(InventoryFullNotificationTimer(None))
            .add_event::<AblateEvent>()
            .add_system(Self::spawn_astroids_aimed_at_ship)
            .add_system(Self::despawn_far_astroids)
            .add_system(Self::handle_astroid_collision_event)
            .add_system(Self::ablate_astroids)
            .add_system(Self::split_astroids_over_split_ratio)
            .add_system(Self::remove_post_animation_text)
            .add_system(Self::display_inventory_full_context_clue)
            .add_system(Self::update_collectible_material_color);
            // .register_inspectable::<Astroid>();
    }
}

impl AstroidPlugin {

    // System to spawn astroids at some distance away from the ship in random directions,
    // each astroid with an initial velocity aimed towards the players ship
    fn spawn_astroids_aimed_at_ship(
        mut commands: Commands,
        player_query: Query<(&Player, &GlobalTransform)>,
        base_station_query: Query<(&BaseStation, &GlobalTransform)>,
        mut astroid_spawner: ResMut<AstroidSpawner>,
        time: Res<Time>,
    ) {
        const SPAWN_DISTANCE: f32 = 350.0;

        astroid_spawner.timer.tick(time.delta());

        if astroid_spawner.timer.finished() {
            astroid_spawner.timer.reset();

            let mut rng = rand::thread_rng();
            let (_player, player_g_transform) = player_query.single();
            let (_base_station, base_station_g_transform) = base_station_query.single();

            let distance_to_base_station = (player_g_transform.translation()
                - base_station_g_transform.translation())
            .length();
            let player_position = player_g_transform.translation().truncate();

            let rand_x: f32 = rng.gen_range(-PI..PI);
            let rand_y: f32 = rng.gen_range(-PI..PI);
            let rand_direction = Vec2::new(rand_x.cos(), rand_y.sin()).normalize();

            let random_spawn_position =
                player_position + (rand_direction * SPAWN_DISTANCE * crate::PIXELS_PER_METER);
            let direction_to_player = (player_position - random_spawn_position).normalize() * 20.0; // maybe?

            Self::spawn_astroid(
                &mut commands,
                AstroidSize::Large,
                AstroidComposition::new_with_distance(distance_to_base_station),
                direction_to_player * crate::PIXELS_PER_METER,
                random_spawn_position,
            );
        }
    }

    fn despawn_far_astroids(
        mut commands: Commands,
        astroid_query: Query<(Entity, &mut Astroid, &mut Transform), With<Astroid>>,
        player_query: Query<(&Player, &Transform), (With<Player>, Without<Astroid>)>,
    ) {
        const DESPAWN_DISTANCE: f32 = 1000.0 * PIXELS_PER_METER;
        let (_player, transform) = player_query.single();
        let player_position = transform.translation.truncate();

        for (entity, _astroid, transform) in astroid_query.iter() {
            let astroid_position = transform.translation.truncate();
            if player_position.distance(astroid_position) > DESPAWN_DISTANCE {
                commands.entity(entity).despawn_recursive();
            }
        }
    }

    pub fn spawn_astroid(
        commands: &mut Commands,
        size: AstroidSize,
        composition: AstroidComposition,
        velocity: Vec2,
        position: Vec2,
    ) {
        let astroid = Astroid::new_with(size, composition);

        let mut rng = rand::thread_rng();

        let splittable = Splittable(rng.gen_range(0.4..0.8));

        let astroid_ent = commands
            .spawn((
                astroid.clone(),
                ShapeBundle {
                    path: GeometryBuilder::build_as(&astroid.polygon()),
                    transform: Transform::from_xyz(position.x, position.y, 0.0),
                    ..default()
                },
                Fill::color(Color::DARK_GRAY),
                RigidBody::Dynamic,
                Velocity {
                    linvel: velocity,
                    angvel: 0.0,
                },
                Sleeping::disabled(),
                Collider::convex_hull(&astroid.polygon().points).unwrap(),
                ActiveEvents::COLLISION_EVENTS,
                ReadMassProperties(MassProperties::default()),
                Restitution::coefficient(0.01),
                splittable,
                Name::new("Astroid"),
            )).id();

        // If the astroid is an ore chunk, add Collectible Tag
        if astroid.clone().size == AstroidSize::OreChunk {
            commands.entity(astroid_ent).insert(Collectible);
        }
    }

    fn update_collectible_material_color(
        mut astroid_query: Query<(&Astroid, &mut Fill), With<Astroid>>,
    ) {
        for (astroid, mut fill) in astroid_query.iter_mut() {
            if astroid.size == AstroidSize::OreChunk {
                // if let DrawMode::Fill(ref mut fill_mode) = *draw_mode {
                    match astroid.primary_composition() {
                        AstroidMaterial::Iron => {
                            *fill = Fill{
                                color: Color::GRAY,
                                options: FillOptions::default(),
                            };
                            // fill_mode.color = Color::GRAY;
                        }
                        AstroidMaterial::Silver => {
                            *fill = Fill{
                                color: Color::SILVER,
                                options: FillOptions::default(),
                            };

                            // fill_mode.color = Color::SILVER;
                        }
                        AstroidMaterial::Gold => {
                            *fill = Fill{
                                color: Color::GOLD,
                                options: FillOptions::default()
                            };

                            // fill_mode.color = Color::GOLD;
                        }
                        _ => {}
                    }
                // }
            }
        }
    }

    fn handle_astroid_collision_event(
        mut commands: Commands,
        rapier_context: Res<RapierContext>,
        mut astroid_query: Query<(Entity, &Astroid, &ReadMassProperties), With<Astroid>>,
        mut player_query: Query<(Entity, &mut Player, &mut Inventory), With<Player>>,
        // mut effect: Query<
        //     (&mut ParticleEffect, &mut Transform),
        //     (
        //         With<ShipAstroidImpactParticles>,
        //         Without<Astroid>,
        //         Without<Player>,
        //     ),
        // >,
        mut inventory_full_notification: ResMut<InventoryFullNotificationTimer>,
    ) {
        let (player_ent, mut player, mut inventory) = player_query.single_mut();

        for (astroid_entity, astroid, mass_properties) in astroid_query.iter_mut() {
            if let Some(contact_pair_view) = rapier_context.contact_pair(player_ent, astroid_entity)
            {
                for manifold in contact_pair_view.manifolds() {
                    // Read the solver contacts.

                    for solver_contact in manifold.solver_contacts() {
                        // Keep in mind that all the solver contact data are expressed in world-space.

                        let mut astroid_collision = false;

                        match astroid.size {
                            AstroidSize::OreChunk => {
                                let ore_chunk_mass = mass_properties.0.mass;
                                for comp in astroid.composition.percent_composition().iter() {
                                    if !inventory.add_to_inventory(&InventoryItem::Material(
                                        *comp.0,
                                        Amount::Weight(OrderedFloat(comp.1 * ore_chunk_mass)),
                                    )) {
                                        inventory_full_notification.0 =
                                            Some(Timer::from_seconds(3.0, TimerMode::Once));
                                    }
                                }

                                // FIXME: will despawn even if there's no room in inventory to collect.
                                commands.entity(astroid_entity).despawn_recursive();
                            }
                            AstroidSize::Small => {
                                player.take_damage(1.0);
                                astroid_collision = true;
                            }
                            AstroidSize::Medium => {
                                player.take_damage(2.5);
                                astroid_collision = true;
                            }
                            AstroidSize::Large => {
                                player.take_damage(5.0);
                                astroid_collision = true;
                            }
                        }

                        if astroid_collision {
                            // let (mut effect, mut effect_translation) = effect.single_mut();
                            // effect_translation.translation =
                            //     (solver_contact.point() * crate::PIXELS_PER_METER).extend(200.0);
                            // effect.maybe_spawner().unwrap().reset();
                        }
                    }
                }
            }
        }
    }

    pub fn display_inventory_full_context_clue(
        mut context_clues_res: ResMut<ContextClues>,
        mut inventory_full_notification: ResMut<InventoryFullNotificationTimer>,
        time: Res<Time>,
    ) {
        if let Some(timer) = inventory_full_notification.0.as_mut() {
            timer.tick(time.delta());

            context_clues_res.0.insert(ContextClue::CargoBayFull);

            if timer.finished() {
                inventory_full_notification.0 = None;
            }
        } else {
            context_clues_res.0.remove(&ContextClue::CargoBayFull);
        }
    }

    pub fn remove_post_animation_text(
        mut commands: Commands,
        mut tween_completed: EventReader<TweenCompleted>
    ) {
        for evt in tween_completed.iter() {
            if evt.user_data == 111 {
                commands
                    .entity(evt.entity).despawn_recursive();
            }
        }
    }

    pub fn ablate_astroids(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut astroids_query: Query<(Entity, &mut Astroid, &GlobalTransform), With<Astroid>>,
        mut ablate_event_reader: EventReader<AblateEvent>,
    ) {

        let font = asset_server.load("fonts/FiraMono-Regular.ttf");

        for ablate_event in ablate_event_reader.iter() {

            let mut rng = rand::thread_rng();
            // let split_angle = rng.gen_range(0.0..PI / 4.0); TODO: Might keep splititng astroids

            if let Ok((ent, mut astroid_to_ablate, _g_trans)) = astroids_query.get_mut(ablate_event.0) {

                let damaged_health = astroid_to_ablate.health.current() - LASER_DAMAGE;
                astroid_to_ablate.health.set_current(damaged_health);

                if damaged_health < 0.0 {
                    commands.entity(ent).despawn_recursive();
                }

                let n: u8 = rng.gen();
                if n > 10 {
                    return;
                }

                let tween = Tween::new(
                    EaseFunction::ExponentialInOut,
                    std::time::Duration::from_millis(3000),
                    TextColorLens {
                        start: Color::Rgba { red: 255.0, green: 0.0, blue: 0.0, alpha: 1.0 },
                        end: Color::Rgba { red: 255.0, green: 0.0, blue: 0.0, alpha: 0.0 },
                        section: 0,
                    },
                )
                .with_repeat_count(RepeatCount::Finite(1))
                .with_completed_event(111);

                commands.spawn((
                    Text2dBundle {
                        text: Text::from_section(
                            "-1HP",
                            TextStyle {
                                font: font.clone(),
                                font_size: 32.0,
                                color: Color::RED,
                            },
                        ),
                        transform: Transform {
                            translation: (ablate_event.1 + ablate_event.2.normalize() * 100.0).extend(999.0),
                            ..default()
                        },
                        ..default()
                    },
                    Animator::new(tween),
                ));

                // TODO: The new comp distance shouldn't be constant it should update based on player distance from base
                let astroid = Astroid::new_with(AstroidSize::OreChunk, AstroidComposition::new_with_distance(100.0));

                let astroid_ent = commands
                    .spawn((
                        astroid.clone(),
                        ShapeBundle {
                            path: GeometryBuilder::build_as(&astroid.polygon()),
                            transform: Transform::from_xyz(ablate_event.1.x, ablate_event.1.y, 0.0),
                            ..default()
                        },
                        Fill::color(Color::DARK_GRAY),
                        RigidBody::Dynamic,
                        Velocity {
                            linvel: ablate_event.2,
                            angvel: 0.0,
                        },
                        Sleeping::disabled(),
                        Ccd::enabled(),
                        Collider::convex_hull(&astroid.polygon().points).unwrap(),
                        ActiveEvents::COLLISION_EVENTS,
                        ReadMassProperties(MassProperties::default()),
                        Restitution::coefficient(0.01),
                        Name::new("Astroid"),
                    )).id();

                // If the astroid is an ore chunk, add Collectible Tag
                if astroid.clone().size == AstroidSize::OreChunk {
                    commands.entity(astroid_ent).insert(Collectible);
                }

            }


        }

    }

    pub fn split_astroids_over_split_ratio(
        mut commands: Commands,
        mut astroid_query: Query<(Entity, &mut Astroid, &GlobalTransform, &Splittable)>
    ) {
        for (ent, astroid, g_t, split) in astroid_query.iter_mut() {

            if astroid.health.current_percent() < split.0 {
                Self::split_astroid(&mut commands, ent, &astroid, g_t.translation().truncate());
            }

        }


    }

    pub fn split_astroid(
        commands: &mut Commands,
        astroid_ent: Entity,
        astroid: &Astroid,
        astroid_translation: Vec2,
        // projectile_velocity: &Velocity,
    ) {
        // let mut rng = rand::thread_rng();
        // let split_angle = rng.gen_range(0.0..PI / 4.0);

        // let right_velocity = projectile_velocity
        //     .linvel
        //     .rotate(Vec2::from_angle(split_angle))
        //     * 0.5;
        // let left_velocity = projectile_velocity
        //     .linvel
        //     .rotate(Vec2::from_angle(-split_angle))
        //     * 0.5;
        let right_velocity = Vec2::ZERO;
        let left_velocity = Vec2::ZERO;


        match &astroid.size {
            AstroidSize::Small => {
                let left_comp = astroid.composition.jitter();
                let right_comp = astroid.composition.jitter();

                AstroidPlugin::spawn_astroid(
                    commands,
                    AstroidSize::OreChunk,
                    right_comp,
                    right_velocity,
                    astroid_translation,
                );
                AstroidPlugin::spawn_astroid(
                    commands,
                    AstroidSize::OreChunk,
                    left_comp,
                    left_velocity,
                    astroid_translation,
                );
                commands.entity(astroid_ent).despawn_recursive();
            }
            AstroidSize::Medium => {
                AstroidPlugin::spawn_astroid(
                    commands,
                    AstroidSize::Small,
                    astroid.composition.jitter(),
                    right_velocity,
                    astroid_translation,
                );
                AstroidPlugin::spawn_astroid(
                    commands,
                    AstroidSize::Small,
                    astroid.composition.jitter(),
                    left_velocity,
                    astroid_translation,
                );
                commands.entity(astroid_ent).despawn_recursive();
            }
            AstroidSize::Large => {
                AstroidPlugin::spawn_astroid(
                    commands,
                    AstroidSize::Medium,
                    astroid.composition.jitter(),
                    right_velocity,
                    astroid_translation,
                );
                AstroidPlugin::spawn_astroid(
                    commands,
                    AstroidSize::Medium,
                    astroid.composition.jitter(),
                    left_velocity,
                    astroid_translation,
                );
                commands.entity(astroid_ent).despawn_recursive();
            }
            _ => {}
        }
    }

}


