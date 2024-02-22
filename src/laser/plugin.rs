use bevy::prelude::*;
use super::events::LaserEvent;
use super::systems::{
    setup_laser,
    fire_laser_raycasting
};

pub struct LaserPlugin;

impl Plugin for LaserPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LaserEvent>()
            .add_systems(Startup, setup_laser)
            .add_systems(Update, fire_laser_raycasting);
    }
}