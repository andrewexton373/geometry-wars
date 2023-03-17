use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{self as lyon, Fill, Stroke, GeometryBuilder, ShapeBundle, FillOptions};
use bevy_rapier2d::prelude::{ActiveEvents, Collider, RapierContext, Sensor, Sleeping, Velocity};
use ordered_float::OrderedFloat;

use crate::{
    astroid::Astroid,
    factory::{FactoryPlugin},
    game_ui::{ContextClue, ContextClues},
    inventory::{Capacity, Inventory, InventoryPlugin},
    player::Player,
    refinery::{RefineryPlugin},
    PIXELS_PER_METER,
};

pub const BASE_STATION_SIZE: f32 = 20.0;

#[derive(Component)]
pub struct BaseStationDirectionIndicator;

pub struct BaseStationPlugin;

#[derive(Component)]
pub struct BaseStation;

#[derive(Resource)]
pub struct CanDeposit(pub bool);

impl Plugin for BaseStationPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::spawn_base_station)
            .add_startup_system(Self::spawn_player_base_guide_arrow)
            .add_system(Self::guide_player_to_base_station)
            .add_system(Self::repel_astroids_from_base_station)
            .add_system(Self::handle_base_station_sensor_collision_event)
            .insert_resource(CanDeposit(true));
    }
}

impl BaseStationPlugin {
    fn spawn_base_station(mut commands: Commands) {
        let base_shape = lyon::shapes::RegularPolygon {
            sides: 6,
            feature: lyon::shapes::RegularPolygonFeature::Radius(
                crate::PIXELS_PER_METER * BASE_STATION_SIZE,
            ),
            ..lyon::shapes::RegularPolygon::default()
        };

        let base_station = commands
            .spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&base_shape),
                    transform: Transform {
                        translation: Vec3::new(0.0, 0.0, -100.0),
                        ..Default::default()
                    },
                    ..default()
                },
                //TODO: Fill and Stroke aren't applying for some reason
                Fill::color(Color::BLUE),
                Stroke::new(Color::WHITE, 5.0),
                Sleeping::disabled(),
                Collider::ball(crate::PIXELS_PER_METER * BASE_STATION_SIZE),
                Sensor,
                ActiveEvents::COLLISION_EVENTS,
                BaseStation,
                Name::new("Base Station"),

            )).id();

        InventoryPlugin::attach_inventory_to_entity(
            &mut commands,
            Inventory {
                items: Vec::new(),
                capacity: Capacity { maximum: OrderedFloat(1000.0) },
            },
            base_station,
        );
        RefineryPlugin::attach_refinery_to_entity(&mut commands, base_station);
        FactoryPlugin::attach_factory_to_entity(&mut commands, base_station);
    }

    fn spawn_player_base_guide_arrow(mut commands: Commands) {
        let direction_indicator_shape = lyon::shapes::RegularPolygon {
            sides: 3,
            feature: lyon::shapes::RegularPolygonFeature::Radius(crate::PIXELS_PER_METER * 2.0),
            ..lyon::shapes::RegularPolygon::default()
        };

        let _direction_indicator = commands
            .spawn((
                BaseStationDirectionIndicator,
                ShapeBundle {
                    path: GeometryBuilder::build_as(&direction_indicator_shape),
                    ..default()
                },
                Fill::color(Color::RED),
                Name::new("BaseStationDirectionIndicator"),
            )).id();

    }

    fn guide_player_to_base_station(
        mut dir_indicator_query: Query<
            (&mut Transform, &mut Fill),
            (
                With<BaseStationDirectionIndicator>,
                Without<BaseStation>,
                Without<Player>,
            ),
        >,
        player_query: Query<(&Player, &GlobalTransform), (With<Player>, Without<BaseStation>)>,
        base_query: Query<(&BaseStation, &GlobalTransform), (With<BaseStation>, Without<Player>)>,
    ) {
        const FADE_DISTANCE: f32 = 500.0;

        let (mut dir_indicator_transform, mut dir_indicator_fill) =
            dir_indicator_query.single_mut();
        let (_player, player_trans) = player_query.single();
        let (_base_station, base_station_trans) = base_query.single();

        let player_pos = player_trans.translation().truncate();
        let base_station_pos = base_station_trans.translation().truncate();

        let distance_to_base = (base_station_pos - player_pos).length();
        let direction_to_base = (base_station_pos - player_pos).normalize();
        let rotation = Vec2::Y.angle_between(direction_to_base);

        dir_indicator_transform.rotation = Quat::from_rotation_z(rotation);
        dir_indicator_transform.translation =
            (player_trans.translation().truncate() + direction_to_base * 100.0).extend(999.0);
        dir_indicator_transform.scale = Vec3::new(0.3, 1.0, 1.0);

        let opacity = (distance_to_base / FADE_DISTANCE).powi(2).clamp(0.0, 1.0);

        *dir_indicator_fill = Fill {
            color: Color::Rgba { 
                red: 255.0,
                green: 0.0,
                blue: 0.0,
                alpha: opacity,
            },
            options: FillOptions::default(),
        }

    }

    fn repel_astroids_from_base_station(
        base_query: Query<(&BaseStation, &GlobalTransform), With<BaseStation>>,
        mut astroid_query: Query<(&Astroid, &GlobalTransform, &mut Velocity), With<Astroid>>,
    ) {
        const REPEL_RADIUS: f32 = 120.0 * PIXELS_PER_METER;
        const REPEL_STRENGTH: f32 = 25.0;

        let (_base_station, base_station_transform) = base_query.single();

        for (_astroid, astroid_transform, mut astroid_velocity) in astroid_query.iter_mut() {
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

    fn handle_base_station_sensor_collision_event(
        rapier_context: Res<RapierContext>,
        mut can_deposit_res: ResMut<CanDeposit>,
        mut context_clues_res: ResMut<ContextClues>,
        mut player_query: Query<(Entity, &mut Player), With<Player>>,
        base_station_query: Query<(Entity, &BaseStation), With<BaseStation>>,
        time: Res<Time>,
    ) {
        let (player_ent, mut player) = player_query.single_mut();
        let (base_station_ent, _base_station) = base_station_query.single();

        if rapier_context.intersection_pair(player_ent, base_station_ent) == Some(true) {
            *can_deposit_res = CanDeposit(true);
            context_clues_res.0.insert(ContextClue::NearBaseStation);

            player.charge_battery(100.0 * time.delta_seconds());
            player.repair_damage(10.0 * time.delta_seconds());
        } else {
            *can_deposit_res = CanDeposit(false);
            context_clues_res.0.remove(&ContextClue::NearBaseStation);
        }
    }
}
