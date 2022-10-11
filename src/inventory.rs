use bevy::utils::HashMap;
use bevy::{prelude::*};

use bevy_inspector_egui::{Inspectable};
use crate::astroid::{AstroidMaterial, Astroid};
use crate::base_station::MetalIngot;
use std::fmt;
use std::ops::{AddAssign, SubAssign};

#[derive(Component, Default, Debug, Clone, PartialEq)]
pub struct Capacity {
    pub maximum: f32
}

#[derive(Component, Default, Debug, Clone)]
pub struct Inventory {
    pub items: Vec<InventoryItem>,
    // pub items: HashMap<AstroidMaterial, f32>,
    pub capacity: Capacity
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Amount {
    Weight(f32),
    Quantity(u32)
}

impl AddAssign for Amount {
    fn add_assign(&mut self, rhs: Self) {
        
        match self {
            Amount::Weight(weight) => {
                match rhs {
                    Amount::Weight(w) => *weight += w,
                    _ => {},
                }
            },
            Amount::Quantity(quantity) => {
                match rhs {
                    Amount::Quantity(q) => *quantity += q,
                    _ => {},
                }
            },
        }

    }
}

impl SubAssign for Amount {
    fn sub_assign(&mut self, rhs: Self) {
        
        match self {
            Amount::Weight(weight) => {
                match rhs {
                    Amount::Weight(w) => *weight -= w,
                    _ => {},
                }
            },
            Amount::Quantity(quantity) => {
                match rhs {
                    Amount::Quantity(q) => *quantity -= q,
                    _ => {},
                }
            },
        }

    }
}

#[derive(Debug, Clone, Copy, PartialEq)]

pub enum InventoryItem {
    Material(AstroidMaterial, Amount),
    Ingot(MetalIngot, Amount)
}

impl Default for InventoryItem {
    fn default() -> Self {
        InventoryItem::Material(AstroidMaterial::Iron, Amount::Weight(0.0))
    }
}

impl InventoryItem {
    pub fn amount(&self) -> Amount {
        match self {
            InventoryItem::Material(_, weight) => *weight,
            InventoryItem::Ingot(_, quantity) => *quantity,
        }
    }

    pub fn add_amount(&self, to_add: Amount) {
        match self {
            InventoryItem::Material(_, mut weight) => {
                weight += to_add;
            },
            InventoryItem::Ingot(_, mut quantity) => {
                 quantity += to_add;
            },
        }
    }

    pub fn remove_amount(&self, to_remove: Amount) {
        match self {
            InventoryItem::Material(_, mut weight) => {
                weight -= to_remove;
            },
            InventoryItem::Ingot(_, mut quantity) => {
                quantity -= to_remove;
            },
        }
    }
}

impl Inventory {

    pub fn has_capacity_for(&self, item: InventoryItem) -> bool {
        match item.amount() {
            Amount::Weight(w) => {
                return self.remaining_capacity() >= w;
            },
            Amount::Quantity(_) => {
                // TODO: calculate weight with quantity * item_weight
                return true;
            },
        }
    }

    pub fn remaining_capacity(&self) -> f32 {
        self.capacity.maximum - self.gross_material_weight()
    }

    /// Returns the current gross weight of materials in the inventory
    pub fn gross_material_weight(&self) -> f32 {
        let mut gross_weight = 0.0;

        for item in self.items.iter() {

            match item.amount() {
                Amount::Weight(w) => {
                    gross_weight += w;
                },
                Amount::Quantity(q) => {
                    // TODO: calculate weight with quantity * item_weight
                },
            }

        }

        gross_weight
    }

    pub fn add_to_inventory(&mut self, item_to_add: InventoryItem) -> bool {

        if self.has_capacity_for(item_to_add) {

            if let Some(found) = self.items.iter().find(|item| matches!(item, item_to_add)) {
                found.add_amount(item_to_add.amount())
            } else {
                self.items.push(item_to_add);
            }

        } else {
            println!("NOT ENOUGH CAPACITY FOR: {:?}", item_to_add);
            return false;
        }

        true
    }

    // I'd rather have the inventory be a hashmap, but was struggling with bevy-inspector traits
    // pub fn add_to_inventory(&mut self, material: AstroidMaterial, weight: f32) -> bool {

    //     if weight > self.remaining_capacity() {
    //         println!("NOT ENOUGH SHIP CAPACITY! Remaining Capacity: {}", self.remaining_capacity());
    //         return false;
    //     }

    //     if self.items.contains_key(&material) {
    //         println!("subsequent pickup");
    //         let item = self.items.get_key_value_mut(&material).unwrap();
    //         *item.1 += weight;
    //     } else {
    //         println!("first pickup");
    //         self.items.insert(material, weight);
    //     }

    //     println!("{:?}", self.items);
        
    //     true
    // }

    pub fn remove_from_inventory(&mut self, item_to_remove: InventoryItem) -> bool {

        match item_to_remove {
            InventoryItem::Material(to_find, _) => {
                if let Some(found_item) = self.items.iter().find(|item| matches!(item, InventoryItem::Material(to_find, _))) {
                    found_item.remove_amount(item_to_remove.amount());
                    return true;
                }
            },
            InventoryItem::Ingot(to_find, _) => {
                if let Some(found_item) = self.items.iter().find(|item| matches!(item, InventoryItem::Ingot(to_find, _))) {
                    found_item.remove_amount(item_to_remove.amount());
                    return true;
                }
            },
        }

        false

    }

    // Remove weight from material in inventory, return amount removed. If there's not enough weight to remove, return None
    // pub fn remove_from_inventory(&mut self, material: &AstroidMaterial, weight: f32) -> Option<f32> {

    //     if let Some(current_weight) = self.items.get_mut(&material) {
    //         if *current_weight >= weight {
    //             *current_weight -= weight;

    //             if *current_weight <= 0.0 {
    //                 self.items.remove(&material);
    //             }
            
    //             return Some(weight);
    //         }
    //     }

    //     None

    // }
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
                                        items: Vec::new(),
                                        capacity: Capacity { maximum: 1000.0 }});
        }

        pub fn attach_inventory_to_entity(commands: &mut Commands, inventory: Inventory, entity: Entity) {
            commands.entity(entity).insert(inventory);
        }
}