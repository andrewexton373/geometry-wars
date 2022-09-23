use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy_prototype_lyon::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use std::f32::consts::PI;
use crate::healthbar::Health;
use crate::{ HitboxCircle };
use crate::projectile::{ProjectilePlugin};
use crate::crosshair::Crosshair;

pub struct PlayerPlugin;

#[derive(Component, Inspectable, Reflect, Default)]
#[reflect(Component)]
pub struct Player {
    // TODO: refactor into velocity Vec2
    pub delta_x: f32,
    pub delta_y: f32,
    pub delta_rotation: f32,
    pub hitbox: HitboxCircle,
    pub health: Health
}

impl Player {
    fn new() -> Player {
        Player { 
            delta_x: 0.0,
            delta_y: 0.0,
            delta_rotation: 0.0,
            hitbox: HitboxCircle { radius: 5.0 },
            health: Health { current: 50.0, maximum: 100.0 }
        }
    }

    pub fn take_damage(&mut self, damage: f32) {
        let modified_health = self.health.current - damage;
        let modified_health = modified_health.clamp(0.0, self.health.maximum);
        self.health.current = modified_health;
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
            .register_inspectable::<Player>()
            .register_type::<Player>();
    }
}

impl PlayerPlugin {
    fn spawn_player(
        mut commands: Commands
    ) {
        let player_shape = shapes::RegularPolygon {
            sides: 3,
            feature: shapes::RegularPolygonFeature::Radius(20.0),
            ..shapes::RegularPolygon::default()
        };
    
        let mut player = commands.spawn()
            .insert(Player::new())
            .insert_bundle(GeometryBuilder::build_as(
                &player_shape,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::CYAN),
                    outline_mode: StrokeMode::new(Color::WHITE, 2.0),
                },
                Transform {
                    scale: Vec3::new(0.5, 1.0, 1.0),
                    ..Default::default()
                }
            ))
            .insert(Name::new("Player"))
            .id();
    }

    fn player_movement(
        keyboard_input: Res<Input<KeyCode>>,
        mut player_query: Query<(&mut Player, &mut Transform), Without<Crosshair>>
    ) {
        const ACCELERATION: f32 = 0.2;
        const DECLERATION: f32 = 0.95;
        const SPIN_ACCELERATION: f32 = 0.4;
        const SPIN_DECELERATION: f32 = 0.1;
        const MAX_VELOCITY: f32 = 6.0;
    
        let (mut player, mut player_trans) = player_query.single_mut();
    
        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            player.delta_x -= ACCELERATION;
        }
    
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            player.delta_x += ACCELERATION;
        }
    
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            player.delta_y += ACCELERATION;
        }
    
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            player.delta_y -= ACCELERATION;
        }
    
    
        player.delta_x = player.delta_x.clamp(-MAX_VELOCITY, MAX_VELOCITY);
        player.delta_y = player.delta_y.clamp(-MAX_VELOCITY, MAX_VELOCITY);
    
        player_trans.translation.x += player.delta_x;
        player_trans.translation.y += player.delta_y;
    
        // Decelerate
        player.delta_x *= DECLERATION;
        player.delta_y *= DECLERATION;
        player.delta_rotation *= SPIN_DECELERATION;
    
    }

    fn ship_rotate_towards_mouse(
        wnds: Res<Windows>,
        // query to get camera transform
        q_camera: Query<(&Camera, &GlobalTransform)>,
        mut player_query: Query<(&mut Player, &mut Transform), Without<Crosshair>>
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
    
        let (mut player, mut player_trans) = player_query.single_mut();
    
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
                player.delta_rotation += SPIN_ACCELERATION * (2.0*PI - ship_angle_difference.abs());
            } else
    
            if ship_angle_difference < 0.0 {
                player.delta_rotation -= SPIN_ACCELERATION * (2.0*PI - ship_angle_difference.abs());
            }
    
        }

        player.delta_rotation = player.delta_rotation.clamp(-MAX_VELOCITY, MAX_VELOCITY);
        player_trans.rotate_z(player.delta_rotation.to_radians());
    
    }

    fn player_fire_weapon(
        mut commands: Commands,
        keyboard_input: Res<Input<MouseButton>>,
        player_query: Query<(&mut Player, &mut Transform)>
    )
    {
        const BULLET_SPEED: f32 = 4.0;
    
        // should be just pressed, but it's fun with keyboard_input.pressed()d
        if keyboard_input.just_pressed(MouseButton::Left) {
            let (player, transform) = player_query.single();
    
            // why does this work? https://www.reddit.com/r/rust_gamedev/comments/rphgsf/calculating_bullet_x_and_y_position_based_off_of/
            let velocity = ((transform.rotation * Vec3::Y) * BULLET_SPEED) + Vec3::new(player.delta_x, player.delta_y, 0.0);

            ProjectilePlugin::spawn_projectile(&mut commands, transform.translation.truncate(), velocity.truncate());
        }
    }
}
