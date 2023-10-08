use crate::factory::UpgradeComponent;
use crate::inventory::{Amount, Inventory, InventoryItem};
use crate::player::Player;
use bevy::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, FromRepr};

pub trait Upgradeable {
    fn set_upgrade_level(&mut self, upgrade_level: UpgradeLevel);
    fn upgrade_effect(&self) -> f32;
}

#[derive(Component, Default)]
pub struct UpgradesComponent {
    pub upgrades: Vec<UpgradeType>,
}

impl UpgradesComponent {
    pub fn new() -> Self {
        let mut upgrades = vec![];
        for upgrade_type in UpgradeType::iter() {
            if upgrade_type != UpgradeType::None {
                upgrades.push(upgrade_type);
            }
        }

        Self { upgrades: upgrades }
    }

    pub fn upgrade(
        &mut self,
        upgrade_type: UpgradeType,
        player: &mut Player,
        ship_inventory: &mut Inventory,
    ) {
        if let Some(to_upgrade) = self
            .upgrades
            .iter_mut()
            .find(|upgrade| **upgrade == upgrade_type)
        {
            let upgrade_requirements = to_upgrade.next().requirements().unwrap().requirements;

            if ship_inventory.has_items(upgrade_requirements.clone()) {
                *to_upgrade = match to_upgrade {
                    UpgradeType::None => UpgradeType::None,
                    UpgradeType::Health(level) => {
                        let next = level.next().unwrap_or_else(|| UpgradeLevel::MaxLevel);
                        player.health.set_upgrade_level(next.clone());
                        ship_inventory.remove_all_from_inventory(upgrade_requirements.clone());
                        UpgradeType::Health(next)
                    }
                    UpgradeType::ShipCargoBay(level) => {
                        let next = level.next().unwrap_or_else(|| UpgradeLevel::MaxLevel);
                        player.battery.set_upgrade_level(next.clone());
                        ship_inventory.remove_all_from_inventory(upgrade_requirements.clone());
                        UpgradeType::ShipCargoBay(next)
                    }
                }
            } else {
                println!("DON'T HAVE MATERIALS REQUIRED FOR UPGRADE!");
            }
        }

        println!("{:?}", self.upgrades);
    }
}

#[derive(FromRepr, EnumIter, Debug, Clone, Copy, Default, PartialEq)]
#[repr(u8)]
pub enum UpgradeLevel {
    #[default]
    Level0 = 0,
    Level1 = 1,
    Level2 = 2,
    Level3 = 3,
    Level4 = 4,
    MaxLevel = 5,
}

impl UpgradeLevel {
    pub fn as_u8(&self) -> u8 {
        match self {
            UpgradeLevel::Level0 => 0,
            UpgradeLevel::Level1 => 1,
            UpgradeLevel::Level2 => 2,
            UpgradeLevel::Level3 => 3,
            UpgradeLevel::Level4 => 4,
            UpgradeLevel::MaxLevel => 5,
        }
    }

    pub fn next(&self) -> Option<UpgradeLevel> {
        let current_lvl = self.as_u8();
        UpgradeLevel::from_repr(current_lvl + 1)
    }
}

#[derive(Default, EnumIter, Clone, Copy, Debug, PartialEq)]
pub enum UpgradeType {
    #[default]
    None,
    Health(UpgradeLevel),
    ShipCargoBay(UpgradeLevel),
}

impl ToString for UpgradeType {
    fn to_string(&self) -> String {
        match self {
            UpgradeType::Health(_) => "Health",
            UpgradeType::ShipCargoBay(_) => "Ship Cargo Bay",
            UpgradeType::None => "NONE UPGRADE.",
        }
        .to_string()
    }
}

impl UpgradeType {
    pub fn requirements(&self) -> Option<UpgradeRequirements> {
        let requirements;

        match self {
            UpgradeType::None => {
                return None;
            }
            UpgradeType::Health(level) => {
                requirements = match level {
                    UpgradeLevel::Level0 => vec![],
                    UpgradeLevel::Level1 => vec![
                        InventoryItem::Component(UpgradeComponent::Cog, Amount::Quantity(1)),
                        InventoryItem::Component(UpgradeComponent::IronPlate, Amount::Quantity(2)),
                    ],
                    UpgradeLevel::Level2 => vec![
                        InventoryItem::Component(UpgradeComponent::Cog, Amount::Quantity(2)),
                        InventoryItem::Component(UpgradeComponent::IronPlate, Amount::Quantity(3)),
                    ],
                    UpgradeLevel::Level3 => vec![
                        InventoryItem::Component(UpgradeComponent::Cog, Amount::Quantity(1)),
                        InventoryItem::Component(UpgradeComponent::IronPlate, Amount::Quantity(2)),
                        InventoryItem::Component(
                            UpgradeComponent::SilverConduit,
                            Amount::Quantity(1),
                        ),
                    ],
                    UpgradeLevel::Level4 => vec![
                        InventoryItem::Component(UpgradeComponent::Cog, Amount::Quantity(3)),
                        InventoryItem::Component(UpgradeComponent::IronPlate, Amount::Quantity(5)),
                        InventoryItem::Component(
                            UpgradeComponent::SilverConduit,
                            Amount::Quantity(3),
                        ),
                        InventoryItem::Component(UpgradeComponent::GoldLeaf, Amount::Quantity(1)),
                    ],
                    UpgradeLevel::MaxLevel => vec![
                        InventoryItem::Component(UpgradeComponent::Cog, Amount::Quantity(10)),
                        InventoryItem::Component(UpgradeComponent::IronPlate, Amount::Quantity(5)),
                        InventoryItem::Component(
                            UpgradeComponent::SilverConduit,
                            Amount::Quantity(5),
                        ),
                        InventoryItem::Component(UpgradeComponent::GoldLeaf, Amount::Quantity(3)),
                    ],
                }
            }
            UpgradeType::ShipCargoBay(level) => {
                requirements = match level {
                    UpgradeLevel::Level0 => vec![],
                    UpgradeLevel::Level1 => vec![
                        InventoryItem::Component(UpgradeComponent::Cog, Amount::Quantity(2)),
                        InventoryItem::Component(UpgradeComponent::IronPlate, Amount::Quantity(3)),
                    ],
                    UpgradeLevel::Level2 => todo!(),
                    UpgradeLevel::Level3 => todo!(),
                    UpgradeLevel::Level4 => todo!(),
                    UpgradeLevel::MaxLevel => todo!(),
                }
            }
        }

        return Some(UpgradeRequirements { requirements });
    }

    pub fn next(&self) -> Self {
        match self {
            UpgradeType::None => {
                return self.clone();
            }
            UpgradeType::Health(level) => {
                if let Some(next_level) = level.next() {
                    UpgradeType::Health(next_level)
                } else {
                    self.clone()
                }
            }
            UpgradeType::ShipCargoBay(level) => {
                if let Some(next_level) = level.next() {
                    UpgradeType::ShipCargoBay(next_level)
                } else {
                    self.clone()
                }
            }
        }
    }
}

pub struct UpgradeEvent(pub UpgradeType);

#[derive(Debug, Clone, PartialEq)]
pub struct UpgradeRequirements {
    pub requirements: Vec<InventoryItem>,
}
