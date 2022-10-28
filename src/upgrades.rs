use crate::astroid::Collectible;
use crate::base_station::{BaseStation, CanDeposit};
use crate::battery::Battery;
use crate::crosshair::Crosshair;
use crate::game_ui::{ContextClue, ContextClues};
use crate::health::Health;
use crate::inventory::{Capacity, Inventory, InventoryPlugin};
use crate::particles::PlayerShipTrailParticles;
use crate::player::Player;
use crate::player_stats_bar::PlayerStatsBarPlugin;
use crate::projectile::ProjectilePlugin;
use crate::widgets::station_menu::{UpgradeEvent, UpgradeLevel, UpgradeType};
use crate::{GameCamera, PIXELS_PER_METER};
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy_hanabi::ParticleEffect;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use bevy_prototype_lyon::prelude as lyon;
use bevy_rapier2d::prelude::*;
use std::f32::consts::PI;
use strum::IntoEnumIterator;


pub trait Upgradeable {
    fn set_upgrade_level(&mut self, upgrade_level: UpgradeLevel);
    fn upgrade_effect(&self) -> f32;
}

#[derive(Component, Default, Inspectable)]
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
            // TODO if the ship inventory has the required components, perform the upgrade.
            // if to_upgrade.requirements()

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