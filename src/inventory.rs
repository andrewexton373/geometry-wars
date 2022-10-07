use bevy::utils::HashMap;
use bevy::{prelude::*};

use bevy_inspector_egui::{Inspectable};
use crate::astroid::AstroidMaterial;
use std::fmt;

#[derive(Component, Default, Debug, Clone, PartialEq)]
pub struct Capacity {
    pub maximum: f32
}

#[derive(Component, Default, Debug, Clone, PartialEq)]
pub struct Inventory {
    pub items: HashMap<AstroidMaterial, f32>,
    pub capacity: Capacity
}

impl Inventory {

    pub fn remaining_capacity(&self) -> f32 {
        self.capacity.maximum - self.gross_material_weight()
    }

    /// Returns the current gross weight of materials in the inventory
    pub fn gross_material_weight(&self) -> f32 {
        let mut gross_weight = 0.0;

        for material in self.items.iter() {
            gross_weight += material.1;
        }

        gross_weight
    }

    // I'd rather have the inventory be a hashmap, but was struggling with bevy-inspector traits
    pub fn add_to_inventory(&mut self, material: AstroidMaterial, weight: f32) -> bool {

        if weight > self.remaining_capacity() {
            println!("NOT ENOUGH SHIP CAPACITY! Remaining Capacity: {}", self.remaining_capacity());
            return false;
        }

        if self.items.contains_key(&material) {
            println!("subsequent pickup");
            let item = self.items.get_key_value_mut(&material).unwrap();
            *item.1 += weight;
        } else {
            println!("first pickup");
            self.items.insert(material, weight);
        }

        println!("{:?}", self.items);
        
        true
    }

    pub fn remove_from_inventory(&mut self, material: AstroidMaterial) -> Option<f32> {

        // let current_inventory = self.items;

        if self.items.contains_key(&material) {
            return self.items.remove(&material);
        }

        None

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
                                        items: HashMap::new(),
                                        capacity: Capacity { maximum: 1000.0 }});
        }

        pub fn attach_inventory_to_entity(commands: &mut Commands, inventory: Inventory, entity: Entity) {
            commands.entity(entity).insert(inventory);
        }
}