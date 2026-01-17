use std::borrow::BorrowMut;

use avian2d::prelude::LinearVelocity;
use bevy::prelude::*;
use rand::Rng;

use crate::{camera::components::GameCamera, sector::Sector};

use super::components::{Layer, StarfieldBackground};

#[derive(Component)]
pub struct BackgroundSector(pub Sector);

pub fn init_starfield(mut commands: Commands) {
    // Spawn 3 Layers
    for layer in 1..4 {
        commands.spawn((Layer(layer), Transform::default(), Visibility::Inherited));
    }
}

pub fn parallax_layers(
    mut layers: Query<(&Layer, &mut Transform)>,
    camera_velocity: Query<&LinearVelocity, With<GameCamera>>,
) {
    if let Ok(velocity) = camera_velocity.single() {
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
}

pub fn generate_background_on_sector_add(
    trigger: Trigger<OnAdd, Sector>,
    sectors: Query<&Sector>,
    mut commands: Commands,
    layers: Query<(Entity, &Layer), With<Layer>>,
) {
    let sector = sectors.get(trigger.target()).unwrap();

    for (layer_ent, layer) in layers.iter() {
        generate_sector(commands.borrow_mut(), layer_ent, layer, sector);
    }
}

pub fn destroy_background_on_sector_remove(
    trigger: Trigger<OnRemove, Sector>,
    mut commands: Commands,
    sectors: Query<&Sector>,
    background_sectors: Query<(Entity, &BackgroundSector)>,
) {
    let sector = sectors.get(trigger.target()).unwrap();

    for (entity, _bg_sector_to_remove) in background_sectors
        .into_iter()
        .filter(|(_, bs)| bs.0 == *sector)
        .collect::<Vec<(Entity, &BackgroundSector)>>()
    {
        commands.entity(entity).despawn_recursive();
    }
}

pub const SECTOR_SIZE: f32 = 1280.0;

fn generate_sector(commands: &mut Commands, layer_entity: Entity, layer: &Layer, sector: &Sector) {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    let layer_scale = match layer.0 {
        1 => 0.5,
        2 => 1.0,
        3 => 2.0,
        _ => 0.0,
    };

    let sector_id = commands
        .spawn((
            BackgroundSector(*sector),
            Transform::from_xyz(
                sector.i as f32 * SECTOR_SIZE,
                sector.j as f32 * SECTOR_SIZE,
                -10.0,
            ),
            Visibility::Inherited,
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
                    Visibility::Inherited,
                    StarfieldBackground,
                ));
            }
        })
        .id();

    commands.entity(layer_entity).add_child(sector_id);
}
