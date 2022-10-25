use crate::astroid::AstroidMaterial;
use crate::factory::UpgradeComponent;
use crate::refinery::MetalIngot;
use bevy::prelude::*;
use std::fmt;
use std::ops::{AddAssign, SubAssign};

#[derive(Component, Default, Debug, Clone, PartialEq)]
pub struct Capacity {
    pub maximum: f32,
}

#[derive(Component, Default, Debug, Clone)]
pub struct Inventory {
    pub items: Vec<InventoryItem>,
    pub capacity: Capacity,
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum Amount {
    Weight(f32),
    Quantity(u32),
}

impl fmt::Debug for Amount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Weight(arg0) => {
                write!(f, "{} Kgs", arg0)
            }
            Self::Quantity(arg0) => {
                write!(f, "x{}", arg0)
            }
        }
    }
}

impl AddAssign for Amount {
    fn add_assign(&mut self, rhs: Self) {
        match self {
            Amount::Weight(weight) => match rhs {
                Amount::Weight(w) => *weight += w,
                _ => {}
            },
            Amount::Quantity(quantity) => match rhs {
                Amount::Quantity(q) => *quantity += q,
                _ => {}
            },
        }
    }
}

impl SubAssign for Amount {
    fn sub_assign(&mut self, rhs: Self) {
        match self {
            Amount::Weight(weight) => match rhs {
                Amount::Weight(w) => *weight -= w,
                _ => {}
            },
            Amount::Quantity(quantity) => match rhs {
                Amount::Quantity(q) => *quantity -= q,
                _ => {}
            },
        }
    }
}

#[derive(Clone, Copy, PartialEq)]

pub enum InventoryItem {
    Material(AstroidMaterial, Amount),
    Ingot(MetalIngot, Amount),
    Component(UpgradeComponent, Amount),
}

impl Default for InventoryItem {
    fn default() -> Self {
        InventoryItem::Material(AstroidMaterial::Iron, Amount::Weight(0.0))
    }
}

impl fmt::Debug for InventoryItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Material(arg0, arg1) => {
                write!(f, "{:?}: {:?}", arg0, arg1)
            }
            Self::Ingot(arg0, arg1) => {
                write!(f, "{:?}: {:?}", arg0, arg1)
            }
            Self::Component(arg0, arg1) => {
                write!(f, "{:?}: {:?}", arg0, arg1)
            }
        }
    }
}

impl InventoryItem {
    pub fn amount(&self) -> Amount {
        match self {
            InventoryItem::Material(_, weight) => *weight,
            InventoryItem::Ingot(_, quantity) => *quantity,
            InventoryItem::Component(_, quantity) => *quantity,
        }
    }

    pub fn add_amount(&mut self, to_add: Amount) {
        match self {
            InventoryItem::Material(_, ref mut weight) => {
                *weight += to_add;
            }
            InventoryItem::Ingot(_, ref mut quantity) => {
                *quantity += to_add;
            }
            InventoryItem::Component(_, ref mut quantity) => {
                *quantity += to_add;
            }
        }
    }

    pub fn remove_amount(&mut self, to_remove: Amount) {
        match self {
            InventoryItem::Material(_, ref mut weight) => {
                *weight -= to_remove;
            }
            InventoryItem::Ingot(_, ref mut quantity) => {
                *quantity -= to_remove;
            }
            InventoryItem::Component(_, ref mut quantity) => {
                *quantity -= to_remove;
            }
        }
    }
}

