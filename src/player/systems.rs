use avian2d::math::PI;
use avian2d::prelude::*;
use bevy::color::palettes::css::WHITE;
use bevy::math::DVec2;
use bevy::prelude::*;
use ordered_float::OrderedFloat;

use super::components::Player;
use super::resources::EmptyInventoryDepositTimer;

use crate::battery::events::{BatteryEvent, BatteryEventType};
use crate::camera::components::CameraTarget;
use crate::health::components::Health;
use crate::inventory::components::{Capacity, Inventory};
use crate::inventory::systems::attach_inventory_to_entity;
use crate::laser::events::LaserEvent;
use crate::player_input::resources::MouseWorldPosition;
use crate::space_station::components::SpaceStation;
use crate::ui::context_clue::resources::{ContextClue, ContextClues};
use crate::upgrades::{components::UpgradesComponent, events::UpgradeEvent};
use crate::{
    battery::{
        components::Battery,
        //  events::DrainBatteryEvent
    },
    rcs::components::RCSBooster,
};

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let player_poly = RegularPolygon::new(20.0, 3);

    let player = commands
        .spawn(Player::new())
        .insert((Name::new("Player"), UpgradesComponent::new()))
        .insert(CameraTarget)
        .insert((
            RigidBody::Dynamic,
            Mass(1.0),
            AngularDamping(0.99),
            ConstantForce::new(0.0, 0.0),
            AngularVelocity::ZERO,
            LinearVelocity::ZERO,
            Friction::new(10.0),
            Collider::convex_hull(
                player_poly
                    .vertices(0.0)
                    .into_iter()
                    .map(|point| DVec2::new(point.x as f64, point.y as f64))
                    .collect(),
            )
            .unwrap(),
            Health::new(),
            Battery::new(),
            RCSBooster::new(),
        ))
        .insert((
            Mesh2d(meshes.add(player_poly)),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(WHITE))),
            Transform::from_xyz(0.0, 0.0, 2.0),
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

pub fn ship_rotate_towards_mouse(
    mouse_position: Res<MouseWorldPosition>,
    mut player_query: Query<(&mut Player, &mut Transform, &mut AngularVelocity)>,
) {
    let cursor_pos = mouse_position.0;
    let (_player, player_trans, mut ang_velocity) = player_query.single_mut().expect("No Player");

    const SPIN_ACCELERATION: f64 = 500.0;

    let player_to_mouse = (cursor_pos - player_trans.translation.truncate())
        .normalize()
        .as_dvec2();
    let player_ship_rotation = (player_trans.rotation * Vec3::Y)
        .truncate()
        .normalize()
        .as_dvec2();

    let ship_angle_difference_percent = player_to_mouse.angle_to(player_ship_rotation) / PI;

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
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<MouseButton>>,
    mut player: Query<(Entity, &Battery, &mut Transform, &GlobalTransform), With<Player>>,
    mut laser_event_writer: MessageWriter<LaserEvent>,
    // mut battery_events: MessageWriter<DrainBatteryEvent>,
) {
    let (entity, battery, player_transform, player_global_trans) =
        player.single_mut().expect("No Player");
    let player_direction = (player_transform.rotation * Vec3::Y).truncate().normalize();

    // Update Line and Opacity When Fired

    if keyboard_input.pressed(MouseButton::Left) {
        if battery.is_empty() {
            return;
        }

        let ray_pos = player_global_trans.translation().truncate();
        let ray_dir = player_direction;

        laser_event_writer.write(LaserEvent(true, ray_pos, ray_dir));
        commands.trigger(BatteryEvent {
            entity,
            event_type: BatteryEventType::Drain,
            amount: 1.0,
        });
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

        if timer.is_finished() {
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

// TODO: Move to upgrades modle
/// Perfom a smelt action with a recipe provided by the SmeltEvent.
pub fn on_upgrade_event(
    mut reader: MessageReader<UpgradeEvent>,
    mut base_station_query: Query<(&SpaceStation, &mut Inventory), With<SpaceStation>>,
    mut player_query: Query<(&mut Player, &mut UpgradesComponent), Without<SpaceStation>>, // mut refinery_timer: ResMut<RefineryTimer>,
) {
    for event in reader.read() {
        println!("Upgrade Event Detected!");
        let (_base_station, mut inventory) =
            base_station_query.single_mut().expect("No Base Station");
        let (mut player, mut upgrades) = player_query.single_mut().expect("No Player Updates");

        let upgrade = event.0;
        println!("{:?}", upgrade);

        upgrades.upgrade(upgrade, &mut player, &mut inventory);
    }
}
