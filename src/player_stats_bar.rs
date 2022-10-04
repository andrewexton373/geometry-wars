use bevy::prelude::*;
use bevy_stat_bars::*;
use bevy_inspector_egui::{Inspectable};
use crate::{ Player, inventory::Inventory };

struct PlayerHealth(f32);
impl StatbarObservable for PlayerHealth {
    fn get_statbar_value(&self) -> f32 {
        self.0
    }
}

struct PlayerShipCapacity(f32);
impl StatbarObservable for PlayerShipCapacity {
    fn get_statbar_value(&self) -> f32 {
        self.0
    }
}

pub struct PlayerStatsBarPlugin;

impl Plugin for PlayerStatsBarPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(PlayerHealth(1.0))
            .insert_resource(PlayerShipCapacity(1.0))
            .add_statbar_resource_observer::<PlayerHealth>()
            .add_statbar_resource_observer::<PlayerShipCapacity>()
            .add_system(Self::update_statbar_values);
    }
}

impl PlayerStatsBarPlugin {

    pub fn spawn_player_health_statbar(
        mut commands: &mut Commands,
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

    pub fn spawn_ship_capacity_statbar(
        mut commands: &mut Commands,
        player_ent: Entity
    ) {
        commands.entity(player_ent)
        .insert_bundle((
            Statbar::<PlayerShipCapacity> {
                color: Color::ORANGE,
                empty_color: Color::DARK_GRAY,
                length: 80.,
                thickness: 8.,
                vertical: false,
                displacement: 65. * Vec2::Y,
                ..Default::default()
            },
            StatbarBorder::<PlayerShipCapacity>::all(Color::WHITE, 2.0),
        ))
        .insert_bundle(SpatialBundle::default());       
    }
    
    fn update_statbar_values(
        player_query: Query<&Player>,
        inventory_res: Res<Inventory>,

        mut player_health: ResMut<PlayerHealth>,
        mut player_ship_capacity: ResMut<PlayerShipCapacity>,

    ) {
        let player = player_query.single();

        player_health.0 = player.health.current / player.health.maximum;
        player_health.0 = player_health.0.clamp(0.0, 1.0);

        player_ship_capacity.0 = 1.0 - inventory_res.remaining_capacity() / inventory_res.capacity.maximum;
        player_ship_capacity.0 = player_ship_capacity.0.clamp(0.0, 1.0);
    }

}
