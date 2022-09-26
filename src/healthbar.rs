use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_stat_bars::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use crate::{ Player };

#[derive(Component, Inspectable, Reflect, Default, Clone, Copy, Debug)]
pub struct Health {
    pub current: f32,
    pub maximum: f32,
}

pub struct HealthBarPlugin;

impl Plugin for HealthBarPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(Self::update_player_heath_bar);
    }
}

impl HealthBarPlugin {

    pub fn attach_player_health_bar(
        mut commands: &mut Commands,
        mut camera: Entity
    ) {
        commands.entity(camera)
            .insert_bundle(StatBarBundle::new(
                StatBar { 
                    value: 1.0, 
                    length: 80.0, 
                    thickness: 4.0, 
                    style: StatBarStyle {
                        bar_color: BarColor::Fixed(Color::GREEN),
                        empty_color: Color::RED,
                        ..Default::default()
                    },  
                    translation: Vec2::new(0.0, 30.0),                   
                    ..Default::default()
                }
            ));
    }

    fn update_player_heath_bar(
        player_query: Query<(&Player)>,
        mut camera_stat_bar_query: Query<&mut StatBar, With<Camera2d>>
    ) {
        let (player) = player_query.single();
        let (mut stat_bar) = camera_stat_bar_query.single_mut();

        stat_bar.value = player.health.current / player.health.maximum;
    }
}