use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_xpbd_2d::prelude::*;
use ordered_float::OrderedFloat;
use std::f32::consts::PI;

use super::components::Player;
use super::resources::EmptyInventoryDepositTimer;

use crate::{battery::{
    components::Battery,
    events::{ChargeBatteryEvent, DrainBatteryEvent},
}, rcs::{components::RCSBooster, events::RCSThrustVectorEvent}};
use crate::camera::components::CameraTarget;
use crate::collectible::components::Collectible;
use crate::crosshair::components::Crosshair;
use crate::health::components::Health;
use crate::inventory::components::{Capacity, Inventory};
use crate::inventory::plugin::InventoryPlugin;
use crate::inventory::systems::attach_inventory_to_entity;
use crate::laser::events::LaserEvent;
use crate::player_input::resources::MouseWorldPosition;
use crate::space_station::components::SpaceStation;
use crate::space_station::resources::CanDeposit;
use crate::ui::context_clue::resources::{ContextClue, ContextClues};
use crate::upgrades::{components::UpgradesComponent, events::UpgradeEvent};
use crate::PIXELS_PER_METER;

pub fn spawn_player(mut commands: Commands) {
    let player_poly = shapes::Polygon {
        points: vec![
            Vec2 {
                x: 0.0,
                y: 4.0 * crate::PIXELS_PER_METER,
            },
            Vec2 {
                x: 2.0 * crate::PIXELS_PER_METER,
                y: -2.0 * crate::PIXELS_PER_METER,
            },
            Vec2 {
                x: -2.0 * crate::PIXELS_PER_METER,
                y: -2.0 * crate::PIXELS_PER_METER,
            },
        ],
        closed: true,
    };

    let player = commands
        .spawn(Player::new())
        .insert((Name::new("Player"), UpgradesComponent::new()))
        .insert(CameraTarget)
        .insert((
            RigidBody::Dynamic,
            Mass(1.0),
            Inertia(1.0),
            AngularDamping(0.99),
            ExternalForce::ZERO,
            AngularVelocity::ZERO,
            LinearVelocity::ZERO,
            Friction::new(10.0),
            Collider::convex_hull(player_poly.points.clone()).unwrap(),
            Health::new(),
            Battery::new(),
            RCSBooster::new()
        ))
        .insert((
            ShapeBundle {
                path: GeometryBuilder::build_as(&player_poly),
                spatial: SpatialBundle {
                    transform: Transform {
                        translation: Vec3 {
                            x: 0.0,
                            y: 0.0,
                            z: 2.0,
                        },
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
            Fill::color(Color::WHITE),
        ))
        .id();

    attach_inventory_to_entity(
        &mut commands,
        Inventory {
            items: Vec::new(),
            capacity: Capacity {
                maximum: OrderedFloat(200.0),
            },
        },
        player,
    );
}

pub fn trickle_charge(
    mut battery_q: Query<(Entity, &Battery), With<Player>>,
    mut battery_events: EventWriter<ChargeBatteryEvent>,
) {
    if let Ok((entity, battery)) = battery_q.get_single() {
        battery_events.send(ChargeBatteryEvent {
            entity,
            charge: 0.01,
        });
    }
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(
        Entity,
        &Player,
        &Battery,
        &mut Transform,
        &mut ExternalForce,
    )>,
    mut battery_events: EventWriter<DrainBatteryEvent>,
    mut thrust_vector_events: EventWriter<RCSThrustVectorEvent>,
) {
    let (entity, player, battery, mut transform, mut ext_force) = player_query.single_mut();

    let mut thrust: Vec2 = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
        thrust += -Vec2::X;
    }

    if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
        thrust += Vec2::X;
    }

    if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
        thrust += Vec2::Y;
    }

    if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
        thrust += -Vec2::Y;
    }

    // If the players ship has no remaining battery capacity, end early.
    if battery.current() <= 0.0 { return };

    const ACCELERATION: f32 = 1000.0 * PIXELS_PER_METER;

    let force = thrust.normalize_or_zero() * ACCELERATION;
    let energy_spent = force.length() / 5000000.0; // TODO: magic number

    if force == Vec2::ZERO { return; }

    battery_events.send(DrainBatteryEvent {
        entity,
        drain: energy_spent,
    });


    thrust_vector_events.send(RCSThrustVectorEvent {
        entity,
        thrust_vector: force
    });

}

