use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

const HEX_SIZE: f32 = 50.0 * crate::PIXELS_PER_METER;

const TILE_SIZE_HEX_ROW: TilemapTileSize = TilemapTileSize { x: HEX_SIZE, y: HEX_SIZE };
const TILE_SIZE_HEX_COL: TilemapTileSize = TilemapTileSize { x: HEX_SIZE, y: HEX_SIZE };
const GRID_SIZE_HEX_ROW: TilemapGridSize = TilemapGridSize { x: HEX_SIZE, y: HEX_SIZE };
const GRID_SIZE_HEX_COL: TilemapGridSize = TilemapGridSize { x: HEX_SIZE, y: HEX_SIZE };


pub struct HexBasePlugin;

impl Plugin for HexBasePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(Self::setup);
    }
}

impl HexBasePlugin {
    pub fn setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>
    ) {

        let texture_handle: Handle<Image> = asset_server.load("flat_hex_tiles.png");
        let map_size = TilemapSize{x: 8, y: 8};

        let tilemap_entity = commands.spawn_empty().id();
        let mut tile_storage = TileStorage::empty(map_size);
        let tilemap_id = TilemapId(tilemap_entity);

        fill_tilemap(
            TileTextureIndex(0),
            map_size,
            tilemap_id,
            &mut commands,
            &mut tile_storage
        );

        let tile_size = TILE_SIZE_HEX_ROW;
        let grid_size = GRID_SIZE_HEX_ROW;
        let map_type = TilemapType::Hexagon(HexCoordSystem::Row);

        commands.entity(tilemap_entity).insert(TilemapBundle {
           grid_size,
            size: map_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(texture_handle.clone()),
            tile_size,
            map_type,
            transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 999.0),
            ..Default::default()
        });
    }
}