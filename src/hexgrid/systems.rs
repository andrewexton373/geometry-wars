use bevy::pbr::wireframe::WireframeMaterial;
use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_xpbd_2d::components::Collider;
use hexx::{shapes, Hex, HexLayout, PlaneMeshBuilder};
use std::f32::consts::PI;

use crate::hexgrid::components::Building;
use crate::player::components::Player;
use crate::player_input::resources::MouseWorldPosition;

use super::components::{HexTile, BuildingType};
use super::events::BuildHexBuildingEvent;
use super::plugin::HEX_SIZE;
use super::resources::{HexGridMap, HighlightedHexes, MouseHoverHex, PlayerHoveringBuilding, SelectedHex};

pub fn setup_hex_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let layout = HexLayout {
        hex_size: HEX_SIZE,
        origin: Vec2::ZERO,
        ..default()
    };

    // layout.h
    // materials
    let mouse_hover_material = materials.add(Color::DARK_GRAY.into());
    let selected_material = materials.add(Color::RED.into());
    let ship_hover_material = materials.add(Color::LIME_GREEN.into());
    let ring_material = materials.add(Color::YELLOW.into());
    let default_material = materials.add(
        Color::Rgba {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha: 0.0,
        }
        .into(),
    );
    let factory_material = materials.add(Color::BISQUE.into());
    let refinery_material = materials.add(Color::ORANGE_RED.into());
    let storage_material = materials.add(Color::GOLD.into());

    // mesh
    let mesh = hexagonal_plane(&layout);
    let mesh_handle = meshes.add(mesh);

    let points: Vec<Vec2> = HexLayout::hex_corners(&layout, Hex::ZERO).into();
    let collider = Collider::convex_hull(points).unwrap();

    let entities = shapes::hexagon(Hex::default(), 1)
        .map(|hex| {
            let pos = layout.hex_to_world_pos(hex);

            let id = commands
                .spawn(MaterialMesh2dBundle {
                    transform: Transform::from_xyz(pos.x, pos.y, 0.0),
                        // .with_rotation(Quat::from_rotation_x(PI / 2.0)),
                    mesh: bevy::sprite::Mesh2dHandle(mesh_handle.clone()),
                    material: default_material.clone(),
                    ..default()
                })
                .with_children(|b| {
                    b.spawn(Text2dBundle {
                        text: Text::from_section(
                            format!("{},{}", hex.x, hex.y),
                            TextStyle {
                                font_size: 32.0,
                                color: Color::BLACK,
                                ..default()
                            },
                        ),
                        transform: Transform::from_xyz(0.0, 0.0, 10.0),
                        ..default()
                    });
                })
                .insert(collider.clone())
                .insert(Name::new("HexTile"))
                .insert(HexTile)
                .insert(Building(BuildingType::None))
                .id();
            (hex, id)
        })
        .collect();
    commands.insert_resource(HexGridMap {
        layout,
        entities,
        mouse_hover_material,
        selected_material,
        ship_hover_material,
        ring_material,
        default_material,
        factory_material,
        refinery_material,
        storage_material,
    });
}

fn hexagonal_plane(hex_layout: &HexLayout) -> Mesh {
    let mesh_info = PlaneMeshBuilder::new(hex_layout)
        .facing(Vec3::Z)
        .center_aligned()
        .build();
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
        .with_indices(Some(Indices::U16(mesh_info.indices)));
    mesh
}

/// Input interaction
pub fn handle_mouse_interaction(
    mut _commands: Commands,
    mouse_position: Res<MouseWorldPosition>,
    map: Res<HexGridMap>,
    mut highlighted_hexes: ResMut<HighlightedHexes>,
    mouse_input: Res<Input<MouseButton>>,
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

pub fn color_mouse_hover_hex(

) {

}


pub fn update_mouse_hover_hex(
    mut _commands: Commands,
    mouse_position: Res<MouseWorldPosition>,
    map: Res<HexGridMap>,
    mut mouse_hover_hex: ResMut<MouseHoverHex>
) {
    let pos = mouse_position.0;

    let hex = map
        .layout
        .world_pos_to_hex(hexx::Vec2 { x: pos.x, y: pos.y });
    if let Some(entity) = map.entities.get(&hex).copied() {

        *mouse_hover_hex = MouseHoverHex {
            entity: Some(entity),
            hover_hex: Some(hex)
        }

    }
}

pub fn update_selected_hex(
    mouse_events: Res<Input<MouseButton>>,
    mouse_hover_hex: Res<MouseHoverHex>,
    mut selected_hex: ResMut<SelectedHex>
) {
    if mouse_events.just_pressed(MouseButton::Left) {
        *selected_hex = SelectedHex {
            entity: mouse_hover_hex.entity,
            selected_hex: mouse_hover_hex.hover_hex
        };

        dbg!("Selected Hex: {}", selected_hex);
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

// fn color_building_types(
//     mut commands: Commands,
//     map: Res<Map>,
//     mut hex_query: Query<(Entity, &BaseHex, &mut Building)>,
// ) {
//     for (ent, _, building) in hex_query.iter_mut() {
//
//         let color = match building.0 {
//             BuildingType::None => Some(map.default_material.clone()),
//             BuildingType::Factory => Some(map.factory_material.clone()),
//             BuildingType::Refinery => Some(map.refinery_material.clone()),
//             BuildingType::Storage => Some(map.storage_material.clone())
//         };
//
//         if let Some(color) = color {
//             commands.entity(ent).insert(color);
//         }
//     }
// }

pub fn color_hexes(
    mut commands: Commands,
    _mouse_pos: Res<MouseWorldPosition>,
    map: Res<HexGridMap>,
    highlighted: Res<HighlightedHexes>,
    mut hex_query: Query<(Entity, &HexTile, &mut Building)>,
    mouse_hover_hex: Res<MouseHoverHex>,
    selected_hex: Res<SelectedHex>
) {
    // 1: Color By Building Type
    for (ent, _, building) in hex_query.iter_mut() {
        let color = match building.0 {
            BuildingType::None => Some(map.default_material.clone()),
            BuildingType::Factory => Some(map.factory_material.clone()),
            BuildingType::Refinery => Some(map.refinery_material.clone()),
            BuildingType::Storage => Some(map.storage_material.clone()),
        };

        if let Some(color) = color {
            commands.entity(ent).insert(color);
        }
    }

    // 2: Color Mouse Hover
    if let Some(entity) = mouse_hover_hex.entity {
        commands
        .entity(entity)
        .insert(map.mouse_hover_material.clone());
        }

    // 3: Color Ship Hover
    let ship_hover_ent = map.entities.get(&highlighted.ship_hover).unwrap();
    commands
        .entity(*ship_hover_ent)
        .insert(map.ship_hover_material.clone());

    if let Some(entity) = selected_hex.entity {
        commands
        .entity(entity)
        .insert(map.selected_material.clone());
    }



    // 4: Wireframe (Stroke) Buildable Hexes

    // 4: Ring?
    // for (vec, mat) in [
    //     (&highlighted_hexes.ring, &map.ring_material),
    // ] {
    //     for h in vec {
    //         if let Some(e) = map.entities.get(h) {
    //             commands.entity(*e).insert(mat.clone());
    //         }
    //     }
    // }
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
