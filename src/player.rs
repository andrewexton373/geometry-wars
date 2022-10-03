use bevy::reflect::FromReflect;
use bevy::utils::HashMap;
use bevy::{prelude::*};
use bevy_prototype_lyon::shapes::{Polygon, RegularPolygon};
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::prelude as lyon;
use bevy::render::camera::RenderTarget;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use kayak_ui::core::{Binding, Bound, MutableBound};
use std::f32::consts::PI;
use std::fmt;
use crate::{PIXELS_PER_METER, GameCamera};
use crate::astroid::{Collectible, Astroid};
use crate::healthbar::Health;
use crate::projectile::{ProjectilePlugin};
use crate::crosshair::Crosshair;
use crate::astroid::AstroidMaterial;

pub struct PlayerPlugin;

const INVENTORY_SIZE: usize = 20;

#[derive(Component, Default, Debug, Clone, PartialEq)]
pub struct Inventory {
    // pub items: HashMap<AstroidMaterial, f32>,
    pub items: [Option<ItemAndWeight>; 20]
}

#[derive(Component, Default, Debug, Inspectable, Copy, Clone, PartialEq, PartialOrd)]
pub struct ItemAndWeight {
    pub item: AstroidMaterial,
    pub weight: f32
}

impl fmt::Display for ItemAndWeight {
    fn fmt(&self, f: &mut  fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Item: {:?} | Weight: {}", &self.item, &self.weight)
    }
}

#[derive(Component, Inspectable, Default)]
pub struct Player {
    // TODO: refactor into velocity Vec2
    pub delta_x: f32,
    pub delta_y: f32,
    pub delta_rotation: f32,
    pub health: Health,
    // pub inventory: Inventory
}

impl Player {
    fn new() -> Player {
        Player { 
            delta_x: 0.0,
            delta_y: 0.0,
            delta_rotation: 0.0,
            health: Health { current: 100.0, maximum: 100.0 },
            // inventory: Inventory { items: [None; INVENTORY_SIZE] }
        }
    }

    pub fn take_damage(&mut self, damage: f32) {
        let modified_health = self.health.current - damage;
        let modified_health = modified_health.clamp(0.0, self.health.maximum);
        self.health.current = modified_health;
    }

    // I'd rather have the inventory be a hashmap, but was struggling with bevy-inspector traits
    pub fn add_to_inventory(&mut self, mut inventory: ResMut<Inventory>, material: AstroidMaterial, weight: f32) {
        let mut current_inventory = inventory.items;
        let mut temp_hash_map = HashMap::new();

        // Fill temp hash map with current inventory items
        for item in current_inventory.into_iter() {
            if item.is_some() {
                temp_hash_map.insert(item.unwrap().item, item.unwrap().weight);
            }
        }

        // If hasmap already contains the material, add weight to existing material's weight
        if let Some(value) = temp_hash_map.get_mut(&material) {
            *value = *value + weight;
        
        // Otherwise add material and it's respective weight as new entry
        } else {
            temp_hash_map.insert(material, weight);
        }

        let mut updated_inventory: Vec<Option<ItemAndWeight>> = temp_hash_map.into_iter().map(|(k, v)| Some(ItemAndWeight {item: k, weight: v})).collect();
        updated_inventory.sort_by(|a, b| {
            let a = a.unwrap();
            let b = b.unwrap();
            a.item.partial_cmp(&b.item).unwrap()
        });
        while updated_inventory.len() < 20
        {
            updated_inventory.push(None);
        }

        inventory.items = updated_inventory.try_into().unwrap_or_else(|v: Vec<Option<ItemAndWeight>>| panic!("Expected a Vec of length {} but it was {}", 4, v.len()));

    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here

        app
            .add_startup_system(Self::spawn_player.label("spawn_player"))
            .add_system(Self::player_movement)
            .add_system(Self::ship_rotate_towards_mouse)
            .add_system(Self::player_fire_weapon)
            .add_system(Self::player_camera_control)
            .add_system(Self::gravitate_collectibles)
            .register_inspectable::<Player>();
    }
}

impl PlayerPlugin {
    fn spawn_player(
        mut commands: Commands
    ) {
        let points = vec![
            Vec2 {x: 0.0, y: 2.0 * crate::PIXELS_PER_METER},
            Vec2 {x: 1.0 * crate::PIXELS_PER_METER, y: -1.0 * crate::PIXELS_PER_METER},
            Vec2{x: -1.0 * crate::PIXELS_PER_METER, y: -1.0* crate::PIXELS_PER_METER}
        ];

        let player_shape = lyon::shapes::Polygon {
            points: points,
            closed: true
        };

        let _player = commands.spawn()
            .insert(Player::new())
            .insert_bundle(lyon::GeometryBuilder::build_as(
                &player_shape,
                lyon::DrawMode::Outlined {
                    fill_mode: lyon::FillMode::color(Color::CYAN),
                    outline_mode: lyon::StrokeMode::new(Color::WHITE, 2.0),
                },
                Default::default()
            ))
            .insert(RigidBody::Dynamic)
            .insert(Velocity::zero())
            .insert(Sleeping::disabled())
            .insert(Ccd::enabled())
            .insert(Collider::convex_hull(&player_shape.points).unwrap())
            .insert(Transform::default())
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(Restitution::coefficient(1.0))
            .insert(Name::new("Player"))
            .id();
    }

