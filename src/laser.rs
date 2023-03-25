use crate::{
    astroid::Astroid,
    // particles::ProjectileImpactParticles,
    player::Player
};
use bevy::prelude::*;
// use bevy_hanabi::ParticleEffect;
use bevy_prototype_lyon::{prelude::{GeometryBuilder, Path, ShapeBundle, ShapePath, Stroke}, shapes};
use bevy_rapier2d::prelude::*;
use crate::events::{AblateEvent, LaserEvent};

#[derive(Component)]
pub struct Laser {}


pub struct LaserPlugin;

impl Plugin for LaserPlugin {
    fn build(&self, app: &mut App) {
        app

            .add_event::<LaserEvent>()
            .add_startup_system(Self::setup_laser)
            .add_system(Self::fire_laser_raycasting);
    }
}

impl LaserPlugin {

    pub fn setup_laser(
        mut commands: Commands,
        mut laser_query: Query<&mut Laser>,
    ) {
        let line = shapes::Line(Vec2::ZERO, Vec2::X);

        // Create Laser if it Doesn't Exist
        let Ok(_laser) = laser_query.get_single_mut() else {
            commands
                .spawn((
                    Laser {},
                    ShapeBundle {
                        path: GeometryBuilder::build_as(&line),
                        transform: Transform {
                            scale: Vec3::new(1.0, 1.0, 1.0),
                            ..Default::default()
                        },
                        ..default()
                    },
                    Stroke::new(Color::rgba(1.0, 0.0, 0.0, 0.9), 5.0),
                    Name::new("Laser")
                ));
                return;
        };
    }

    pub fn fire_laser_raycasting(
        mut laser_event_reader: EventReader<LaserEvent>,
        mut ablate_event_writer: EventWriter<AblateEvent>,
        rapier_context: Res<RapierContext>,
        mut laser_query: Query<(&mut Laser, &mut Stroke, &mut Transform, &mut Path), Without<Player>>,
        // mut effect: Query<
    //     (&mut ParticleEffect, &mut Transform),
    //     (
    //         With<ProjectileImpactParticles>,
    //         Without<Astroid>,
    //         Without<Player>,
    //         Without<Laser>
    //     ),
    // >,
    ) {

        let (_, _, _, mut laser_path) = laser_query.single_mut();

        // Reset laser
        let line = shapes::Line(Vec2::ZERO, Vec2::ZERO);
        *laser_path = ShapePath::build_as(&line);

        for fire_laser_event in laser_event_reader.iter() {

            let laser_active = fire_laser_event.0;
            let ray_pos = fire_laser_event.1;
            let ray_dir = fire_laser_event.2;

            // If laser is active
            if laser_active {

                let line = shapes::Line(ray_pos, ray_pos + ray_dir * 10000.0);
                *laser_path = ShapePath::build_as(&line);

                let max_toi = 10000.0;
                let solid = false; // i think?
                let filter = QueryFilter::default();

                if let Some((entity, intersection)) = rapier_context.cast_ray_and_get_normal (
                    ray_pos, ray_dir, max_toi, solid, filter
                ) {
                    let hit_point = intersection.point;
                    let hit_normal = intersection.normal;
                    // let (mut effect, mut effect_translation) = effect.single_mut();
                    //
                    // effect_translation.translation =
                    //     (hit_point).extend(200.0);
                    // effect.maybe_spawner().unwrap().reset();

                    let line = shapes::Line(ray_pos, hit_point);
                    *laser_path = ShapePath::build_as(&line);

                    ablate_event_writer.send(AblateEvent(entity, hit_point, hit_normal));
                }

            }

        }

    }

}
