use bevy::utils::HashMap;
use bevy::{prelude::*};
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::prelude as lyon;
use bevy::render::camera::RenderTarget;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use std::f32::consts::PI;
use crate::{PIXELS_PER_METER, GameCamera};
use crate::astroid::{Collectible};
use crate::healthbar::Health;
use crate::projectile::{ProjectilePlugin};
use crate::crosshair::Crosshair;
use crate::astroid::AstroidMaterial;
use std::fmt;


const INVENTORY_SIZE: usize = 20;

#[derive(Component, Default, Debug, Clone, PartialEq)]
pub struct Inventory {
    // pub items: HashMap<AstroidMaterial, f32>,
    pub items: [Option<ItemAndWeight>; 20]
}

impl Inventory {
    // I'd rather have the inventory be a hashmap, but was struggling with bevy-inspector traits
    pub fn add_to_inventory(&mut self, material: AstroidMaterial, weight: f32) {
        let mut current_inventory = self.items;
        let mut temp_hash_map = HashMap::new();

        // Fill temp hash map with current inventory items
        for item in current_inventory.into_iter() {
            if item.is_some() {
                temp_hash_map.insert(item.unwrap().item, item.unwrap().weight);
            }
        }

        // If hasmap already contains the material, add weight to existing material's weight
        if let Some(value) = temp_hash_map.get_mut(&material) {
            *value = *value + weight;
        
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

    }
}

impl InventoryPlugin {

        

}