impl Inventory {
    pub fn has_capacity_for(&self, item: InventoryItem) -> bool {
        match item.amount() {
            Amount::Weight(w) => {
                return self.remaining_capacity() >= w;
            }
            Amount::Quantity(_) => {
                // TODO: calculate weight with quantity * item_weight
                return true;
            }
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
                }
                Amount::Quantity(q) => {
                    // TODO: calculate weight with quantity * item_weight
                }
            }
        }

        gross_weight
    }

    pub fn add_to_inventory(&mut self, item_to_add: InventoryItem) -> bool {
        if self.has_capacity_for(item_to_add) {
            match item_to_add {
                InventoryItem::Material(material, _weight) => {
                    if let Some(found) = self.items.iter_mut().find_map(|item| match item {
                        InventoryItem::Material(m, _) if *m == material => Some(item),
                        _ => None,
                    }) {
                        found.add_amount(item_to_add.amount());
                    } else {
                        self.items.push(item_to_add);
                    }
                }
                InventoryItem::Ingot(ingot, _quantity) => {
                    if let Some(found) = self.items.iter_mut().find_map(|item| match item {
                        InventoryItem::Ingot(i, _) if *i == ingot => Some(item),
                        _ => None,
                    }) {
                        found.add_amount(item_to_add.amount())
                    } else {
                        self.items.push(item_to_add);
                    }
                }
                InventoryItem::Component(component, _quantity) => {
                    if let Some(found) = self.items.iter_mut().find_map(|item| match item {
                        InventoryItem::Component(i, _) if *i == component => Some(item),
                        _ => None,
                    }) {
                        found.add_amount(item_to_add.amount())
                    } else {
                        self.items.push(item_to_add);
                    }
                }
            }
        } else {
            println!("NOT ENOUGH CAPACITY FOR: {:?}", item_to_add);
            return false;
        }

        true
    }

    pub fn remove_from_inventory(&mut self, item_to_remove: InventoryItem) -> bool {
        match item_to_remove {
            InventoryItem::Material(to_find, _) => {
                if let Some((index, found_item)) =
                    self.items
                        .iter_mut()
                        .enumerate()
                        .find_map(|(index, item)| match item {
                            InventoryItem::Material(m, _) if *m == to_find => Some((index, item)),
                            _ => None,
                        })
                {
                    if found_item.amount() >= item_to_remove.amount() {
                        found_item.remove_amount(item_to_remove.amount());
                        if found_item.amount() == Amount::Weight(0.0) {
                            self.items.remove(index);
                        }
                        return true;
                    } else {
                        return false;
                    }
                }
            }
            InventoryItem::Ingot(to_find, _) => {
                if let Some((index, found_item)) =
                    self.items
                        .iter_mut()
                        .enumerate()
                        .find_map(|(index, item)| match item {
                            InventoryItem::Ingot(i, _) if *i == to_find => Some((index, item)),
                            _ => None,
                        })
                {
                    if found_item.amount() >= item_to_remove.amount() {
                        found_item.remove_amount(item_to_remove.amount());

                        if found_item.amount() == Amount::Quantity(0) {
                            self.items.remove(index);
                        }
                        return true;
                    } else {
                        return false;
                    }
                }
            }
            InventoryItem::Component(to_find, _) => {
                if let Some((index, found_item)) =
                    self.items
                        .iter_mut()
                        .enumerate()
                        .find_map(|(index, item)| match item {
                            InventoryItem::Component(i, _) if *i == to_find => Some((index, item)),
                            _ => None,
                        })
                {
                    if found_item.amount() >= item_to_remove.amount() {
                        if found_item.amount() == Amount::Quantity(0) {
                            self.items.remove(index);
                        }
                        return true;
                    } else {
                        return false;
                    }
                }
            }
        }

        false
    }
}

// #[derive(Component, Default, Debug, Inspectable, Copy, Clone, PartialEq, PartialOrd)]
// pub struct ItemAndWeight {
//     pub item: AstroidMaterial,
//     pub weight: f32
// }

// impl fmt::Display for ItemAndWeight {
//     fn fmt(&self, f: &mut  fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "Item: {:?} | Weight: {}", &self.item, &self.weight)
//     }
// }

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, _app: &mut App) {}
}

impl InventoryPlugin {
    pub fn attach_inventory_to_entity(
        commands: &mut Commands,
        mut inventory: Inventory,
        entity: Entity,
    ) {
        // TODO: REMOVE ONLY FOR TESTING.
        inventory.add_to_inventory(InventoryItem::Ingot(
            MetalIngot::IronIngot,
            Amount::Quantity(2),
        ));
        commands.entity(entity).insert(inventory);
    }
}
