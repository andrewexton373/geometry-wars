use bevy::{
    asset::Assets,
    color::Color,
    ecs::{
        entity::Entity,
        event::EventReader,
        query::Without,
        system::{Commands, Query, Res, ResMut},
    },
    input::{keyboard::KeyCode, ButtonInput},
    prelude::NextState,
    sprite::{ColorMaterial, MeshMaterial2d},
};

use crate::{
    hexgrid::{
        components::HexTile,
        resources::{MouseHoverHex, SelectedHex},
    },
    player_input::resources::MouseWorldPosition,
    space_station::modules::components::SpaceStationModuleType,
    ui::context_clue::resources::{ContextClue, ContextClues},
    AppState,
};

use super::{
    components::BuildableHex, events::BuildSpaceStationModuleEvent, resources::BuildModeMaterials,
};

pub fn init_materials(
    mut materials: ResMut<BuildModeMaterials>,
    mut assets: ResMut<Assets<ColorMaterial>>,
) {
    *materials = BuildModeMaterials {
        buildable_hex_material: assets.add(Color::rgba(1.0, 1.0, 1.0, 0.5)),
        mouse_hover_hex_material: assets.add(Color::rgba(1.0, 1.0, 1.0, 0.7)),
        selected_hex_material: assets.add(Color::rgba(1.0, 1.0, 1.0, 0.9)),
    }
}

pub fn color_hexes(
    mut commands: Commands,
    _mouse_pos: Res<MouseWorldPosition>,
    mouse_hover_hex: Res<MouseHoverHex>,
    selected_hex: Res<SelectedHex>,
    materials: Res<BuildModeMaterials>,
) {
    // 2: Color Mouse Hover
    if let Some(entity) = mouse_hover_hex.entity {
        commands
            .entity(entity)
            .insert(MeshMaterial2d(materials.mouse_hover_hex_material.clone()));
    }

    // 3: Color Selected Hover
    if let Some(entity) = selected_hex.entity {
        commands
            .entity(entity)
            .insert(MeshMaterial2d(materials.selected_hex_material.clone()));
    }
}

pub fn highlight_build_locations(
    mut commands: Commands,
    build_locations_q: Query<(Entity, &HexTile), Without<SpaceStationModuleType>>,
    materials: Res<BuildModeMaterials>,
) {
    for (build_location_ent, _) in build_locations_q.iter() {
        commands
            .entity(build_location_ent)
            .insert(MeshMaterial2d(materials.buildable_hex_material.clone()))
            .insert(BuildableHex);
    }
}

pub fn handle_build_events(
    mut commands: Commands,
    mut build_events: EventReader<BuildSpaceStationModuleEvent>,
) {
    for build_event in build_events.read() {
        // dbg!("{:?}", build_event.module_type);
        commands
            .entity(build_event.entity)
            .insert(build_event.module_type);
    }
}

pub fn handle_build_mode_enter(
    keys: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<AppState>>,
    mut context_clues: ResMut<ContextClues>,
) {
    if keys.just_pressed(KeyCode::KeyB) {
        game_state.set(AppState::BuildMode);
        context_clues.0.insert(ContextClue::BuildModeEnabled);
    }
}

pub fn handle_build_mode_exit(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<AppState>>,
    mut context_clues: ResMut<ContextClues>,
    hex_tile_q: Query<(Entity, &HexTile)>,
) {
    if keys.just_pressed(KeyCode::KeyB) {
        game_state.set(AppState::InGame);
        context_clues.0.remove(&ContextClue::BuildModeEnabled);

        for (ent, _) in hex_tile_q.iter() {
            commands.entity(ent).remove::<BuildableHex>();
        }
    }
}
