use crate::astroid::{Collectible};
use crate::base_station::{BaseStation, CanDeposit};
use crate::battery::Battery;
use crate::crosshair::Crosshair;
use crate::engine::Engine;
use crate::game_ui::{ContextClue, ContextClues};
use crate::health::Health;
use crate::inventory::{Capacity, Inventory, InventoryPlugin};
use crate::laser::{LaserEvent};
use crate::particles::{PlayerShipTrailParticles};
use crate::upgrades::{UpgradesComponent, UpgradeEvent};
use crate::{GameCamera, PIXELS_PER_METER};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_hanabi::ParticleEffect;
use bevy_prototype_lyon::prelude::{self as lyon, Fill, GeometryBuilder, ShapeBundle};
use bevy_rapier2d::prelude::*;
use ordered_float::OrderedFloat;
use std::f32::consts::PI;

pub struct PlayerPlugin;

#[derive(Resource)]
pub struct EmptyInventoryDepositTimer(Option<Timer>);

#[derive(Component, Default)]
pub struct Player {
    pub health: Health,
    pub battery: Battery,
    pub engine: Engine
}

impl Player {
    fn new() -> Player {
        Player {
            health: Health::new(),
            battery: Battery::new(),
            engine: Engine::new() // upgrades: UpgradesComponent::new()
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
            .add_event::<UpgradeEvent>()
            .add_startup_system(Self::spawn_player)
            .add_system(Self::update_player_mass)
            .add_system(Self::player_movement.after(Self::update_player_mass))
            .add_system(Self::ship_rotate_towards_mouse.after(Self::player_movement))
            // .add_system(Self::player_fire_weapon)
            .add_system(Self::player_fire_laser)
            .add_system(Self::player_camera_control)
            .add_system(Self::player_deposit_control)
            .add_system(Self::gravitate_collectibles)
            .add_system(Self::trickle_charge)
            .add_system(Self::ship_battery_is_empty_context_clue)
            .add_system(Self::display_empty_ship_inventory_context_clue)
            .add_system(Self::on_upgrade_event)
            .insert_resource(EmptyInventoryDepositTimer(None));
    }
}

impl PlayerPlugin {
    fn spawn_player(mut commands: Commands) {
        let points = vec![
            Vec2 {
                x: 0.0,
                y: 2.0 * crate::PIXELS_PER_METER,
            },
            Vec2 {
                x: 1.0 * crate::PIXELS_PER_METER,
                y: -1.0 * crate::PIXELS_PER_METER,
            },
            Vec2 {
                x: -1.0 * crate::PIXELS_PER_METER,
                y: -1.0 * crate::PIXELS_PER_METER,
            },
        ];

        let player_shape = lyon::shapes::Polygon {
            points,
            closed: true,
        };

        let player = commands
            .spawn((
                Player::new(),
                ShapeBundle {
                    path: GeometryBuilder::build_as(&player_shape),
                    transform: Transform {
                        translation: Vec3 {
                            x: 0.0,
                            y: 0.0,
                            z: 100.0,
                        },
                        ..Default::default()
                    },
                    ..default()
                },
                Fill::color(Color::CYAN),
                RigidBody::Dynamic,
                AdditionalMassProperties::Mass(10.0),
                ExternalForce {
                    force: Vec2::new(0.0, 0.0),
                    torque: 0.0,
                },
                Damping {
                    linear_damping: 0.8,
                    angular_damping: 0.0,
                },
                Velocity::zero(),
                Sleeping::disabled(),
                Ccd::enabled(),
                Collider::convex_hull(&player_shape.points).unwrap(),
                ActiveEvents::COLLISION_EVENTS,
                UpgradesComponent::new(),
                Name::new("Player"),
            )).id();

        InventoryPlugin::attach_inventory_to_entity(
            &mut commands,
            Inventory {
                items: Vec::new(),
                capacity: Capacity { maximum: OrderedFloat(200.0) },
            },
            player,
        );

    }

    fn trickle_charge(
        mut player_query: Query<&mut Player>,
    ) {
        let mut player = player_query.single_mut();
        player.charge_battery(0.00001);
    }

    fn player_movement(
        keyboard_input: Res<Input<KeyCode>>,
        mut player_query: Query<
            (
                &mut Player,
                &mut Transform,
                &mut Velocity,
                &mut ExternalForce,
            ),
            (With<Player>, Without<Crosshair>),
        >,
        mut effect: Query<
            (&mut ParticleEffect, &mut Transform),
            (With<PlayerShipTrailParticles>, Without<Player>),
        >,
    ) {
        const ACCELERATION: f32 = 12000.0 * PIXELS_PER_METER;

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

            ext_force.force = force;

            if force.length() > 0.0 {
                let (mut effect, mut effect_trans) = effect.single_mut();
                effect_trans.translation = transform.translation;
                effect.maybe_spawner().unwrap().reset();
            }
        }

        velocity.angvel = 0.0; // Prevents spin on astrid impact

        // TODO: is there a better place to do this?
        transform.scale = Vec3::new(2.0, 2.0, 1.0);
    }

