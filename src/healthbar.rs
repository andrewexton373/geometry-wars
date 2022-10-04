use bevy::prelude::*;
use bevy_stat_bars::*;
use bevy_inspector_egui::{Inspectable};
use crate::{ Player };



struct PlayerHealth(f32);
impl StatbarObservable for PlayerHealth {
    fn get_statbar_value(&self) -> f32 {
        self.0
    }
}

pub struct HealthBarPlugin;

impl Plugin for HealthBarPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(PlayerHealth(1.0))
            .add_statbar_resource_observer::<PlayerHealth>()
            .add_system(Self::update_statbar_value);
    }
}

impl HealthBarPlugin {

    pub fn spawn_player_health_statbar(
        mut commands: Commands,
        player_ent: Entity
    ) {
            commands.entity(player_ent)
            .insert_bundle((
                Statbar::<PlayerHealth> {
                    color: Color::GREEN,
                    empty_color: Color::RED,
                    length: 80.,
                    thickness: 8.,
                    vertical: false,
                    displacement: 50. * Vec2::Y,
                    ..Default::default()
                },
                StatbarBorder::<PlayerHealth>::all(Color::WHITE, 2.0),
            ))
            .insert_bundle(SpatialBundle::default());        
    }
    
    fn update_statbar_value(
        player_query: Query<&Player>,
        mut player_health: ResMut<PlayerHealth>,
    ) {
        let player = player_query.single();
        player_health.0 = player.health.current / player.health.maximum;
        player_health.0 = player_health.0.clamp(0.0, 1.0);
    }

}
