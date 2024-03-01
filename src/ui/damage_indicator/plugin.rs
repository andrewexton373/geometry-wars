use bevy::app::{App, Plugin, Update};
use bevy_tweening::TweeningPlugin;

use super::{
    events::DamageIndicatorEvent,
    systems::{damage_indicator_events, remove_post_animation_text},
};

pub struct DamageIndicatorPlugin;

impl Plugin for DamageIndicatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TweeningPlugin)
            .add_event::<DamageIndicatorEvent>()
            .add_systems(
                Update,
                (damage_indicator_events, remove_post_animation_text),
            );
    }
}
