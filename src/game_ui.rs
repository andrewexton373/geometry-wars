use kayak_ui::{core::{Binding, MutableBound}};

use kayak_ui::bevy::{BevyContext, BevyKayakUIPlugin, FontMapping, UICameraBundle};
use kayak_ui::core::{
    render,
    bind
};
use kayak_ui::widgets::{App as KayakApp};


use bevy::{prelude::*};

use crate::{inventory::{Inventory, InventoryItem}, player::Player, base_station::{BaseStation}, refinery::Refinery, widgets::{context_clue::UIContextClueView, inventory::{UIShipInventory, UIBaseInventory}, crafting::UICraftingTabsView}, factory::Factory};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct UIItems {
    pub ship_inventory_items: Vec<InventoryItem>,
    pub station_inventory_items: Vec<InventoryItem>,
    pub refinery: Refinery,
    pub factory: Factory,
    pub remaining_refinery_time: f32,
    pub context_clue: Option<ContextClue>
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum ContextClue {
    #[default]
    NearBaseStation
}

impl ContextClue {
    pub fn text(&self) -> String {
        match *self {
            ContextClue::NearBaseStation => "Near Base Station, Deposit Collected Ore with SPACE.",
        }.to_string()
    }
}

pub struct Clue(pub Option<ContextClue>);

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugin(BevyKayakUIPlugin)
            .add_startup_system(Self::setup_game_ui)
            .insert_resource(Clue(None))
            .add_system(Self::update_ui_data);
    }
}

impl GameUIPlugin {

    fn setup_game_ui(
        mut commands: Commands,
        mut font_mapping: ResMut<FontMapping>,
        asset_server: Res<AssetServer>,
    ) {
        commands.spawn_bundle(UICameraBundle::new())
        .insert(Name::new("UICamera"));

        font_mapping.set_default(asset_server.load("roboto.kayak_font"));
        commands.insert_resource(bind(UIItems::default()));

        let context = BevyContext::new(|context| {
            render! {
                <KayakApp>
                    <UIShipInventory />
                    <UIBaseInventory />
                    <UIContextClueView />
                    <UICraftingTabsView />
                </KayakApp>
            }
        });

        commands.insert_resource(context);
    }

    fn update_ui_data(
        player_inventory_query: Query<&Inventory, (With<Player>, Without<BaseStation>)>,
        base_station_query: Query<(&Inventory, &Refinery, &Factory), (With<BaseStation>, Without<Player>)>,
        context_clue_res: Res<Clue>,
        ui_items: Res<Binding<UIItems>>,
    ) {
        let ship_inventory = player_inventory_query.single();
        let (station_inventory, station_refinery, station_factory) = base_station_query.single();

        let mut clue = None;
        if let Clue(Some(context_clue)) = context_clue_res.into_inner() {
            clue = Some(context_clue.clone());
        }
    
        // update ui by updating binding object
        ui_items.set(UIItems {
            ship_inventory_items: ship_inventory.items.clone(),
            station_inventory_items: station_inventory.items.clone(),
            refinery: station_refinery.clone(),
            factory: station_factory.clone(),
            remaining_refinery_time: 0.0,
            context_clue: clue
        });
        
    }

}


