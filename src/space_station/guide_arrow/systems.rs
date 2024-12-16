use bevy::{color::palettes::css::RED, prelude::*};

use crate::{player::components::Player, space_station::components::SpaceStation};

use super::components::SpaceStationDirectionIndicator;


pub fn spawn_player_base_guide_arrow(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // let direction_indicator_shape = RegularPolygon {
    //     sides: 3,
    //     feature: RegularPolygonFeature::Radius(crate::PIXELS_PER_METER as f32 * 2.0),
    //     ..RegularPolygon::default()
    // };

    let direction_indicator_shape = RegularPolygon::new(crate::PIXELS_PER_METER as f32 * 2.0, 3);

    let _direction_indicator = commands
        .spawn((
            SpaceStationDirectionIndicator,
            Mesh2d(meshes.add(direction_indicator_shape)),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::from(RED)))),
            // generate_guide_arrow(0.0),
            Transform::from_xyz(0.0, 0.0, 1.0),
            Name::new("SpaceStationDirectionIndicator"),
        ))
        .id();
}

pub fn guide_player_to_space_station(
    mut dir_indicator_query: Query<
        (&mut Transform),
        (
            With<SpaceStationDirectionIndicator>,
            Without<SpaceStation>,
            Without<Player>,
        ),
    >,
    player_query: Query<(&Player, &GlobalTransform), (With<Player>, Without<SpaceStation>)>,
    base_query: Query<(&SpaceStation, &GlobalTransform), (With<SpaceStation>, Without<Player>)>,
) {
    const FADE_DISTANCE: f32 = 500.0;

    let (mut dir_indicator_transform) = dir_indicator_query.single_mut();
    let (_player, player_trans) = player_query.single();
    let (_base_station, base_station_trans) = base_query.single();

    let player_pos = player_trans.translation().truncate();
    let base_station_pos = base_station_trans.translation().truncate();

    let distance_to_base = (base_station_pos - player_pos).length();
    let direction_to_base = (base_station_pos - player_pos).normalize();
    let rotation = Vec2::Y.angle_between(direction_to_base);

    dir_indicator_transform.rotation = Quat::from_rotation_z(rotation);
    dir_indicator_transform.translation =
        (player_trans.translation().truncate() + direction_to_base * 100.0).extend(0.0);

    dir_indicator_transform.scale = Vec3::new(0.3, 1.0, 1.0);

    let opacity = (distance_to_base / FADE_DISTANCE).powi(2).clamp(0.0, 1.0);

    // *shape = generate_guide_arrow(opacity);
}
