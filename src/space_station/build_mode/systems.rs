use bevy::{
    ecs::{
        event::EventReader,
        schedule::NextState,
        system::{Res, ResMut},
    },
    input::{
        keyboard::{KeyCode, KeyboardInput},
        Input,
    },
};

use crate::{
    ui::context_clue::resources::{ContextClue, ContextClues},
    AppState,
};

pub fn highlight_build_locations(mut context_clues: ResMut<ContextClues>) {}

pub fn handle_build_mode_enter(
    keys: Res<Input<KeyCode>>,
    mut game_state: ResMut<NextState<AppState>>,
    mut context_clues: ResMut<ContextClues>,
) {
    if keys.just_pressed(KeyCode::B) {
        game_state.set(AppState::BuildMode);
        context_clues.0.insert(ContextClue::BuildModeEnabled);
    }
}

pub fn handle_build_mode_exit(
    keys: Res<Input<KeyCode>>,
    mut game_state: ResMut<NextState<AppState>>,
    mut context_clues: ResMut<ContextClues>,
) {
    if keys.just_pressed(KeyCode::B) {
        game_state.set(AppState::InGame);
        context_clues.0.remove(&ContextClue::BuildModeEnabled);
    }
}
