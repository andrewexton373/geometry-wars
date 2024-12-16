use std::borrow::BorrowMut;

use avian2d::prelude::LinearVelocity;
use bevy::prelude::*;
use rand::Rng;

use crate::{camera::components::GameCamera, player::components::Player};

use super::components::{Layer, Sector, StarfieldBackground};

pub fn init_starfield(mut commands: Commands) {
    // Spawn 3 Layers
    for layer in 1..4 {
        commands.spawn((Layer(layer), Transform::default(), Visibility::default()));
    }
}

pub fn parallax_layers(
    mut layers: Query<(&Layer, &mut Transform)>,
    player_velocity: Query<&LinearVelocity, With<Player>>,
) {
    let velocity = player_velocity.single();

    for (layer, mut transform) in layers.iter_mut() {
        // Transform Each Layer Correlated to Player Linear Velocity
        *transform = Transform {
            translation: (transform.translation.truncate()
                + velocity.xy().as_vec2() / (600.0 * layer.0 as f32))
                .extend(0.0),
            rotation: transform.rotation,
            scale: transform.scale,
        };
    }
}

pub const SECTOR_SIZE: f32 = 1280.0;

pub fn generate_visible_sectors(
    mut commands: Commands,
    camera_viewport: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    sectors: Query<(Entity, &Sector)>,
    layers: Query<(Entity, &Layer), With<Layer>>,
) {
    let (camera, camera_gt) = camera_viewport.single();

    // Get viewport bounds in worldspace
    let bottom_left = camera
        .ndc_to_world(camera_gt, Vec3::new(-1.0, -1.0, 0.0))
        .unwrap();
    let top_right = camera
        .ndc_to_world(camera_gt, Vec3::new(1.0, 1.0, 0.0))
        .unwrap();

    // Get sector indicies min, and max for x and y values
    let i_min = ((bottom_left.x / SECTOR_SIZE) as i32) - 1;
    let i_max = ((top_right.x / SECTOR_SIZE) as i32) + 1;
    let j_min = ((bottom_left.y / SECTOR_SIZE) as i32) - 1;
    let j_max = ((top_right.y / SECTOR_SIZE) as i32) + 1;

    for (layer_entity, layer) in layers.iter() {
        // For each sector that's visible in the viewport plusminus one additional sector
        for i in i_min..=i_max {
            for j in j_min..=j_max {
                // If a sector already exists
                let valid_sector = sectors
                    .iter()
                    .find(|(_, sector)| sector.i == i && sector.j == j);

                if valid_sector.is_some() {
                    continue;
                }

                generate_sector(
                    commands.borrow_mut(),
                    layer_entity,
                    layer,
                    Sector {
                        i,
                        j,
                        sector_size: SECTOR_SIZE,
                    },
                );
            }
        }

        // Filter Invalid sectors to despawn
        let invalid_sectors: Vec<(Entity, &Sector)> = sectors
            .iter()
            .filter(|(_, sector)| {
                sector.i < i_min || sector.i > i_max || sector.j < j_min || sector.j > j_max
            })
            .collect();

        // Despawn each invalid sector
        for (entity, _) in invalid_sectors {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn generate_sector(commands: &mut Commands, layer_entity: Entity, layer: &Layer, sector: Sector) {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    let layer_scale = match layer.0 {
        1 => 0.5,
        2 => 1.0,
        3 => 2.0,
        _ => 0.0,
    };

    let sector_id = commands
        .spawn((
            sector,
            SpatialBundle {
                transform: Transform::from_xyz(
                    sector.i as f32 * SECTOR_SIZE,
                    sector.j as f32 * SECTOR_SIZE,
                    -10.0,
                ),
                ..default()
            },
        ))
        .with_children(|parent| {
            // Generate Foreground Stars
            for _ in 0..128 {
                let r = SECTOR_SIZE / 2.0;
                let p = Vec3::new(rng.gen_range(-r..r), rng.gen_range(-r..r), 0.0);

                let s = rng.gen_range(0.2..2.8);
                let scale = Vec3::new(s, s, 1.0) * layer_scale;
                parent.spawn((
                    Sprite {
                        color: Color::WHITE,
                        custom_size: Some(Vec2::new(
                            0.2 * crate::PIXELS_PER_METER as f32,
                            0.2 * crate::PIXELS_PER_METER as f32,
                        )),
                        ..default()
                    },
                    Transform {
                        translation: p,
                        scale,
                        ..default()
                    },
                    StarfieldBackground,
                ));
            }
        })
        .id();

    commands.entity(layer_entity).add_child(sector_id);
}
