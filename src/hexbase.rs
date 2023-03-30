use bevy::prelude::*;
use bevy::sprite::{Anchor, MaterialMesh2dBundle};
use bevy_ecs_tilemap::prelude::*;
use bevy_ecs_tilemap::helpers::hex_grid::axial::AxialPos;
use crate::GameCamera;
use crate::player::Player;
use crate::player_input::MousePostion;

use std::collections::HashMap;
use std::f32::consts::PI;

use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::window::PrimaryWindow;
use hexx::shapes;
use hexx::*;

// const HEX_SIZE: f32 = 58.0 * crate::PIXELS_PER_METER;

/// World size of the hexagons (outer radius)
const HEX_SIZE: Vec2 = Vec2::splat(10.0 * crate:: PIXELS_PER_METER);

#[derive(Debug, Default, Resource)]
struct HighlightedHexes {
    pub selected: Hex,
    pub halfway: Hex,
    pub ring: Vec<Hex>,
    pub wedge: Vec<Hex>,
    pub dir_wedge: Vec<Hex>,
    pub line: Vec<Hex>,
    pub half_ring: Vec<Hex>,
    pub rotated: Vec<Hex>,
}

#[derive(Debug, Resource)]
struct Map {
    layout: HexLayout,
    entities: HashMap<Hex, Entity>,
    selected_material: Handle<ColorMaterial>,
    ring_material: Handle<ColorMaterial>,
    wedge_material: Handle<ColorMaterial>,
    dir_wedge_material: Handle<ColorMaterial>,
    line_material: Handle<ColorMaterial>,
    half_ring_material: Handle<ColorMaterial>,
    default_material: Handle<ColorMaterial>,
}

// const TILE_SIZE_HEX_ROW: TilemapTileSize = TilemapTileSize { x: 443.0, y: 512.0 };
// const TILE_SIZE_HEX_COL: TilemapTileSize = TilemapTileSize { x: 443.0, y: 512.0 };
// const GRID_SIZE_HEX_ROW: TilemapGridSize = TilemapGridSize { x: 443.0, y: 512.0 };
// const GRID_SIZE_HEX_COL: TilemapGridSize = TilemapGridSize { x: 443.0, y: 512.0 };

#[derive(Component, Debug, Clone, Copy)]
pub enum BuildingType {
    None,
    Factory,
    Refinery,
    Storage
}

#[derive(Component)]
pub struct Building(BuildingType);

#[derive(Component)]
struct Hovered;

#[derive(Resource, Default)]
pub struct PlayerHoveringBuilding(pub(crate) Option<(Entity, BuildingType)>);

#[derive(Deref, Resource)]
pub struct TileHandleHexRow(Handle<Image>);

impl FromWorld for TileHandleHexRow {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self(asset_server.load("hex-station-block.png"))
    }
}

pub struct HexBasePlugin;

impl Plugin for HexBasePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(TilemapPlugin)
            .init_resource::<TileHandleHexRow>()
            .init_resource::<PlayerHoveringBuilding>()
            // .add_system(Self::click_to_change_building_type)
            // .add_system(Self::color_building_types)
            // .add_system(Self::player_interaction)
            // .add_system(Self::hover_highlight_tile_label)
            // .add_system(Self::grow_hovered)
            // .add_startup_system(Self::setup);
            .add_system(Self::handle_input)
            .add_startup_system(Self::setup_hex_grid);
    }
}

impl HexBasePlugin {
    pub fn setup_hex_grid(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        let layout = HexLayout {
            hex_size: HEX_SIZE,
            ..default()
        };
        // materials
        let selected_material = materials.add(Color::RED.into());
        let ring_material = materials.add(Color::YELLOW.into());
        let wedge_material = materials.add(Color::CYAN.into());
        let dir_wedge_material = materials.add(Color::VIOLET.into());
        let line_material = materials.add(Color::ORANGE.into());
        let half_ring_material = materials.add(Color::LIME_GREEN.into());
        let default_material = materials.add(Color::WHITE.into());
        // mesh
        let mesh = Self::hexagonal_plane(&layout);
        let mesh_handle = meshes.add(mesh);

        let entities = shapes::hexagon(Hex::default(), 3)
            .map(|hex| {
                let pos = layout.hex_to_world_pos(hex);
                println!("{:?}", pos);
                let id = commands
                    .spawn(MaterialMesh2dBundle {
                        transform: Transform::from_xyz(pos.x, pos.y, 100.0).with_rotation(Quat::from_rotation_x(PI/2.0)),
                        mesh: bevy::sprite::Mesh2dHandle(mesh_handle.clone()),
                        material: default_material.clone(),
                        ..default()
                    })
                    .insert(Name::new("HEX"))
                    .id();
                (hex, id)
            })
            .collect();
        commands.insert_resource(Map {
            layout,
            entities,
            selected_material,
            ring_material,
            default_material,
            line_material,
            half_ring_material,
            wedge_material,
            dir_wedge_material,
        });
    }

