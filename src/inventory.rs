use bevy::utils::HashMap;
use bevy::{prelude::*};

use bevy_inspector_egui::{Inspectable};
use crate::astroid::AstroidMaterial;
use crate::player::Player;
use std::fmt;

#[derive(Component, Default, Debug, Clone, PartialEq)]
pub struct Capacity {
    pub maximum: f32
}

pub const INVENTORY_SIZE: usize = 20;

#[derive(Component, Default, Debug, Clone, PartialEq)]
pub struct Inventory {
    pub items: [Option<ItemAndWeight>; INVENTORY_SIZE],
    pub capacity: Capacity
}

impl Inventory {

    pub fn remaining_capacity(&self) -> f32 {
        self.capacity.maximum - self.gross_material_weight()
    }

    /// Returns the current gross weight of materials in the inventory
    pub fn gross_material_weight(&self) -> f32 {
        let mut gross_weight = 0.0;

        for material in self.items.iter().flatten() {
            gross_weight += material.weight;
        }

        gross_weight
    }

    // I'd rather have the inventory be a hashmap, but was struggling with bevy-inspector traits
    pub fn add_to_inventory(&mut self, material: AstroidMaterial, weight: f32) -> bool {

        if weight > self.remaining_capacity() {
            println!("NOT ENOUGH SHIP CAPACITY! Remaining Capacity: {}", self.remaining_capacity());
            return false;
        }

        let current_inventory = self.items;
        let mut temp_hash_map = HashMap::new();

        // Fill temp hash map with current inventory items
        for item in current_inventory.into_iter().flatten() {
            temp_hash_map.insert(item.item, item.weight);
        }

        // If hasmap already contains the material, add weight to existing material's weight
        if let Some(value) = temp_hash_map.get_mut(&material) {
            *value += weight;
        
        // Otherwise add material and it's respective weight as new entry
        } else {
            temp_hash_map.insert(material, weight);
        }

        let mut updated_inventory: Vec<Option<ItemAndWeight>> = temp_hash_map.into_iter().map(|(k, v)| Some(ItemAndWeight {item: k, weight: v})).collect();
        updated_inventory.sort_by(|a, b| {
            let a = a.unwrap();
            let b = b.unwrap();
            a.item.partial_cmp(&b.item).unwrap()
        });
        while updated_inventory.len() < 20
        {
            updated_inventory.push(None);
        }

        self.items = updated_inventory.try_into().unwrap_or_else(|v: Vec<Option<ItemAndWeight>>| panic!("Expected a Vec of length {} but it was {}", 20, v.len()));
        
        true
    }

    pub fn remove_from_inventory(&mut self, material: AstroidMaterial) -> Option<f32> {

        let current_inventory = self.items;
        let mut temp_hash_map = HashMap::new();

        // Fill temp hash map with current inventory items
        for item in current_inventory.into_iter().flatten() {
            temp_hash_map.insert(item.item, item.weight);
        }

        let result = temp_hash_map.remove(&material);

        let mut updated_inventory: Vec<Option<ItemAndWeight>> = temp_hash_map.into_iter().map(|(k, v)| Some(ItemAndWeight {item: k, weight: v})).collect();
        updated_inventory.sort_by(|a, b| {
            let a = a.unwrap();
            let b = b.unwrap();
            a.item.partial_cmp(&b.item).unwrap()
        });
        while updated_inventory.len() < 20
        {
            updated_inventory.push(None);
        }

        self.items = updated_inventory.try_into().unwrap_or_else(|v: Vec<Option<ItemAndWeight>>| panic!("Expected a Vec of length {} but it was {}", 20, v.len()));


        result

    }
}

#[derive(Component, Default, Debug, Inspectable, Copy, Clone, PartialEq, PartialOrd)]
pub struct ItemAndWeight {
    pub item: AstroidMaterial,
    pub weight: f32
}

impl fmt::Display for ItemAndWeight {
    fn fmt(&self, f: &mut  fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Item: {:?} | Weight: {}", &self.item, &self.weight)
    }
}

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(Self::setup_inventory);
    }
}

impl InventoryPlugin {

        fn setup_inventory(mut commands: Commands) {
            commands.insert_resource(Inventory {
                                        items: [None; INVENTORY_SIZE],
                                        capacity: Capacity { maximum: 1000.0 }});
        }

        pub fn attach_inventory_to_entity(mut commands: &mut Commands, inventory: Inventory, entity: Entity) {
            commands.entity(entity).insert(inventory);
        }

}