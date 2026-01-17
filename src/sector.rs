use avian2d::prelude::{Collider, DebugRender, RigidBody, Sensor};
use bevy::{color::palettes::css::BLUE, prelude::*};

use crate::camera::components::GameCamera;
pub struct SectorPlugin;

impl Plugin for SectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_sectors)
            .add_systems(
                Update,
                (
                    update_valid_sector_bounds,
                    generate_visible_sectors.after(update_valid_sector_bounds),
                    destroy_invalid_sectors.after(update_valid_sector_bounds),
                ),
            )
            .init_resource::<ValidSectorBounds>();
    }
}

#[derive(Component)]
pub struct Sectors;

#[derive(Component, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Sector {
    pub i: i128,
    pub j: i128,
}

#[derive(Resource)]
pub struct ValidSectorBounds {
    i_min: i128,
    i_max: i128,
    j_min: i128,
    j_max: i128,
}

impl Default for ValidSectorBounds {
    fn default() -> Self {
        Self {
            i_min: -1,
            i_max: 1,
            j_min: -1,
            j_max: 1,
        }
    }
}

pub const SECTOR_SIZE: f32 = 1280.0;

pub fn init_sectors(mut commands: Commands) {
    commands.spawn((
        Name::new("SECTORS"),
        Sectors,
        GlobalTransform::default(),
        InheritedVisibility::VISIBLE,
    ));
}

// Update the valid sector bounds based on current camera viewport
pub fn update_valid_sector_bounds(
    camera_viewport: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    mut bounds: ResMut<ValidSectorBounds>,
) {
    let (camera, camera_gt) = camera_viewport.single().expect("No Camera");

    // Get viewport bounds in worldspace
    let bottom_left = camera
        .ndc_to_world(camera_gt, Vec3::new(-1.0, -1.0, 0.0))
        .unwrap();
    let top_right = camera
        .ndc_to_world(camera_gt, Vec3::new(1.0, 1.0, 0.0))
        .unwrap();

    // Get sector indicies min, and max for x and y values
    let i_min = ((bottom_left.x / SECTOR_SIZE) as i128) - 1;
    let i_max = ((top_right.x / SECTOR_SIZE) as i128) + 1;
    let j_min = ((bottom_left.y / SECTOR_SIZE) as i128) - 1;
    let j_max = ((top_right.y / SECTOR_SIZE) as i128) + 1;

    *bounds = ValidSectorBounds {
        i_min,
        i_max,
        j_min,
        j_max,
    }
}

pub fn generate_visible_sectors(
    mut commands: Commands,
    bounds: Res<ValidSectorBounds>,
    sectors: Query<(Entity, &Sector)>,
    parent: Query<Entity, With<Sectors>>,
) {
    // For each sector that's visible in the viewport plusminus one additional sector
    for i in bounds.i_min..=bounds.i_max {
        for j in bounds.j_min..=bounds.j_max {
            // If a sector already exists
            let valid_sector = sectors
                .iter()
                .find(|(_, sector)| sector.i == i && sector.j == j);

            if valid_sector.is_some() {
                continue;
            }

            // Need to generate new sector
            let sector = Sector { i, j };
            // info!("GENERATING SECTOR: {:?}", sector);

            commands
                .entity(parent.single().expect("No Parent for Sector"))
                .with_child((
                    sector,
                    Transform::from_xyz(
                        sector.i as f32 * SECTOR_SIZE,
                        sector.j as f32 * SECTOR_SIZE,
                        -10.0,
                    ),
                    // RigidBody::Static,
                    // Sensor,
                    // Collider::rectangle(SECTOR_SIZE as f64, SECTOR_SIZE as f64),
                    // DebugRender::default().with_collider_color(BLUE.into()),
                    Visibility::Inherited,
                ));
        }
    }
}

fn destroy_invalid_sectors(
    mut commands: Commands,
    bounds: Res<ValidSectorBounds>,
    sectors: Query<(Entity, &Sector)>,
) {
    // Filter Invalid sectors to despawn
    let invalid_sectors: Vec<(Entity, &Sector)> = sectors
        .iter()
        .filter(|(_, sector)| {
            sector.i < bounds.i_min
                || sector.i > bounds.i_max
                || sector.j < bounds.j_min
                || sector.j > bounds.j_max
        })
        .collect();

    // Despawn each invalid sector
    for (entity, sector) in invalid_sectors {
        // info!("DESTROYING SECTOR: {:?}", sector);
        commands.entity(entity).despawn();
    }
}
