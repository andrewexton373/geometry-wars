use bevy::app::App;
use bevy::app::Plugin;
use bevy::app::Update;

use super::systems::ui_ship_information;

pub struct ShipInformationPlugin;

impl Plugin for ShipInformationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (ui_ship_information));
    }
}
