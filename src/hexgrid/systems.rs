use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::utils::hashbrown::HashMap;
use hexx::{shapes, Hex, HexLayout, PlaneMeshBuilder};

use crate::hexgrid::components::Building;
use crate::player::components::Player;
use crate::player_input::resources::MouseWorldPosition;

use super::components::{BuildingType, HexTile};
use super::events::BuildHexBuildingEvent;
use super::plugin::HEX_SIZE;
use super::resources::{
    HexGridMap, HighlightedHexes, MouseHoverHex, PlayerHoveringBuilding, SelectedHex,
};

pub fn setup_hex_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut hex_grid_map: ResMut<HexGridMap>,
) {
    let layout = HexLayout {
        hex_size: HEX_SIZE,
        origin: hexx::Vec2::ZERO,
        ..default()
    };

    // layout.h
    // materials
    // let mouse_hover_material = materials.add(Color::DARK_GRAY.into());
    // let selected_material = materials.add(Color::RED.into());
    // let ship_hover_material = materials.add(Color::LIME_GREEN.into());
    // let ring_material = materials.add(Color::YELLOW.into());
    let default_material = materials.add(Color::from(Srgba {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
        alpha: 0.0,
    }));
    // let factory_material = materials.add(Color::BISQUE.into());
    // let refinery_material = materials.add(Color::ORANGE_RED.into());
    // let storage_material = materials.add(Color::GOLD.into());

    // mesh
    let mesh = hexagonal_plane(&layout);
    let mesh_handle = meshes.add(mesh);

    let points: Vec<Vec2> = HexLayout::hex_corners(&layout, Hex::ZERO).into();
    let collider =
        Collider::convex_hull(points.iter().map(|point| point.as_dvec2()).collect()).unwrap();

    let entities: HashMap<Hex, Entity> = shapes::hexagon(Hex::default(), 1)
        .map(|hex| {
            let pos = layout.hex_to_world_pos(hex);

            let id = commands
                .spawn((
                    Mesh2d(mesh_handle.clone()),
                    MeshMaterial2d(default_material.clone()),
                    Transform::from_xyz(pos.x, pos.y, 0.0),
                ))
                .with_children(|b| {
                    b.spawn((
                        Text2d(format!("{},{}", hex.x, hex.y)),
                        TextFont {
                            font_size: 32.0,
                            ..default()
                        },
                        TextColor(Color::BLACK),
                        Transform::from_xyz(0.0, 0.0, 10.0),
                    ));
                })
                .insert(collider.clone())
                .insert(Name::new("HexTile"))
                .insert(HexTile)
                // .insert(Building(BuildingType::None))
                .id();
            (hex, id)
        })
        .collect();

    *hex_grid_map = HexGridMap { layout, entities };
}

fn hexagonal_plane(hex_layout: &HexLayout) -> Mesh {
    let mesh_info = PlaneMeshBuilder::new(hex_layout)
        .facing(Vec3::Z)
        .center_aligned()
        .build();

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
    .with_inserted_indices(Indices::U16(mesh_info.indices))
}

/// ButtonInput interaction
pub fn handle_mouse_interaction(
    mut _commands: Commands,
    mouse_position: Res<MouseWorldPosition>,
    map: Res<HexGridMap>,
    mut highlighted_hexes: ResMut<HighlightedHexes>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut hex_query: Query<(Entity, &HexTile, &mut Building)>,
) {
    let pos = mouse_position.0;

    let hex = map
        .layout
        .world_pos_to_hex(hexx::Vec2 { x: pos.x, y: pos.y });
    if let Some(entity) = map.entities.get(&hex).copied() {
        if mouse_input.just_released(MouseButton::Left) {
            if let Ok((_, _, mut building)) = hex_query.get_mut(entity) {
                building.0 = BuildingType::Factory;
            }
        }

        // Draw a  line
        highlighted_hexes.line = Hex::ZERO.line_to(hex).collect();
        // Draw a ring
        // highlighted_hexes.ring = Hex::ZERO.ring(hex.ulength());

        highlighted_hexes.selected = hex;
    }
}

pub fn update_mouse_hover_hex(
    mut _commands: Commands,
    mouse_position: Res<MouseWorldPosition>,
    map: Res<HexGridMap>,
    mut mouse_hover_hex: ResMut<MouseHoverHex>,
) {
    let pos = mouse_position.0;

    let hex = map
        .layout
        .world_pos_to_hex(hexx::Vec2 { x: pos.x, y: pos.y });
    if let Some(entity) = map.entities.get(&hex).copied() {
        *mouse_hover_hex = MouseHoverHex {
            entity: Some(entity),
            hover_hex: Some(hex),
        }
    }
}

pub fn update_selected_hex(
    mouse_events: Res<ButtonInput<MouseButton>>,
    mouse_hover_hex: Res<MouseHoverHex>,
    mut selected_hex: ResMut<SelectedHex>,
) {
    if mouse_events.just_pressed(MouseButton::Left) {
        *selected_hex = SelectedHex {
            entity: mouse_hover_hex.entity,
            selected_hex: mouse_hover_hex.hover_hex,
        };
    }
}

pub fn handle_ship_hovering_context(
    mut _commands: Commands,
    map: Res<HexGridMap>,
    mut highlighted: ResMut<HighlightedHexes>,
    mut player_hovering_building: ResMut<PlayerHoveringBuilding>,
    hex_query: Query<(Entity, &HexTile, &mut Building)>,
    player_query: Query<(Entity, &Player, &GlobalTransform)>,
) {
    *player_hovering_building = PlayerHoveringBuilding(None);
    let (_, _, player_gt) = player_query.single();

    let player_pos = player_gt.translation().truncate();

    let hex = map.layout.world_pos_to_hex(hexx::Vec2 {
        x: player_pos.x,
        y: player_pos.y,
    });
    if let Some(entity) = map.entities.get(&hex).copied() {
        highlighted.ship_hover = hex;
        if let Ok((_, _, building)) = hex_query.get(entity) {
            *player_hovering_building = PlayerHoveringBuilding(Some((entity, building.0)));
        }
    }
}

pub fn handle_build_events(
    mut commands: Commands,
    mut build_events: EventReader<BuildHexBuildingEvent>,
) {
    for evt in build_events.read() {
        println!("HANDLING!");
        commands.entity(evt.0).insert(Building(evt.1));
    }
}
