use bevy_rapier2d::prelude::Velocity;
// use kayak_ui::core::{Binding, MutableBound};

// use kayak_ui::bevy::{BevyContext, BevyKayakUIPlugin, FontMapping, UICameraBundle};
// use kayak_ui::core::{bind, render};
// use kayak_ui::widgets::App as KayakApp;

use bevy::{prelude::*, utils::HashSet};

// use crate::upgrades::UpgradesComponent;
// use crate::widgets::ship_information::{ShipInformation, UIShipInformationView};
// use crate::widgets::station_menu::{UIStationMenu, UpgradeType};
use crate::{
    base_station::BaseStation,
    factory::Factory,
    inventory::{Inventory, InventoryItem},
    player::{Player, ShipInformation},
    refinery::Refinery, upgrades::UpgradeType,
    // widgets::{
    //     context_clue::UIContextClueView,
    //     inventory::{UIShipInventory},
    // },
};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct UIItems {
    pub ship_inventory_items: Vec<InventoryItem>,
    pub station_inventory_items: Vec<InventoryItem>,
    pub refinery: Refinery,
    pub factory: Factory,
    pub remaining_refinery_time: f32,
    pub context_clues: HashSet<ContextClue>,
    pub ship_info: ShipInformation,
    pub upgrades: Vec<UpgradeType>,
}

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ContextClue {
    #[default]
    NearBaseStation,
    CargoBayFull,
    ShipFuelEmpty,
    ShipInventoryEmpty,
}

impl ContextClue {
    pub fn text(&self) -> String {
        match *self {
            ContextClue::NearBaseStation => "Near Base Station, Deposit Collected Ore with SPACE.",
            ContextClue::CargoBayFull => {
                "The Player's Ship Cargo Bay is Full. Deposit Ore at Base Station."
            }
            ContextClue::ShipFuelEmpty => "The Player's Ship Fuel Tank is Empty!",
            ContextClue::ShipInventoryEmpty => "The Player's Ship Inventory is Empty!",
            _ => "Missing Context Clue Note.",
        }
        .to_string()
    }
}

#[derive(Resource)]
pub struct ContextClues(pub HashSet<ContextClue>);

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app
            // .add_plugin(BevyKayakUIPlugin)
            // .add_startup_system(Self::setup_game_ui)
            .insert_resource(ContextClues(HashSet::new()));
            // .add_system(Self::update_ui_data);
    }
}

// impl GameUIPlugin {
//     fn setup_game_ui(
//         mut commands: Commands,
//         mut font_mapping: ResMut<FontMapping>,
//         asset_server: Res<AssetServer>,
//     ) {
//         commands
//             .spawn_bundle(UICameraBundle::new())
//             .insert(Name::new("UICamera"));

//         font_mapping.set_default(asset_server.load("roboto.kayak_font"));
//         commands.insert_resource(bind(UIItems::default()));

//         let context = BevyContext::new(|context| {
//             render! {
//                 <KayakApp>
//                     <UIShipInventory />
//                     // <UIBaseInventory />
//                     <UIContextClueView />
//                     // <UICraftingTabsView />
//                     <UIShipInformationView />
//                     <UIStationMenu />

//                 </KayakApp>
//             }
//         });

//         commands.insert_resource(context);
//     }

//     fn update_ui_data(
//         player_query: Query<
//             (&UpgradesComponent, &Inventory, &Velocity),
//             (With<Player>, Without<BaseStation>),
//         >,
//         base_station_query: Query<
//             (&Inventory, &Refinery, &Factory),
//             (With<BaseStation>, Without<Player>),
//         >,
//         context_clues_res: Res<ContextClues>,
//         ui_items: Res<Binding<UIItems>>,
//     ) {
//         let (upgrades, ship_inventory, ship_velocity) = player_query.single();
//         let (station_inventory, station_refinery, station_factory) = base_station_query.single();

//         // update ui by updating binding object
//         ui_items.set(UIItems {
//             ship_inventory_items: ship_inventory.items.clone(),
//             station_inventory_items: station_inventory.items.clone(),
//             refinery: station_refinery.clone(),
//             factory: station_factory.clone(),
//             remaining_refinery_time: 0.0,
//             context_clues: context_clues_res.into_inner().0.clone(),
//             ship_info: ShipInformation {
//                 net_weight: ship_inventory.gross_material_weight(),
//                 speed: ship_velocity.linvel.length(),
//                 direction: ship_velocity.linvel.angle_between(Vec2::Y), // FIXME: In Rads for now, also wrong
//             },
//             upgrades: upgrades.upgrades.clone(),
//         });
//     }
// }
