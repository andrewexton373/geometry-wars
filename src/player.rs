use crate::astroid_size::Collectible;
use crate::base_station::{BaseStation, CanDeposit};
use crate::battery::Battery;
use crate::crosshair::Crosshair;
use crate::engine::Engine;
use crate::events::LaserEvent;
use crate::game_ui::{ContextClue, ContextClues};
use crate::health::Health;
use crate::inventory::{Capacity, Inventory, InventoryPlugin};
use crate::player_input::MouseWorldPosition;
use crate::upgrades::{UpgradeEvent, UpgradesComponent};
use crate::PIXELS_PER_METER;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{self as lyon, Fill, GeometryBuilder, ShapeBundle};
use bevy_prototype_lyon::shapes;
use bevy_xpbd_2d::prelude::*;
use bevy_xpbd_2d::components::{Collider, ExternalForce, LinearVelocity, RigidBody};
use bevy_xpbd_2d::parry::shape::{ConvexPolygon, SharedShape};
// use bevy_rapier2d::prelude::*;
use ordered_float::OrderedFloat;
use std::f32::consts::PI;
use bevy::input::gamepad::GamepadEvent::Axis;

pub struct PlayerPlugin;

#[derive(Resource)]
pub struct EmptyInventoryDepositTimer(Option<Timer>);

#[derive(Component, Default)]
pub struct Player {
    pub health: Health,
    pub battery: Battery,
    pub engine: Engine,
}

impl Player {
    fn new() -> Player {
        Player {
            health: Health::new(),
            battery: Battery::new(),
            engine: Engine::new(), // upgrades: UpgradesComponent::new()
        }
    }

    // TODO Move these to health and battery respectively...
    pub fn take_damage(&mut self, damage: f32) {
        let modified_health = self.health.current() - damage;
        self.health.set_current(modified_health);
    }

    pub fn repair_damage(&mut self, amount: f32) {
        let updated_health = self.health.current() + amount;
        self.health.set_current(updated_health);
    }

    pub fn drain_battery(&mut self, amount: f32) {
        let updated_capacity = self.battery.current() - amount;
        self.battery.set_current(updated_capacity);
    }

    pub fn charge_battery(&mut self, amount: f32) {
        let updated_capacity = self.battery.current() + amount;
        self.battery.set_current(updated_capacity);
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here

        app
            .insert_resource(EmptyInventoryDepositTimer(None))
            .add_event::<UpgradeEvent>()
            .add_systems(Startup, Self::spawn_player)
            .add_systems(Update, (
                Self::update_player_mass,
                Self::player_movement.after(Self::update_player_mass),
                Self::ship_rotate_towards_mouse.after(Self::player_movement),
                Self::player_fire_laser,
                Self::player_camera_control,
                Self::player_deposit_control,
                Self::gravitate_collectibles,
                Self::trickle_charge,
                Self::ship_battery_is_empty_context_clue,
                Self::display_empty_ship_inventory_context_clue,
                Self::on_upgrade_event
            ));
    }
}

