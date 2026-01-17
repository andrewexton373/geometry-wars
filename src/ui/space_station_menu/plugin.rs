use bevy::app::{App, Plugin, Update};
use bevy_egui::EguiPrimaryContextPass;

use super::systems::ui_space_station_menu;

pub struct SpaceStationMenu;

impl Plugin for SpaceStationMenu {
    fn build(&self, app: &mut App) {
        app.add_systems(EguiPrimaryContextPass, ui_space_station_menu);
    }
}
