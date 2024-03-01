use bevy::app::{App, Plugin};

use super::events::UpgradeEvent;

pub struct UpgradesPlugin;

impl Plugin for UpgradesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UpgradeEvent>();
    }
}
