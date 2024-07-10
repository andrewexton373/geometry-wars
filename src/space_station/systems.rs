use std::fmt::DebugTuple;

use bevy::prelude::*;
use bevy_prototype_lyon::{prelude::{
    self as lyon, Fill, FillOptions, GeometryBuilder, ShapeBundle, Stroke,
}, shapes};
use bevy_xpbd_2d::prelude::*;
use hexx::{hex, Hex};
use ordered_float::OrderedFloat;

use crate::{
    asteroid::components::Asteroid,
    battery::events::ChargeBatteryEvent,
    factory::FactoryPlugin,
    health::{components::Health, events::RepairEvent},
    hexgrid::{
        components::{BuildingType, HexTile},
        resources::HexGridMap,
    },
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
    modules::{components::{SpaceStationModule, SpaceStationModuleType}, turret::components::Turret},
    resources::{
        CanDeposit, PlayerHoveringSpaceStationModule, SpaceStationModuleMaterialMap,
        SPACE_STATION_SIZE,
    },
};

pub fn init_space_station_module_material_map(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(SpaceStationModuleMaterialMap {
        core_material: materials.add(Color::DARK_GRAY),
        fabrication_material: materials.add(Color::ORANGE_RED),
        storage_material: materials.add(Color::TEAL),
        turret_material: materials.add(Color::PINK),
        buildable_material: materials.add(Color::rgba(0.0, 0.0, 0.0, 0.0)),
    });
}

pub fn init_space_station_core(mut commands: Commands, hex_grid_map: Res<HexGridMap>) {
    if let Some(origin_hex_ent) = hex_grid_map.entities.get(&Hex::ORIGIN).copied() {
        commands.entity(origin_hex_ent).insert((
            SpaceStationModuleType::Core,
            Health::with_maximum(1000.0),
            SpaceStation,
            Name::new("Base Station"),
        ));

        attach_inventory_to_entity(
            &mut commands,
            Inventory {
                items: vec![],
                capacity: Capacity {
                    maximum: 2000.0.into(),
                },
            },
            origin_hex_ent,
        )
    }
}

pub fn init_space_station_turret(
    mut commands: Commands,
    hex_grid_map: Res<HexGridMap>,
) {
    if let Some(origin_hex_ent) = hex_grid_map.entities.get(&Hex::new(0, 1)).copied() {
        commands.entity(origin_hex_ent).insert((
            SpaceStationModuleType::Turret,
            Health::with_maximum(1000.0),
            Name::new("Space Station Turret"),
        )).with_children(|parent| {

            let barrel = shapes::Line(
                Vec2 {
                    x: 20.0,
                    y: 0.0
                },
                Vec2 {
                    x: 40.0,
                    y: 0.0,
                },
            );

            let body = shapes::RegularPolygon {
                sides: 8,
                center: Vec2::ZERO,
                feature: lyon::RegularPolygonFeature::Radius(20.0),
                ..default()
            };
        
            let geometry = GeometryBuilder::new().add(&barrel).add(&body);
            

            parent.spawn((
                Turret,
                Name::new("Turret"),
                ShapeBundle {
                    path: geometry.build(),
                    spatial: SpatialBundle {
                        transform: Transform::from_xyz(0.0, 0.0, 1.0),
                        
                        ..default()
                    },
                    ..default()
                },
                Fill::color(Color::WHITE),
                Stroke::new(Color::BLACK, 8.0)
            ));


        });
    }
}


pub fn color_space_station_modules(
    mut commands: Commands,
    module_query: Query<(Entity, Option<&SpaceStationModuleType>), With<HexTile>>,
    module_material_map: Res<SpaceStationModuleMaterialMap>,
) {
    for (ent, module_type) in module_query.iter() {
        if let Some(module_type) = module_type {
            let material = match *module_type {
                SpaceStationModuleType::Core => &module_material_map.core_material,
                SpaceStationModuleType::Factory => &module_material_map.fabrication_material,
                SpaceStationModuleType::Refinery => &module_material_map.fabrication_material,
                SpaceStationModuleType::Storage => &module_material_map.storage_material,
                SpaceStationModuleType::Turret => &module_material_map.turret_material,
            };

            // Color HexTiles based on Module Type.
            commands.entity(ent).insert(material.clone());
        } else {
            // Color Hex as Transparent Buildable HexTile
            commands
                .entity(ent)
                .insert(module_material_map.buildable_material.clone());
        }
    }
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
    mut charge_events: EventWriter<ChargeBatteryEvent>,
    time: Res<Time>,
) {
    let (player_ent, mut player) = player_query.single_mut();
    let (base_station_ent, _base_station) = base_station_query.single();

    if let Some(_collision) = collisions.get(player_ent, base_station_ent) {
        *can_deposit_res = CanDeposit(true);
        context_clues_res.0.insert(ContextClue::NearBaseStation);

        charge_events.send(ChargeBatteryEvent {
            entity: player_ent,
            charge: 100.0 * time.delta_seconds(),
        });

        repair_events.send(RepairEvent {
            entity: player_ent,
            repair: 10.0 * time.delta_seconds(),
        });
    } else {
        *can_deposit_res = CanDeposit(false);
        context_clues_res.0.remove(&ContextClue::NearBaseStation);
    }
}

pub fn update_space_station_module_context(
    mut space_station_module_context: ResMut<PlayerHoveringSpaceStationModule>,
    player_ship_q: Query<Entity, With<Player>>,
    space_station_module_q: Query<(Entity, &BuildingType), With<SpaceStationModule>>,
    collisions: Res<Collisions>,
) {
    let player_ent = player_ship_q.single();

    for (module_ent, module_type) in space_station_module_q.iter() {
        if let Some(_) = collisions.get(player_ent, module_ent) {
            space_station_module_context.0 = Some((module_ent, *module_type));
            dbg!("Module Context {}", space_station_module_context.0);
        }
    }
}