impl PlayerPlugin {
    fn spawn_player(mut commands: Commands) {

        let player_poly = shapes::Polygon {
            points: vec![
                Vec2 {x: 0.0, y: 2.0 * crate::PIXELS_PER_METER},
                Vec2 {x: 1.0 * crate::PIXELS_PER_METER, y: -1.0 * crate::PIXELS_PER_METER},
                Vec2 {x: -1.0 * crate::PIXELS_PER_METER, y: -1.0 * crate::PIXELS_PER_METER}
            ],
            closed: true
        };        

        let player = commands.spawn(Player::new())
            .insert((
                Name::new("Player"),
                UpgradesComponent::new(),
            ))
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
            ))
            .insert((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&player_poly),
                    ..default()
                },
                Fill::color(Color::WHITE),
            )).id();

        InventoryPlugin::attach_inventory_to_entity(
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

    fn trickle_charge(mut player_query: Query<&mut Player>) {
        let mut player = player_query.single_mut();
        player.charge_battery(0.00001);
    }

    fn player_movement(
        keyboard_input: Res<Input<KeyCode>>,
        mut player_query: Query<
            (
                &mut Player,
                &mut Transform,
                &mut LinearVelocity,
                &mut ExternalForce,
            ),
            (With<Player>, Without<Crosshair>),
        >,
    ) {
        const ACCELERATION: f32 = 120000.0 * PIXELS_PER_METER;

        let (mut player, mut transform, mut velocity, mut ext_force) = player_query.single_mut();

        let mut thrust = Vec2::ZERO;

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

        thrust *= player.engine.power_level;

        // If player has battery capacity remaining, apply controlled thrust.
        if player.battery.current() > 0.0 {
            let force = thrust.normalize_or_zero() * ACCELERATION;
            let energy_spent = force.length() / 500000.0; // TODO: magic number

            player.drain_battery(energy_spent);

            ext_force.set_force(force);

            // TODO: Remove Playing Component from Respective Particle System

            if force.length() > 0.0 {
                // TODO: Add Playing Component to Respective Particle System
            }
        }

        // velocity.angvel = 0.0; // Prevents spin on astrid impact

        // TODO: is there a better place to do this?
        transform.scale = Vec3::new(2.0, 2.0, 1.0);
    }

    fn ship_rotate_towards_mouse(
        mouse_position: Res<MouseWorldPosition>,
        mut player_query: Query<(&mut Player, &mut Transform, &mut AngularVelocity), Without<Crosshair>>,
    ) {
        let cursor_pos = mouse_position.0;
        let (_player, player_trans, mut ang_velocity) = player_query.single_mut();

        const SPIN_ACCELERATION: f32 = 500.0;

        let player_to_mouse = (cursor_pos - player_trans.translation.truncate()).normalize();
        let player_ship_rotation = (player_trans.rotation * Vec3::Y).truncate().normalize();

        let ship_angle_difference_percent = Vec2::angle_between(
            player_to_mouse,
            player_ship_rotation,
        ) / PI;

        //Rotate towards position mouse is on
        if ship_angle_difference_percent > 0.001 {
            ang_velocity.0 = -SPIN_ACCELERATION * ship_angle_difference_percent.powf(2.0);
        } else if ship_angle_difference_percent < -0.001 {
            ang_velocity.0 = SPIN_ACCELERATION * ship_angle_difference_percent.powf(2.0);
        } else {
            ang_velocity.0 = 0.0;
        }
    }

    fn player_fire_laser(
        keyboard_input: Res<Input<MouseButton>>,
        mut player_query: Query<(
            Entity,
            &mut Player,
            &mut Transform,
            &GlobalTransform,
        )>,
        mut laser_event_writer: EventWriter<LaserEvent>,
    ) {
        let (_ent, mut player, player_transform, player_global_trans) = player_query.single_mut();
        let player_direction = (player_transform.rotation * Vec3::Y).truncate().normalize();

        // Update Line and Opacity When Fired

        if keyboard_input.pressed(MouseButton::Left) {
            if player.battery.is_empty() {
                return;
            }

            let ray_pos = player_global_trans.translation().truncate();
            let ray_dir = player_direction;

            // dbg!("{:?}", ray_pos);

            laser_event_writer.send(LaserEvent(true, ray_pos, ray_dir));
            player.drain_battery(1.0);
        }
    }

    fn player_camera_control(
        kb: Res<Input<KeyCode>>,
        time: Res<Time>,
        mut query: Query<&mut OrthographicProjection, With<Camera2d>>,
    ) {
        let dist = 0.75 * time.delta().as_secs_f32();

        for mut projection in query.iter_mut() {
            let mut log_scale = projection.scale.ln();

            if kb.pressed(KeyCode::Period) {
                log_scale -= dist;
            }
            if kb.pressed(KeyCode::Comma) {
                log_scale += dist;
            }

            projection.scale = log_scale.exp();
        }
    }

    fn player_deposit_control(
        kb: Res<Input<KeyCode>>,
        can_deposit: Res<CanDeposit>,
        mut empty_deposit_timer: ResMut<EmptyInventoryDepositTimer>,
        mut player_query: Query<&mut Inventory, (With<Player>, Without<BaseStation>)>,
        mut base_station_query: Query<&mut Inventory, (With<BaseStation>, Without<Player>)>,
    ) {
        // If player pressed space and they're in depositing range
        if kb.just_pressed(KeyCode::Space) && can_deposit.0 {
            let mut player_inventory = player_query.single_mut();
            let mut base_station_inventory = base_station_query.single_mut();

            if player_inventory.items.is_empty() {
                let timer = empty_deposit_timer.as_mut();
                *timer =
                    EmptyInventoryDepositTimer(Some(Timer::from_seconds(3.0, TimerMode::Once)));
            }

            for item in player_inventory.clone().items.iter() {
                base_station_inventory.add_to_inventory(item);
                player_inventory.remove_from_inventory(item);
            }
        }
    }

    fn display_empty_ship_inventory_context_clue(
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

    fn gravitate_collectibles(
        mut collectible_query: Query<(Entity, &Collectible, &Transform, &mut LinearVelocity)>,
        player_query: Query<(Entity, &Player, &Transform), With<Player>>,
    ) {
        const MAX_GRAVITATION_DISTANCE: f32 = 30.0 * crate::PIXELS_PER_METER;
        let (_player_ent, _player, player_transform) = player_query.single();

        for (_ent, _collectible, collectible_tranform, mut velocity) in
            collectible_query.iter_mut()
        {
            let distance_to_player_from_collectible = player_transform
                .translation
                .truncate()
                .distance(collectible_tranform.translation.truncate());
            if distance_to_player_from_collectible < MAX_GRAVITATION_DISTANCE {
                let percent_distance_from_max =
                    distance_to_player_from_collectible / MAX_GRAVITATION_DISTANCE;
                let direction_to_player_from_collectible =
                    (player_transform.translation.truncate()
                        - collectible_tranform.translation.truncate())
                    .normalize();
                let gravitation_factor = 1.0 - percent_distance_from_max;
                velocity.0 += direction_to_player_from_collectible
                    * gravitation_factor
                    * 5.0
                    * crate::PIXELS_PER_METER;
            }
        }
    }

    /// Updates the player mass with the ship's net mass for physics engine.
    fn update_player_mass(
        mut player_query: Query<(&Player, &Inventory, &mut Mass)>,
    ) {
        const PLAYER_MASS: f32 = 1000.0;

        for (_player, inventory, mut mass) in player_query.iter_mut() {
            let inventory_weight = inventory.gross_material_weight();
            mass.0 = (inventory_weight + PLAYER_MASS).0;
        }
    }

    fn ship_battery_is_empty_context_clue(
        mut context_clues_res: ResMut<ContextClues>,
        player_query: Query<&Player>,
    ) {
        for player in player_query.into_iter() {
            if player.battery.is_empty() {
                context_clues_res.0.insert(ContextClue::ShipFuelEmpty);
            } else {
                context_clues_res.0.remove(&ContextClue::ShipFuelEmpty);
            }
        }
    }

    /// Perfom a smelt action with a recipe provided by the SmeltEvent.
    fn on_upgrade_event(
        mut reader: EventReader<UpgradeEvent>,
        mut base_station_query: Query<(&BaseStation, &mut Inventory), With<BaseStation>>,
        mut player_query: Query<(&mut Player, &mut UpgradesComponent), Without<BaseStation>>, // mut refinery_timer: ResMut<RefineryTimer>,
    ) {
        for event in reader.read() {
            println!("Upgrade Event Detected!");
            let (_base_station, mut inventory) = base_station_query.single_mut();
            let (mut player, mut upgrades) = player_query.single_mut();

            let upgrade = event.0.clone();
            println!("{:?}", upgrade);

            upgrades.upgrade(upgrade, &mut player, &mut inventory);
        }
    }
}