    fn player_movement(
        keyboard_input: Res<Input<KeyCode>>,
        mut player_query: Query<(&mut Transform, &mut Velocity), (With<Player>, Without<Crosshair>)>,
    ) {
        const ACCELERATION: f32 =  3.0 * PIXELS_PER_METER;
        const DECLERATION: f32 = 0.95;

            let (mut transform, mut velocity) = player_query.single_mut();
    
            if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
                velocity.linvel += Vec2 {x: -ACCELERATION, y: 0.0 };
            }
        
            if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
                velocity.linvel += Vec2 {x: ACCELERATION, y: 0.0 };
            }
        
            if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
                velocity.linvel += Vec2 {x: 0.0, y: ACCELERATION };
            }
        
            if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
                velocity.linvel += Vec2 {x: 0.0, y: -ACCELERATION };
            }
        
            velocity.linvel *= DECLERATION;
            velocity.angvel = 0.0; // Prevents spin on astrid impact

            // TODO: is there a better place to do this?
            transform.scale = Vec3::new(2.0, 2.0, 1.0);
    
    }

    fn ship_rotate_towards_mouse(
        wnds: Res<Windows>,
        // query to get camera transform
        q_camera: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
        mut player_query: Query<(&mut Player, &mut Transform, &mut Velocity), Without<Crosshair>>
    ) {
        // get the camera info and transform
        // assuming there is exactly one main camera entity, so query::single() is OK
        let (camera, camera_transform) = q_camera.single();

        // get the window that the camera is displaying to (or the primary window)
        let wnd = if let RenderTarget::Window(id) = camera.target {
            wnds.get(id).unwrap()
        } else {
            wnds.get_primary().unwrap()
        };

        const SPIN_ACCELERATION: f32 = 0.4;
        const MAX_VELOCITY: f32 = 6.0;
    
        let (mut player, mut player_trans, mut velocity) = player_query.single_mut();
    
         // check if the cursor is inside the window and get its position
         if let Some(screen_pos) = wnd.cursor_position() {
            // get the size of the window
            let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
    
            // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
            let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
    
            // matrix for undoing the projection and camera transform
            let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
    
            // use it to convert ndc to world-space coordinates
            let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
    
            // reduce it to a 2D value
            let world_pos: Vec2 = world_pos.truncate();
    
            let player_to_mouse = Vec2::new(player_trans.translation.x, player_trans.translation.y) - world_pos;
            let ship_angle_difference = Vec2::angle_between(player_to_mouse, (player_trans.rotation * Vec3::Y).truncate());
    
            //Rotate towards position mouse is on
            if ship_angle_difference > 0.0 {
                // player.delta_rotation += SPIN_ACCELERATION * (2.0*PI - ship_angle_difference.abs());
                velocity.angvel += SPIN_ACCELERATION * (2.0*PI - ship_angle_difference.abs());
            } else
    
            if ship_angle_difference < 0.0 {
                // player.delta_rotation += -SPIN_ACCELERATION * (2.0*PI - ship_angle_difference.abs());
                velocity.angvel += -SPIN_ACCELERATION * (2.0*PI - ship_angle_difference.abs());
            }
    
        }

        player.delta_rotation = player.delta_rotation.clamp(-MAX_VELOCITY, MAX_VELOCITY);
        player_trans.rotate_z(player.delta_rotation.to_radians());
    
    }

    fn player_fire_weapon(
        mut commands: Commands,
        keyboard_input: Res<Input<MouseButton>>,
        player_query: Query<(&mut Player, &mut Transform, &Velocity)>
    )
    {    
        // should be just pressed, but it's fun with keyboard_input.pressed()d
        if keyboard_input.just_pressed(MouseButton::Left) {
            let (player, transform, velocity) = player_query.single();
    
            // why does this work? https://www.reddit.com/r/rust_gamedev/comments/rphgsf/calculating_bullet_x_and_y_position_based_off_of/
            let player_velocity = (transform.rotation * Vec3::Y) + Vec3::new(player.delta_x, player.delta_y, 0.0) * PIXELS_PER_METER;


            ProjectilePlugin::spawn_projectile(&mut commands, transform.translation.truncate(), player_velocity.truncate());
        }
    }

    fn player_camera_control(kb: Res<Input<KeyCode>>, time: Res<Time>, mut query: Query<&mut OrthographicProjection, With<Camera2d>>) {
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

    fn gravitate_collectibles(
        mut collectible_query: Query<(Entity, &Collectible, &Transform, &mut Velocity)>,
        player_query: Query<(Entity, &Player, &Transform), With<Player>>
    ) {
        const MAX_GRAVITATION_DISTANCE: f32 = 30.0 * crate::PIXELS_PER_METER;
        let (_player_ent, _player, player_transform) = player_query.single();

        for (_ent, _collectible, collectible_tranform, mut veclocity) in collectible_query.iter_mut() {
            let distance_to_player_from_collectible = player_transform.translation.truncate().distance(collectible_tranform.translation.truncate());
            if distance_to_player_from_collectible < MAX_GRAVITATION_DISTANCE {
                let percent_distance_from_max = distance_to_player_from_collectible / MAX_GRAVITATION_DISTANCE;
                let direction_to_player_from_collectible = (player_transform.translation.truncate() - collectible_tranform.translation.truncate()).normalize();
                let gravitation_factor = 1.0 - percent_distance_from_max;
                veclocity.linvel += direction_to_player_from_collectible * gravitation_factor * 5.0 * crate::PIXELS_PER_METER;
            }

        }
    }

}
