use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{
    self as lyon, Fill, FillOptions, GeometryBuilder, ShapeBundle, Stroke,
};
use bevy_xpbd_2d::prelude::*;
use ordered_float::OrderedFloat;

use crate::{
    asteroid::components::Asteroid,
    factory::FactoryPlugin,
    health::events::RepairEvent,
    inventory::{
        components::{Capacity, Inventory},
        plugin::InventoryPlugin,
        systems::attach_inventory_to_entity,
    },
    player::components::Player,
    refinery::RefineryPlugin,
    ui::context_clue::resources::{ContextClue, ContextClues},
    PIXELS_PER_METER,
};

use super::{
    components::SpaceStation,
    resources::{CanDeposit, SPACE_STATION_SIZE},
};

pub fn spawn_space_station(mut commands: Commands) {
    let base_shape = lyon::shapes::RegularPolygon {
        sides: 6,
        feature: lyon::shapes::RegularPolygonFeature::Radius(
            crate::PIXELS_PER_METER * SPACE_STATION_SIZE,
        ),
        ..lyon::shapes::RegularPolygon::default()
    };

    let base_station = commands
        .spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&base_shape),
                spatial: Transform::from_xyz(0.0, 0.0, -100.0).into(),

                ..default()
            },
            Fill::color(Color::BLUE),
            Stroke::new(Color::WHITE, 5.0),
            Collider::ball(crate::PIXELS_PER_METER * SPACE_STATION_SIZE),
            SpaceStation,
            Name::new("Base Station"),
        ))
        .id();

    attach_inventory_to_entity(
        &mut commands,
        Inventory {
            items: Vec::new(),
            capacity: Capacity {
                maximum: OrderedFloat(1000.0),
            },
        },
        base_station,
    );
    RefineryPlugin::attach_refinery_to_entity(&mut commands, base_station);
    FactoryPlugin::attach_factory_to_entity(&mut commands, base_station);
}

pub fn repel_asteroids_from_space_station(
    base_query: Query<(&SpaceStation, &GlobalTransform), With<SpaceStation>>,
    mut asteroid_query: Query<(&Asteroid, &GlobalTransform, &mut LinearVelocity), With<Asteroid>>,
) {
    const REPEL_RADIUS: f32 = 120.0 * PIXELS_PER_METER;
    const REPEL_STRENGTH: f32 = 25.0;

    let (_base_station, base_station_transform) = base_query.single();

    for (_asteroid, asteroid_transform, mut asteroid_velocity) in asteroid_query.iter_mut() {
        let base_station_pos = base_station_transform.translation().truncate();
        let asteroid_pos = asteroid_transform.translation().truncate();

        let distance = base_station_pos.distance(asteroid_pos);
        let distance_weight = 1.0 - (distance / REPEL_RADIUS);

        if distance < REPEL_RADIUS {
            let repel_vector = (asteroid_pos - base_station_pos).normalize();
            asteroid_velocity.0 += repel_vector * distance_weight * REPEL_STRENGTH;
        }
    }
}

pub fn handle_space_station_collision_event(
    collisions: Res<Collisions>,
    mut player_query: Query<(Entity, &mut Player), With<Player>>,
    base_station_query: Query<(Entity, &SpaceStation), With<SpaceStation>>,
    mut can_deposit_res: ResMut<CanDeposit>,
    mut context_clues_res: ResMut<ContextClues>,
    mut repair_events: EventWriter<RepairEvent>,
    time: Res<Time>,
) {
    let (player_ent, mut player) = player_query.single_mut();
    let (base_station_ent, _base_station) = base_station_query.single();

    if let Some(_collision) = collisions.get(player_ent, base_station_ent) {
        *can_deposit_res = CanDeposit(true);
        context_clues_res.0.insert(ContextClue::NearBaseStation);

        player.charge_battery(100.0 * time.delta_seconds());

        repair_events.send(RepairEvent {
            entity: player_ent,
            repair: 10.0 * time.delta_seconds(),
        });
    } else {
        *can_deposit_res = CanDeposit(false);
        context_clues_res.0.remove(&ContextClue::NearBaseStation);
    }
}