    /// Compute a bevy mesh from the layout
    fn hexagonal_plane(hex_layout: &HexLayout) -> Mesh {
        let mesh_info = MeshInfo::hexagonal_plane(hex_layout, Hex::ZERO);
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices.to_vec());
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals.to_vec());
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs.to_vec());
        mesh.set_indices(Some(Indices::U16(mesh_info.indices)));
        mesh
    }
    /// Input interaction
    fn handle_input(
        mut commands: Commands,
        mouse_position: Res<MousePostion>,
        map: Res<Map>,
        mut highlighted_hexes: Local<HighlightedHexes>,
    ) {
        let pos = mouse_position.0;
        // let window = windows.single();
        // if let Some(pos) = window.cursor_position() {
        //     let pos = Vec2::new(pos.x, window.height() - pos.y)
        //         - Vec2::new(window.width(), window.height()) / 2.0;
            let hex = map.layout.world_pos_to_hex(pos);
            if let Some(entity) = map.entities.get(&hex).copied() {
                if hex == highlighted_hexes.selected {
                    return;
                }
                // Clear highlighted hexes materials
                for vec in [
                    &highlighted_hexes.ring,
                    &highlighted_hexes.line,
                    &highlighted_hexes.wedge,
                    &highlighted_hexes.dir_wedge,
                    &highlighted_hexes.half_ring,
                    &highlighted_hexes.rotated,
                ] {
                    for entity in vec.iter().filter_map(|h| map.entities.get(h)) {
                        commands
                            .entity(*entity)
                            .insert(map.default_material.clone());
                    }
                }
                commands
                    .entity(map.entities[&highlighted_hexes.selected])
                    .insert(map.default_material.clone());
                commands
                    .entity(map.entities[&highlighted_hexes.halfway])
                    .insert(map.default_material.clone());
                // Draw a  line
                highlighted_hexes.line = Hex::ZERO.line_to(hex).collect();
                // Draw a ring
                highlighted_hexes.ring = Hex::ZERO.ring(hex.ulength());
                // Draw an wedge
                highlighted_hexes.wedge = Hex::ZERO.wedge_to(hex).collect();
                // Draw a half ring
                highlighted_hexes.half_ring = Hex::ZERO.ring(hex.ulength() / 2);
                // Draw rotations
                highlighted_hexes.rotated = (1..6).map(|i| hex.rotate_right(i)).collect();
                // Draw an dual wedge
                highlighted_hexes.dir_wedge = Hex::ZERO.corner_wedge_to(hex / 2).collect();
                for (vec, mat) in [
                    (&highlighted_hexes.ring, &map.ring_material),
                    (&highlighted_hexes.wedge, &map.wedge_material),
                    (&highlighted_hexes.dir_wedge, &map.dir_wedge_material),
                    (&highlighted_hexes.line, &map.line_material),
                    (&highlighted_hexes.half_ring, &map.half_ring_material),
                    (&highlighted_hexes.rotated, &map.selected_material),
                ] {
                    for h in vec {
                        if let Some(e) = map.entities.get(h) {
                            commands.entity(*e).insert(mat.clone());
                        }
                    }
                }
                // Make the half selction red
                highlighted_hexes.halfway = hex / 2;
                commands
                    .entity(map.entities[&highlighted_hexes.halfway])
                    .insert(map.selected_material.clone());
                // Make the selected tile red
                commands
                    .entity(entity)
                    .insert(map.selected_material.clone());
                highlighted_hexes.selected = hex;
        //     }
        }
    }


    // pub fn setup(
    //     mut commands: Commands,
    //     tile_handle_hex_row: Res<TileHandleHexRow>
    // ) {
    //     let RADIUS = 1;
    //     let map_size = TilemapSize{x: RADIUS * 3, y: RADIUS * 3};
    //     let origin = TilePos {x: 1, y: 1};
    //     let coord_system = HexCoordSystem::Row;
    //
    //     let tile_positions = generate_hexagon(
    //         AxialPos::from_tile_pos_given_coord_system(&origin, coord_system),
    //         RADIUS,
    //     )
    //         .into_iter()
    //         .map(|axial_pos| axial_pos.as_tile_pos_given_coord_system(coord_system))
    //         .collect::<Vec<TilePos>>();
    //
    //     let tile_positions_with_type = tile_positions.iter().enumerate().map(|(i, pos)| {
    //         let bt = match i {
    //             0 => BuildingType::Storage,
    //             1 => BuildingType::Factory,
    //             2 => BuildingType::Refinery,
    //             _ => BuildingType::None
    //         };
    //
    //         (pos, bt)
    //     }).collect::<Vec<(&TilePos, BuildingType)>>();
    //
    //     let tilemap_entity = commands.spawn_empty().id();
    //     let mut tile_storage = TileStorage::empty(map_size);
    //     let tilemap_id = TilemapId(tilemap_entity);
    //
    //     commands.entity(tilemap_id.0).with_children(|parent| {
    //
    //         for (tile_pos, building_type) in tile_positions_with_type {
    //
    //             println!("{:?}", building_type);
    //             let tile_entity = parent
    //                 .spawn(TileBundle {
    //                     position: *tile_pos,
    //                     tilemap_id,
    //                     ..Default::default()
    //                 })
    //                 .insert(Building(building_type))
    //                 .id();
    //             tile_storage.checked_set(&tile_pos, tile_entity)
    //         }
    //     });
    //
    //     let tile_size = TILE_SIZE_HEX_ROW;
    //     let grid_size = tile_size.into();
    //     let map_type = TilemapType::Hexagon(HexCoordSystem::Row);
    //
    //     let mut center_trans = get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0);
    //
    //     commands.entity(tilemap_entity)
    //         .insert(TilemapBundle {
    //            grid_size,
    //             size: map_size,
    //             storage: tile_storage,
    //             texture: TilemapTexture::Single(tile_handle_hex_row.clone()),
    //             tile_size,
    //             map_type,
    //             transform: center_trans,
    //             ..Default::default()
    //         })
    //         .insert(Name::new("Tilemap"));
    // }
    //
    // fn player_interaction(
    //     commands: Commands,
    //     mut player_hovering_building: ResMut<PlayerHoveringBuilding>,
    //     player_q: Query<(&Player, &GlobalTransform)>,
    //     tilemap_q: Query<(
    //         &TilemapSize,
    //         &TilemapGridSize,
    //         &TilemapType,
    //         &TileStorage,
    //         &Transform,
    //     )>,
    //     mut building_tiles_q: Query<(Entity, &Building)>
    //
    // ) {
    //     let (player, p_gt) = player_q.single();
    //     *player_hovering_building = PlayerHoveringBuilding(None);
    //
    //     for (map_size, grid_size, map_type, tile_storage, map_transform) in tilemap_q.iter() {
    //
    //
    //         let player_position = p_gt.translation().truncate();
    //
    //         let player_in_map_pos: Vec2 = {
    //             // Extend the cursor_pos vec2 by 0.0 and 1.0
    //             let player_position = Vec4::from((player_position, 0.0, 1.0));
    //             let player_in_map_pos = map_transform.compute_matrix().inverse() * player_position;
    //             Vec2 { x: player_in_map_pos.x, y: player_in_map_pos.y }
    //         };
    //
    //         // Once we have a world position we can transform it into a possible tile position.
    //         if let Some(tile_pos) =
    //             TilePos::from_world_pos(&player_in_map_pos, map_size, grid_size, map_type)
    //         {
    //             // Highlight the relevant tile's label
    //             if let Some(tile_entity) = tile_storage.get(&tile_pos) {
    //                 if let Ok((_, building)) = building_tiles_q.get(tile_entity) {
    //                     *player_hovering_building = PlayerHoveringBuilding(Some((tile_entity, building.0)));
    //                 }
    //             }
    //         }
    //
    //     }
    // }
    //
    // fn click_to_change_building_type(
    //     mut commands: Commands,
    //     mouse_input: Res<Input<MouseButton>>,
    //     mouse_position: Res<MousePostion>,
    //     tilemap_q: Query<(
    //         &TilemapSize,
    //         &TilemapGridSize,
    //         &TilemapType,
    //         &TileStorage,
    //         &Transform,
    //     )>,
    //
    // ) {
    //
    //     if mouse_input.just_released(MouseButton::Left) {
    //
    //         let cursor_pos = mouse_position.0;
    //
    //         for (map_size, grid_size, map_type, tile_storage, map_transform) in tilemap_q.iter() {
    //
    //                 let cursor_in_map_pos: Vec2 = {
    //                     // Extend the cursor_pos vec2 by 0.0 and 1.0
    //                     let cursor_pos = Vec4::from((cursor_pos, 0.0, 1.0));
    //                     let cursor_in_map_pos = map_transform.compute_matrix().inverse() * cursor_pos;
    //                     Vec2 { x: cursor_in_map_pos.x, y: cursor_in_map_pos.y }
    //                 };
    //
    //                 // Once we have a world position we can transform it into a possible tile position.
    //                 if let Some(tile_pos) =
    //                     TilePos::from_world_pos(&cursor_in_map_pos, map_size, grid_size, map_type)
    //                 {
    //                     println!("some tile pos");
    //                     // Highlight the relevant tile's label
    //                     if let Some(tile_entity) = tile_storage.get(&tile_pos) {
    //                         println!("Setting Building Type To Storage.");
    //                         commands.entity(tile_entity).insert(Building(BuildingType::Storage));
    //                     }
    //                 }
    //
    //         }
    //     }
    // }
    //
    // fn color_building_types(
    //     mut building_tiles_q: Query<(&Building, &mut TileColor)>
    //
    // ) {
    //     for (building, mut color) in building_tiles_q.iter_mut() {
    //
    //         let building_color = match building.0 {
    //             BuildingType::None => Color::WHITE,
    //             BuildingType::Factory => Color::RED,
    //             BuildingType::Refinery => Color::BLUE,
    //             BuildingType::Storage => Color::GREEN
    //         };
    //
    //         *color = building_color.into();
    //     }
    // }
    //
    // fn hover_highlight_tile_label(
    //     mut commands: Commands,
    //     tilemap_q: Query<(
    //         &TilemapSize,
    //         &TilemapGridSize,
    //         &TilemapType,
    //         &TileStorage,
    //         &Transform,
    //     )>,
    //     highlighted_tiles_q: Query<Entity, With<Hovered>>,
    //     mouse_position: Res<MousePostion>,
    //    // tile_label_q: Query<&TileLabel>,
    //     mut text_q: Query<&mut Text>,
    // ) {
    //     let cursor_pos = mouse_position.into_inner().0;
    //
    //
    //     // Un-highlight any previously highlighted tile labels.
    //     for highlighted_tile_entity in highlighted_tiles_q.iter() {
    //         // if let Ok(label) = tile_label_q.get(highlighted_tile_entity) {
    //         //     if let Ok(mut tile_text) = text_q.get_mut(label.0) {
    //         //         for mut section in tile_text.sections.iter_mut() {
    //         //             section.style.color = Color::BLACK;
    //         //         }
    //         //         commands.entity(highlighted_tile_entity).remove::<Hovered>();
    //         //     }
    //         // }
    //         commands.entity(highlighted_tile_entity).remove::<Hovered>();
    //
    //     }
    //
    //     for (map_size, grid_size, map_type, tile_storage, map_transform) in tilemap_q.iter() {
    //
    //         let cursor_in_map_pos: Vec2 = {
    //             // Extend the cursor_pos vec2 by 0.0 and 1.0
    //             let cursor_pos = Vec4::from((cursor_pos, 0.0, 1.0));
    //             let cursor_in_map_pos = map_transform.compute_matrix().inverse() * cursor_pos;
    //             Vec2 { x: cursor_in_map_pos.x, y: cursor_in_map_pos.y }
    //         };
    //
    //         // Once we have a world position we can transform it into a possible tile position.
    //         if let Some(tile_pos) =
    //             TilePos::from_world_pos(&cursor_in_map_pos, map_size, grid_size, map_type)
    //         {
    //             // Highlight the relevant tile's label
    //             if let Some(tile_entity) = tile_storage.get(&tile_pos) {
    //                 println!("HIT!!!");
    //                 // if let Ok(label) = tile_label_q.get(tile_entity) {
    //                 //     if let Ok(mut tile_text) = text_q.get_mut(label.0) {
    //                 //         for mut section in tile_text.sections.iter_mut() {
    //                 //             section.style.color = Color::RED;
    //                 //         }
    //                 //         commands.entity(tile_entity).insert(Hovered);
    //                 //     }
    //                 // }
    //                 commands.entity(tile_entity).insert(Hovered);
    //
    //             }
    //         }
    //
    //     }
    // }
    //
    // fn grow_hovered(
    //     mut hovered_query: Query<(&mut Transform, &Hovered)>
    // ) {
    //     for (mut t, hov) in hovered_query.iter_mut() {
    //         println!("HIT");
    //         t.scale += Vec3{ x: 2.0, y: 2.0, z: 1.0 };
    //     }
    // }
}