use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use crate::GameCamera;
use crate::player_input::MousePostion;

const HEX_SIZE: f32 = 58.0 * crate::PIXELS_PER_METER;

const TILE_SIZE_HEX_ROW: TilemapTileSize = TilemapTileSize { x: 50.0, y: 58.0 };
const TILE_SIZE_HEX_COL: TilemapTileSize = TilemapTileSize { x: 58.0, y: 50.0 };
const GRID_SIZE_HEX_ROW: TilemapGridSize = TilemapGridSize { x: 50.0, y: 58.0 };
const GRID_SIZE_HEX_COL: TilemapGridSize = TilemapGridSize { x: 58.0, y: 50.0 };

#[derive(Component)]
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

#[derive(Deref, Resource)]
pub struct TileHandleHexRow(Handle<Image>);

impl FromWorld for TileHandleHexRow {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self(asset_server.load("bw-tile-hex-row.png"))
    }
}

pub struct HexBasePlugin;

impl Plugin for HexBasePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(TilemapPlugin)
            .init_resource::<TileHandleHexRow>()
            .add_system(Self::click_to_change_building_type)
            .add_system(Self::color_building_types)
            // .add_system(Self::hover_highlight_tile_label)
            // .add_system(Self::grow_hovered)
            .add_startup_system(Self::setup);
    }
}

impl HexBasePlugin {
    pub fn setup(
        mut commands: Commands,
        tile_handle_hex_row: Res<TileHandleHexRow>
    ) {
        println!("SETUP HEX");
        let map_size = TilemapSize{x: 3, y: 3};

        let tilemap_entity = commands.spawn_empty().id();
        let mut tile_storage = TileStorage::empty(map_size);
        let tilemap_id = TilemapId(tilemap_entity);

        fill_tilemap_hexagon(
            TileTextureIndex(0),
            TilePos {
                x: 1,
                y: 1,
            },
            1,
            HexCoordSystem::Row,
            tilemap_id,
            &mut commands,
            &mut tile_storage
        );

        let tile_size = TILE_SIZE_HEX_ROW;
        let grid_size = GRID_SIZE_HEX_ROW;
        let map_type = TilemapType::Hexagon(HexCoordSystem::Row);

        commands.entity(tilemap_entity)
            .insert(TilemapBundle {
               grid_size,
                size: map_size,
                storage: tile_storage,
                texture: TilemapTexture::Single(tile_handle_hex_row.clone()),
                tile_size,
                map_type,
                transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
                ..Default::default()
            })
            .insert(Name::new("Tilemap"));
    }

    fn click_to_change_building_type(
        mut commands: Commands,
        mouse_input: Res<Input<MouseButton>>,
        mouse_position: Res<MousePostion>,
        tilemap_q: Query<(
            &TilemapSize,
            &TilemapGridSize,
            &TilemapType,
            &TileStorage,
            &Transform,
        )>,
    ) {

        if mouse_input.just_released(MouseButton::Left) {

            let cursor_pos = mouse_position.0;

            for (map_size, grid_size, map_type, tile_storage, map_transform) in tilemap_q.iter() {

                    let cursor_in_map_pos: Vec2 = {
                        // Extend the cursor_pos vec2 by 0.0 and 1.0
                        let cursor_pos = Vec4::from((cursor_pos, 0.0, 1.0));
                        let cursor_in_map_pos = map_transform.compute_matrix().inverse() * cursor_pos;
                        Vec2 { x: cursor_in_map_pos.x, y: cursor_in_map_pos.y }
                    };

                    // Once we have a world position we can transform it into a possible tile position.
                    if let Some(tile_pos) =
                        TilePos::from_world_pos(&cursor_in_map_pos, map_size, grid_size, map_type)
                    {
                        // Highlight the relevant tile's label
                        if let Some(tile_entity) = tile_storage.get(&tile_pos) {
                            println!("Setting Building Type To Storage.");
                            commands.entity(tile_entity).insert(Building(BuildingType::Storage));
                        }
                    }

            }
        }
    }

    fn color_building_types(
        mut building_tiles_q: Query<(&Building, &mut TileColor)>

    ) {
        for (building, mut color) in building_tiles_q.iter_mut() {

            let building_color = match building.0 {
                BuildingType::None => Color::WHITE,
                BuildingType::Factory => Color::RED,
                BuildingType::Refinery => Color::BLUE,
                BuildingType::Storage => Color::GREEN
            };

            *color = building_color.into();
        }
    }

    fn hover_highlight_tile_label(
        mut commands: Commands,
        tilemap_q: Query<(
            &TilemapSize,
            &TilemapGridSize,
            &TilemapType,
            &TileStorage,
            &Transform,
        )>,
        highlighted_tiles_q: Query<Entity, With<Hovered>>,
        mouse_position: Res<MousePostion>,
       // tile_label_q: Query<&TileLabel>,
        mut text_q: Query<&mut Text>,
    ) {
        let cursor_pos = mouse_position.into_inner().0;


        // Un-highlight any previously highlighted tile labels.
        for highlighted_tile_entity in highlighted_tiles_q.iter() {
            // if let Ok(label) = tile_label_q.get(highlighted_tile_entity) {
            //     if let Ok(mut tile_text) = text_q.get_mut(label.0) {
            //         for mut section in tile_text.sections.iter_mut() {
            //             section.style.color = Color::BLACK;
            //         }
            //         commands.entity(highlighted_tile_entity).remove::<Hovered>();
            //     }
            // }
            commands.entity(highlighted_tile_entity).remove::<Hovered>();

        }

        for (map_size, grid_size, map_type, tile_storage, map_transform) in tilemap_q.iter() {

            let cursor_in_map_pos: Vec2 = {
                // Extend the cursor_pos vec2 by 0.0 and 1.0
                let cursor_pos = Vec4::from((cursor_pos, 0.0, 1.0));
                let cursor_in_map_pos = map_transform.compute_matrix().inverse() * cursor_pos;
                Vec2 { x: cursor_in_map_pos.x, y: cursor_in_map_pos.y }
            };

            // Once we have a world position we can transform it into a possible tile position.
            if let Some(tile_pos) =
                TilePos::from_world_pos(&cursor_in_map_pos, map_size, grid_size, map_type)
            {
                // Highlight the relevant tile's label
                if let Some(tile_entity) = tile_storage.get(&tile_pos) {
                    println!("HIT!!!");
                    // if let Ok(label) = tile_label_q.get(tile_entity) {
                    //     if let Ok(mut tile_text) = text_q.get_mut(label.0) {
                    //         for mut section in tile_text.sections.iter_mut() {
                    //             section.style.color = Color::RED;
                    //         }
                    //         commands.entity(tile_entity).insert(Hovered);
                    //     }
                    // }
                    commands.entity(tile_entity).insert(Hovered);

                }
            }

        }
    }

    fn grow_hovered(
        mut hovered_query: Query<(&mut Transform, &Hovered)>
    ) {
        for (mut t, hov) in hovered_query.iter_mut() {
            println!("HIT");
            t.scale += Vec3{ x: 2.0, y: 2.0, z: 1.0 };
        }
    }
}