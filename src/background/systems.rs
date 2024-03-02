use bevy::prelude::*;
use rand::Rng;

use crate::{camera::components::GameCamera, player::components::Player};

use super::components::{Sector, StarfieldBackground};

pub fn init_starfield(
    mut commands: Commands
) {
    
}

pub const SECTOR_SIZE: f32 = 1280.0;

pub fn update_visible_sectors(
    mut commands: Commands,
    camera_viewport: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    sectors: Query<(Entity, &Sector)>
) {
    let (camera, camera_gt) = camera_viewport.single();

    let bottom_left = camera.ndc_to_world(camera_gt, Vec3::new(-1.0, -1.0, 0.0)).unwrap();
    let top_right = camera.ndc_to_world(camera_gt, Vec3::new(1.0, 1.0, 0.0)).unwrap();

    let i_min = ((bottom_left.x / SECTOR_SIZE) as i32) - 1;
    let i_max = ((top_right.x / SECTOR_SIZE) as i32) + 1;
    let j_min = ((bottom_left.y / SECTOR_SIZE) as i32) - 1;
    let j_max = ((top_right.y / SECTOR_SIZE) as i32) + 1;

    for i in i_min..=i_max {
        for j in j_min..=j_max {

            let mut valid = false;

            for (_, sector) in sectors.iter() {
                if i == sector.i && j == sector.j {
                    // Do nothing with valid sector
                    valid = true;
                }
            }

            if valid {
                 continue;
            }

            // Sector was determined to be invalid, so generate the sector
            let new_sector = (
                Sector{
                    i,
                    j,
                    sector_size: SECTOR_SIZE,
                 },
                 SpatialBundle {
                    transform: Transform::from_xyz(i as f32 * SECTOR_SIZE, j as f32 * SECTOR_SIZE, -10.0),
                    ..default()
                 },
            );

            let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

            commands.spawn(new_sector).with_children(|parent| {


                // Generate Stars
                for _ in 0..256 {
                    let r = SECTOR_SIZE/2.0;
                    let p = Vec3::new(rng.gen_range(-r..r), rng.gen_range(-r..r), 0.0);

                    let s = rng.gen_range(0.3..2.0);
                    let scale = Vec3::new(s, s, 1.0);
                    parent.spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::WHITE,
                                custom_size: Some(Vec2::new(0.2 * crate::PIXELS_PER_METER, 0.2 * crate::PIXELS_PER_METER)),
                                ..default()
                            },
                            transform: Transform {
                                translation: p,
                                scale: scale, 
                                ..default()
                            },
                            
                            ..default()
                        },
                        StarfieldBackground
                    ));
                }
            });

        }
    }

    // Despawn sectors out of range
    for (ent, sector) in sectors.iter() {
        if sector.i < i_min|| sector.i > i_max {
            commands.entity(ent).despawn_recursive();
        }

        if sector.j < j_min || sector.j > j_max {
            commands.entity(ent).despawn_recursive();
        }
    }


}