    fn ship_rotate_towards_mouse(
        window_query: Query<&Window, With<PrimaryWindow>>,
        q_camera: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
        mut player_query: Query<(&mut Player, &mut Transform, &mut Velocity), Without<Crosshair>>,
    ) {
        // get the camera info and transform
        // assuming there is exactly one main camera entity, so query::single() is OK
        let (camera, camera_transform) = q_camera.single();

        let Ok(wnd) = window_query.get_single() else {
            return;
        };

        const SPIN_ACCELERATION: f32 = 0.4;

        let (_player, player_trans, mut velocity) = player_query.single_mut();

        // check if the cursor is inside the window and get its position
        if let Some(screen_pos) = wnd.cursor_position() {
            // get the size of the window
            let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

            // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
            let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

            // matrix for undoing the projection and camera transform
            let ndc_to_world =
                camera_transform.compute_matrix() * camera.projection_matrix().inverse();

            // use it to convert ndc to world-space coordinates
            let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

            // reduce it to a 2D value
            let world_pos: Vec2 = world_pos.truncate();

            let player_to_mouse =
                Vec2::new(player_trans.translation.x, player_trans.translation.y) - world_pos;
            let ship_angle_difference = Vec2::angle_between(
                player_to_mouse,
                (player_trans.rotation * Vec3::Y).truncate(),
            );

            //Rotate towards position mouse is on
            if ship_angle_difference > 0.0 {
                velocity.angvel += SPIN_ACCELERATION * (2.0 * PI - ship_angle_difference.abs());
            } else if ship_angle_difference < 0.0 {
                velocity.angvel += -SPIN_ACCELERATION * (2.0 * PI - ship_angle_difference.abs());
            }
        }
    }

    // TODO: I think a laser might be better, need to do some raycasting though.
    fn player_fire_laser(
        keyboard_input: Res<Input<MouseButton>>,
        mut player_query: Query<(Entity, &mut Player, &mut Transform, &GlobalTransform, &Velocity)>,
        mut laser_event_writer:EventWriter<LaserEvent>
    ) {

        let (_ent, mut player, transform, global_trans, _velocity) = player_query.single_mut();
        let player_direction = (transform.rotation * Vec3::Y).normalize();

        // Update Line and Opacity When Fired
        if keyboard_input.pressed(MouseButton::Left) {

            if player.battery.is_empty() { return; }

            let ray_dir = player_direction.truncate();
            let ray_pos = global_trans.translation().truncate() + ray_dir * 100.0; // move racasting ray ahead of ship to avoid contact (there's probably a better way lol)
            
            laser_event_writer.send(LaserEvent(true, ray_pos, ray_dir));
            player.drain_battery(1.0);

        } else {
            // Raycast to Find Target
            let ray_dir = player_direction.truncate();
            let ray_pos = global_trans.translation().truncate() + ray_dir * 100.0; // move racasting ray ahead of ship to avoid contact (there's probably a better way lol)

            laser_event_writer.send(LaserEvent(false, ray_pos, ray_dir));
        }


    }


    // fn player_fire_weapon(
    //     mut commands: Commands,
    //     keyboard_input: Res<Input<MouseButton>>,
    //     player_query: Query<(&mut Player, &mut Transform, &Velocity)>,
    // ) {
    //     // should be just pressed, but it's fun with keyboard_input.pressed()d
    //     if keyboard_input.just_pressed(MouseButton::Left) {
    //         let (player, transform, _velocity) = player_query.single();

    //         // why does this work? https://www.reddit.com/r/rust_gamedev/comments/rphgsf/calculating_bullet_x_and_y_position_based_off_of/
    //         // FIXME: clean this up, it's confusing..., should also be using velocity here.
    //         let player_velocity = (transform.rotation * Vec3::Y)
    //             + Vec3::new(player.delta_x, player.delta_y, 0.0) * PIXELS_PER_METER;

    //         ProjectilePlugin::spawn_projectile(
    //             &mut commands,
    //             transform.translation.truncate(),
    //             player_velocity.truncate(),
    //         );
    //     }
    // }

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
                *timer = EmptyInventoryDepositTimer(Some(Timer::from_seconds(3.0, TimerMode::Once)));
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
        time: Res<Time>
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
        mut collectible_query: Query<(Entity, &Collectible, &Transform, &mut Velocity)>,
        player_query: Query<(Entity, &Player, &Transform), With<Player>>,
    ) {
        const MAX_GRAVITATION_DISTANCE: f32 = 30.0 * crate::PIXELS_PER_METER;
        let (_player_ent, _player, player_transform) = player_query.single();

        for (_ent, _collectible, collectible_tranform, mut veclocity) in
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
                veclocity.linvel += direction_to_player_from_collectible
                    * gravitation_factor
                    * 5.0
                    * crate::PIXELS_PER_METER;
            }
        }
    }

    /// Updates the player mass with the ship's net mass for rapier2d movement physics.
    fn update_player_mass(
        mut player_query: Query<(&Player, &Inventory, &mut AdditionalMassProperties)>,
    ) {
        const PLAYER_MASS: f32 = 100.0;

        for (_player, inventory, mut mass_properties) in player_query.iter_mut() {
            let inventory_weight = inventory.gross_material_weight();
            let mass_properties = mass_properties.as_mut();
            *mass_properties = AdditionalMassProperties::Mass(inventory_weight.0 + PLAYER_MASS);
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
        for event in reader.iter() {
            println!("Upgrade Event Detected!");
            let (_base_station, mut inventory) = base_station_query.single_mut();
            let (mut player, mut upgrades) = player_query.single_mut();

            let upgrade = event.0.clone();
            println!("{:?}", upgrade);

            upgrades.upgrade(upgrade, &mut player, &mut inventory);
        }
    }
}
