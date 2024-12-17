use super::components::{Inventory, InventoryItem};
use bevy::prelude::*;

use crate::{
    items::{Amount, MetalIngot},
    player::{components::Player, resources::EmptyInventoryDepositTimer},
    player_input::events::DepositInventoryEvent,
    space_station::components::SpaceStation,
    upgrades::components::UpgradeComponent,
};

pub fn attach_inventory_to_entity(
    commands: &mut Commands,
    mut inventory: Inventory,
    entity: Entity,
) {
    // TODO: REMOVE ONLY FOR TESTING.
    inventory.add_to_inventory(&InventoryItem::Ingot(
        MetalIngot::IronIngot,
        Amount::Quantity(5),
    ));
    inventory.add_to_inventory(&InventoryItem::Component(
        UpgradeComponent::Cog,
        Amount::Quantity(2),
    ));
    inventory.add_to_inventory(&InventoryItem::Component(
        UpgradeComponent::IronPlate,
        Amount::Quantity(2),
    ));
    commands.entity(entity).insert(inventory);
}

pub fn deposit_inventory(
    _trigger: Trigger<DepositInventoryEvent>,
    // mut deposit_events: EventReader<DepositInventoryEvent>,
    mut player_query: Query<&mut Inventory, (With<Player>, Without<SpaceStation>)>,
    mut base_station_query: Query<&mut Inventory, (With<SpaceStation>, Without<Player>)>,
    mut empty_deposit_timer: ResMut<EmptyInventoryDepositTimer>,
) {
    let mut player_inventory = player_query.single_mut();
    let mut base_station_inventory = base_station_query.single_mut();

    if player_inventory.items.is_empty() {
        let timer = empty_deposit_timer.as_mut();
        *timer = EmptyInventoryDepositTimer(Some(Timer::from_seconds(3.0, TimerMode::Once)));
    }

    for item in player_inventory.clone().items.iter() {
        base_station_inventory.add_to_inventory(item);
        player_inventory.remove_from_inventory(item);
    }
}
