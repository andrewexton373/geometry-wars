use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{self as lyon, DrawMode};
use bevy_rapier2d::prelude::{Velocity, Collider, Sleeping};

use crate::{astroid::{Astroid, self}, PIXELS_PER_METER};


pub struct BaseStationPlugin;

#[derive(Component)]
pub struct BaseStation;

impl Plugin for BaseStationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(Self::spawn_base_station)
            .add_system(Self::guide_player_to_base_station)
            .add_system(Self::repel_astroids_from_base_station);
    }
}

impl BaseStationPlugin {
    fn spawn_base_station(
        mut commands: Commands
    ) {
        let base_shape = lyon::shapes::RegularPolygon {
            sides: 6,
            feature: lyon::shapes::RegularPolygonFeature::Radius(crate::PIXELS_PER_METER * 20.0),
            ..lyon::shapes::RegularPolygon::default()
        };
    
        let _base_station = commands.spawn()
            .insert_bundle(lyon::GeometryBuilder::build_as(
                &base_shape,
                lyon::DrawMode::Outlined {
                    fill_mode: lyon::FillMode::color(Color::MIDNIGHT_BLUE),
                    outline_mode: lyon::StrokeMode::new(Color::WHITE, 5.0),
                },
                Transform { translation: Vec3::new(0.0, 0.0, -1.0), ..Default::default() }
            ))
            .insert(Sleeping::disabled())
            // .insert(Ccd::enabled())
            // .insert(Collider::triangle(player_shape.feature, b, c)) // Need points of triangle
            // .insert(Collider::ball(crate::PIXELS_PER_METER * 1.0))
            .insert(Transform::default())
            // .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(BaseStation)
            .insert(Name::new("Base Station"))
            .id();
    }

    fn guide_player_to_base_station(

    ) {

    }

    fn repel_astroids_from_base_station(
        base_query: Query<(&BaseStation, &GlobalTransform), With<BaseStation>>,
        mut astroid_query: Query<(&Astroid, &GlobalTransform, &mut Velocity), With<Astroid>>
    ) {
        const REPEL_RADIUS: f32 = 120.0 * PIXELS_PER_METER;
        const REPEL_STRENGTH: f32 = 25.0;

        let (base_station, base_station_transform) = base_query.single();

        for (astroid, astroid_transform, mut astroid_velocity) in astroid_query.iter_mut() {
            let base_station_pos = base_station_transform.translation().truncate();
            let astroid_pos = astroid_transform.translation().truncate();

            let distance = base_station_pos.distance(astroid_pos);
            let distance_weight = 1.0 - (distance / REPEL_RADIUS);

            if distance < REPEL_RADIUS {
                let repel_vector = (astroid_pos - base_station_pos).normalize();
                astroid_velocity.linvel += repel_vector * distance_weight * REPEL_STRENGTH;
            }
        }
    }
}