pub fn ship_rotate_towards_mouse(
    mouse_position: Res<MouseWorldPosition>,
    mut player_query: Query<
        (&mut Player, &mut Transform, &mut AngularVelocity),
        Without<Crosshair>,
    >,
) {
    let cursor_pos = mouse_position.0;
    let (_player, player_trans, mut ang_velocity) = player_query.single_mut();

    const SPIN_ACCELERATION: f32 = 500.0;

    let player_to_mouse = (cursor_pos - player_trans.translation.truncate()).normalize();
    let player_ship_rotation = (player_trans.rotation * Vec3::Y).truncate().normalize();

    let ship_angle_difference_percent =
        Vec2::angle_between(player_to_mouse, player_ship_rotation) / PI;

    //Rotate towards position mouse is on
    if ship_angle_difference_percent > 0.001 {
        ang_velocity.0 = -SPIN_ACCELERATION * ship_angle_difference_percent.powf(2.0);
    } else if ship_angle_difference_percent < -0.001 {
        ang_velocity.0 = SPIN_ACCELERATION * ship_angle_difference_percent.powf(2.0);
    } else {
        ang_velocity.0 = 0.0;
    }
}

pub fn player_fire_laser(
    keyboard_input: Res<Input<MouseButton>>,
    mut player_query: Query<(
        Entity,
        &mut Player,
        &Battery,
        &mut Transform,
        &GlobalTransform,
    )>,
    mut laser_event_writer: EventWriter<LaserEvent>,
    mut battery_events: EventWriter<DrainBatteryEvent>,
) {
    let (entity, mut player, battery, player_transform, player_global_trans) =
        player_query.single_mut();
    let player_direction = (player_transform.rotation * Vec3::Y).truncate().normalize();

    // Update Line and Opacity When Fired

    if keyboard_input.pressed(MouseButton::Left) {
        if battery.is_empty() {
            return;
        }

        let ray_pos = player_global_trans.translation().truncate();
        let ray_dir = player_direction;

        laser_event_writer.send(LaserEvent(true, ray_pos, ray_dir));
        battery_events.send(DrainBatteryEvent { entity, drain: 1.0 });
    }
}

pub fn display_empty_ship_inventory_context_clue(
    mut context_clues: ResMut<ContextClues>,
    mut empty_deposit_timer: ResMut<EmptyInventoryDepositTimer>,
    time: Res<Time>,
) {
    if let Some(timer) = empty_deposit_timer.0.as_mut() {
        timer.tick(time.delta());
        context_clues.0.insert(ContextClue::ShipInventoryEmpty);

        if timer.finished() {
            empty_deposit_timer.0 = None;
        }
    } else {
        context_clues.0.remove(&ContextClue::ShipInventoryEmpty);
    }
}

// TODO: Refector Collectibles into Module?

/// Updates the player mass with the ship's net mass for physics engine.
pub fn update_player_mass(mut player_query: Query<(&Player, &Inventory, &mut Mass)>) {
    const PLAYER_MASS: f32 = 1000.0;

    for (_player, inventory, mut mass) in player_query.iter_mut() {
        let inventory_weight = inventory.gross_material_weight();
        mass.0 = (inventory_weight + PLAYER_MASS).0;
    }
}

pub fn ship_battery_is_empty_context_clue(
    mut context_clues_res: ResMut<ContextClues>,
    mut battery_q: Query<&Battery, With<Player>>,
) {
    if let Ok(battery) = battery_q.get_single_mut() {
        if battery.is_empty() {
            context_clues_res.0.insert(ContextClue::ShipFuelEmpty);
        } else {
            context_clues_res.0.remove(&ContextClue::ShipFuelEmpty);
        }
    }
}

// TODO: Move to upgrades modle
/// Perfom a smelt action with a recipe provided by the SmeltEvent.
pub fn on_upgrade_event(
    mut reader: EventReader<UpgradeEvent>,
    mut base_station_query: Query<(&SpaceStation, &mut Inventory), With<SpaceStation>>,
    mut player_query: Query<(&mut Player, &mut UpgradesComponent), Without<SpaceStation>>, // mut refinery_timer: ResMut<RefineryTimer>,
) {
    for event in reader.read() {
        println!("Upgrade Event Detected!");
        let (_base_station, mut inventory) = base_station_query.single_mut();
        let (mut player, mut upgrades) = player_query.single_mut();

        let upgrade = event.0;
        println!("{:?}", upgrade);

        upgrades.upgrade(upgrade, &mut player, &mut inventory);
    }
}
