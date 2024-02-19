use crate::events::{AblateEvent, LaserEvent};
use crate::particles::LaserImpactParticleSystem;
use crate::player::Player;
use bevy::prelude::*;
use bevy_particle_systems::Playing;
use bevy_prototype_lyon::{
    prelude::{GeometryBuilder, Path, ShapeBundle, ShapePath, Stroke},
    shapes,
};
// use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Laser;

pub struct LaserPlugin;

impl Plugin for LaserPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LaserEvent>()
            .add_systems(Startup, Self::setup_laser)
            .add_systems(Update, Self::fire_laser_raycasting);
    }
}

impl LaserPlugin {
    pub fn setup_laser(mut commands: Commands, mut laser_query: Query<&mut Laser>) {
        let line = shapes::Line(Vec2::ZERO, Vec2::X);

        // Create Laser if it Doesn't Exist
        let Ok(_laser) = laser_query.get_single_mut() else {

            commands.spawn(Laser)
                .insert((
                    ShapeBundle {
                        path: GeometryBuilder::build_as(&line),
                        ..default()
                    },
                    Stroke::new(Color::rgba(1.0, 0.0, 0.0, 0.9), 5.0),
                    Name::new("Laser"),
                ));    
            return;
        };
    }

    pub fn fire_laser_raycasting(
        mut commands: Commands,
        mut laser_event_reader: EventReader<LaserEvent>,
        mut ablate_event_writer: EventWriter<AblateEvent>,
        // rapier_context: Res<RapierContext>,
        mut laser_query: Query<
            &mut Path,
            With<Laser>,
        >,
        mut laser_impact_particles_query: Query<
            (Entity, &LaserImpactParticleSystem, &mut Transform),
            Without<Laser>,
        >,
    ) {
        // TODO: Change Laser State To Off On Player Left Unclick
        for (ent, _, mut _t) in laser_impact_particles_query.iter_mut() {
            commands.entity(ent).remove::<Playing>();
        }

        let mut laser_path = laser_query.single_mut();

        // dbg!("{:?}", &laser_path.0);

        // Reset laser
        let line = shapes::Line(Vec2::ZERO, Vec2::ZERO);
        *laser_path = ShapePath::build_as(&line);

        // dbg!("SET 0: {:?}", &laser_path.0);


        for fire_laser_event in laser_event_reader.read() {
            let laser_active = fire_laser_event.0;
            let ray_pos = fire_laser_event.1;
            let ray_dir = fire_laser_event.2;

            // If laser is active
            if laser_active {
                let line = shapes::Line(ray_pos, ray_pos + ray_dir * 10000.0);
                *laser_path = ShapePath::build_as(&line);

                let max_toi = 10000.0;
                let solid = false; // i think?
                // let filter = QueryFilter::default();

                // if let Some((entity, intersection)) =
                //     rapier_context.cast_ray_and_get_normal(ray_pos, ray_dir, max_toi, solid, filter)
                // {
                //     let hit_point = intersection.point;
                //     let hit_normal = intersection.normal;

                //     for (ent, _, mut t) in laser_impact_particles_query.iter_mut() {
                //         commands.entity(ent).insert(Playing);
                //         t.translation = hit_point.extend(0.0);
                //     }

                //     let line = shapes::Line(ray_pos, hit_point);
                    
                //     *laser_path = ShapePath::build_as(&line);

                //     // dbg!("LASER PATH: {:?}", &laser_path.0);


                //     ablate_event_writer.send(AblateEvent(entity, hit_point, hit_normal));
                // }
            }
        }
    }
}
