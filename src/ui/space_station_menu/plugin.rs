use bevy::app::{App, Plugin, Update};

use super::systems::ui_space_station_menu;

pub struct SpaceStationMenu;

impl Plugin for SpaceStationMenu {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, ui_space_station_menu);
    }
}
