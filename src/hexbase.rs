use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_ecs_tilemap::prelude::*;
use bevy_ecs_tilemap::helpers::hex_grid::axial::AxialPos;
use crate::GameCamera;
use crate::player::Player;
use crate::player_input::MousePostion;

const HEX_SIZE: f32 = 58.0 * crate::PIXELS_PER_METER;

const TILE_SIZE_HEX_ROW: TilemapTileSize = TilemapTileSize { x: 443.0, y: 512.0 };
const TILE_SIZE_HEX_COL: TilemapTileSize = TilemapTileSize { x: 443.0, y: 512.0 };
const GRID_SIZE_HEX_ROW: TilemapGridSize = TilemapGridSize { x: 443.0, y: 512.0 };
const GRID_SIZE_HEX_COL: TilemapGridSize = TilemapGridSize { x: 443.0, y: 512.0 };

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
            .add_system(Self::click_to_change_building_type)
            .add_system(Self::color_building_types)
            .add_system(Self::player_interaction)
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
        let RADIUS = 1;
        let map_size = TilemapSize{x: RADIUS * 3, y: RADIUS * 3};
        let origin = TilePos {x: 1, y: 1};
        let coord_system = HexCoordSystem::Row;

        let tile_positions = generate_hexagon(
            AxialPos::from_tile_pos_given_coord_system(&origin, coord_system),
            RADIUS,
        )
            .into_iter()
            .map(|axial_pos| axial_pos.as_tile_pos_given_coord_system(coord_system))
            .collect::<Vec<TilePos>>();

        let tile_positions_with_type = tile_positions.iter().enumerate().map(|(i, pos)| {
            let bt = match i {
                0 => BuildingType::Storage,
                1 => BuildingType::Factory,
                2 => BuildingType::Refinery,
                _ => BuildingType::None
            };

            (pos, bt)
        }).collect::<Vec<(&TilePos, BuildingType)>>();

        let tilemap_entity = commands.spawn_empty().id();
        let mut tile_storage = TileStorage::empty(map_size);
        let tilemap_id = TilemapId(tilemap_entity);

        commands.entity(tilemap_id.0).with_children(|parent| {

            for (tile_pos, building_type) in tile_positions_with_type {

                println!("{:?}", building_type);
                let tile_entity = parent
                    .spawn(TileBundle {
                        position: *tile_pos,
                        tilemap_id,
                        ..Default::default()
                    })
                    .insert(Building(building_type))
                    .id();
                tile_storage.checked_set(&tile_pos, tile_entity)
            }
        });

        let tile_size = TILE_SIZE_HEX_ROW;
        let grid_size = tile_size.into();
        let map_type = TilemapType::Hexagon(HexCoordSystem::Row);

        let mut center_trans = get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0);

        commands.entity(tilemap_entity)
            .insert(TilemapBundle {
               grid_size,
                size: map_size,
                storage: tile_storage,
                texture: TilemapTexture::Single(tile_handle_hex_row.clone()),
                tile_size,
                map_type,
                transform: center_trans,
                ..Default::default()
            })
            .insert(Name::new("Tilemap"));
    }

    fn player_interaction(
        commands: Commands,
        mut player_hovering_building: ResMut<PlayerHoveringBuilding>,
        player_q: Query<(&Player, &GlobalTransform)>,
        tilemap_q: Query<(
            &TilemapSize,
            &TilemapGridSize,
            &TilemapType,
            &TileStorage,
            &Transform,
        )>,
        mut building_tiles_q: Query<(Entity, &Building)>

    ) {
        let (player, p_gt) = player_q.single();
        *player_hovering_building = PlayerHoveringBuilding(None);

        for (map_size, grid_size, map_type, tile_storage, map_transform) in tilemap_q.iter() {


            let player_position = p_gt.translation().truncate();

            let player_in_map_pos: Vec2 = {
                // Extend the cursor_pos vec2 by 0.0 and 1.0
                let player_position = Vec4::from((player_position, 0.0, 1.0));
                let player_in_map_pos = map_transform.compute_matrix().inverse() * player_position;
                Vec2 { x: player_in_map_pos.x, y: player_in_map_pos.y }
            };

            // Once we have a world position we can transform it into a possible tile position.
            if let Some(tile_pos) =
                TilePos::from_world_pos(&player_in_map_pos, map_size, grid_size, map_type)
            {
                // Highlight the relevant tile's label
                if let Some(tile_entity) = tile_storage.get(&tile_pos) {
                    if let Ok((_, building)) = building_tiles_q.get(tile_entity) {
                        *player_hovering_building = PlayerHoveringBuilding(Some((tile_entity, building.0)));
                    }
                }
            }

        }
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
                        println!("some tile pos");
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