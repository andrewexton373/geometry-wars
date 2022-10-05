use kayak_ui::core::{
    rsx,
    styles::{Style as KayakStyle, StyleProp, Units},
    widget,
};

use kayak_ui::{core::{VecTracker, constructor, Binding, Bound}, widgets::If};
use kayak_ui::widgets::{Text, Window, Element};

use bevy::prelude::*;

use crate::game_ui::UIItems;


#[widget]
pub fn UIShipInventory() {
    let ui_items = context.query_world::<Res<Binding<UIItems>>, _, _>(move |inventory| inventory.clone());
    context.bind(&ui_items);

    let inventory = ui_items.get().ship_inventory_items;
    let can_deposit = ui_items.get().can_deposit;
    
    rsx! {
        <Window position={(900.0, 450.0)} size={(200.0, 300.0)} title={"Ship Inventory".to_string()}>

            <If condition={can_deposit}>
                <Text content={"Press SPACE to deposit ore.".to_string()} size={16.0} />
            </If>

            <Element>
                {VecTracker::from(inventory.iter().filter(|item| item.is_some()).map(|item| {
                    constructor! {
                        <Text content={format!("Material: {:?} \n| Net Weight: {}kgs", item.unwrap().item.clone(), item.unwrap().weight)} size={16.0} />
                    }
                }))}
            </Element>
        </Window>
    }
}


#[widget]
pub fn UIBaseInventory() {
    let ui_items = context.query_world::<Res<Binding<UIItems>>, _, _>(move |inventory| inventory.clone());
    context.bind(&ui_items);

    let inventory = ui_items.get().station_inventory_items;
    
    rsx! {
        <Window position={(1100.0, 450.0)} size={(200.0, 300.0)} title={"Station Inventory".to_string()}>

            <Element>
                {VecTracker::from(inventory.iter().filter(|item| item.is_some()).map(|item| {
                    constructor! {
                        <Text content={format!("Material: {:?} \n| Net Weight: {}kgs", item.unwrap().item.clone(), item.unwrap().weight)} size={16.0} />
                    }
                }))}
            </Element>
        </Window>
    }
}