use kayak_ui::{core::{Binding, MutableBound}};

use kayak_ui::bevy::{BevyContext, BevyKayakUIPlugin, FontMapping, UICameraBundle};
use kayak_ui::core::{
    render,
    bind
};
use kayak_ui::widgets::{App as KayakApp};


use bevy::prelude::*;

use crate::{game_ui_widgets::{UIShipInventory, UIBaseInventory}, inventory::{Inventory, INVENTORY_SIZE, ItemAndWeight}, player::Player, base_station::{BaseStation, CanDeposit}};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct UIItems {
    pub ship_inventory_items: [Option<ItemAndWeight>; INVENTORY_SIZE],
    pub station_inventory_items: [Option<ItemAndWeight>; INVENTORY_SIZE],

    pub can_deposit: bool
}



pub struct GameUIPlugin;


impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugin(BevyKayakUIPlugin)
            .add_startup_system(Self::setup_game_ui)
            .add_system(Self::update_ui_data);
            // .add_system_set(SystemSet::on_enter(GameState::Main).with_system(setup_game_ui))
            // .add_event::<UIEvent>();
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
                </KayakApp>
            }
        });

        commands.insert_resource(context);
    }

    fn update_ui_data(
        player_inventory_query: Query<&Inventory, (With<Player>, Without<BaseStation>)>,
        base_station_inventory_query: Query<&Inventory, (With<BaseStation>, Without<Player>)>,
        can_deposit_res: Res<CanDeposit>,
        ui_items: Res<Binding<UIItems>>,
    ) {
        let ship_inventory = player_inventory_query.single();
        let station_inventory = base_station_inventory_query.single();
    
        // update ui by updating binding object
        ui_items.set(UIItems {
            ship_inventory_items: ship_inventory.items,
            station_inventory_items: station_inventory.items,
            can_deposit: can_deposit_res.0
        });    
    }

}

