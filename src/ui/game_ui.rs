use bevy::{prelude::*, utils::HashSet};
use bevy_egui::{
    egui::{self, Align2},
    EguiContexts, EguiPlugin,
};
use bevy_xpbd_2d::prelude::*;

use crate::hexbase::{BuildingType, PlayerHoveringBuilding};
use crate::upgrades::{UpgradeEvent, UpgradesComponent};
use crate::{
    asteroid::components::Asteroid,
    events::{BuildHexBuildingEvent, CraftEvent},
};
use crate::{
    factory::Factory,
    inventory::{Inventory, InventoryItem},
    player::components::Player,
    refinery::{Refinery, SmeltEvent},
    space_station::components::SpaceStation,
    upgrades::UpgradeType,
    GameCamera,
};

// #[derive(Default, Debug, Clone, PartialEq)]
// pub struct UIItems {
//     pub ship_inventory_items: Vec<InventoryItem>,
//     pub station_inventory_items: Vec<InventoryItem>,
//     pub refinery: Refinery,
//     pub factory: Factory,
//     pub remaining_refinery_time: f32,
//     pub context_clues: HashSet<ContextClue>,
//     pub upgrades: Vec<UpgradeType>,
// }

#[derive(Hash, Clone)]
struct ItemType {
    name: String,
}


impl GameUIPlugin {
    
}
