use bevy::prelude::*;
use super::components::{
    Inventory,
    InventoryItem,
};

use crate::{items::{Amount, MetalIngot}, upgrades::UpgradeComponent};

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