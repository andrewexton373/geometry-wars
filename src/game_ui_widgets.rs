use kayak_ui::core::{
    rsx,
    widget, use_state,
};

use kayak_ui::{core::{VecTracker, constructor, Binding, Bound}, widgets::If};
use kayak_ui::widgets::{Text, Window, Element, Button, Background};
use kayak_ui::core::{
    color::Color,
    render_command::RenderCommand,
    styles::{Corner, Style, StyleProp, Units},
    EventType, OnEvent, WidgetProps,
    CursorIcon
};

use bevy::prelude::*;

use crate::game_ui::UIItems;


#[widget]
pub fn UIShipInventory() {
    let ui_items = context.query_world::<Res<Binding<UIItems>>, _, _>(move |ui_items| ui_items.clone());
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
    let ui_items = context.query_world::<Res<Binding<UIItems>>, _, _>(move |ui_items| ui_items.clone());
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


// TODO: set this up
#[widget]
pub fn UIRefineryView() {
    let ui_items = context.query_world::<Res<Binding<UIItems>>, _, _>(move |ui_items| ui_items.clone());
    context.bind(&ui_items);

    // let inventory = ui_items.get().station_inventory_items;
    let refinery = ui_items.get().refinery;
    
    rsx! {
        <Window position={(0.0, 450.0)} size={(200.0, 300.0)} title={"Station Refinery".to_string()}>

        <If condition={refinery.currently_processing.is_some()}>
            <Text content={format!("Currently Processing: {:?}\n Into BLANK Ingot", refinery.currently_processing).to_string()} size={16.0} />
        </If>

        <If condition={refinery.currently_processing.is_none()}>
             <Element>
                {VecTracker::from(refinery.recipes.into_iter().map(|recipe| {
                    constructor! {
                        <Background>
                            <Text content={format!("Craftable Item: {:?}\n From: {:?}", recipe.item_created, recipe.items_required)} size={16.0} />
                            <Text content={"+".to_string()} size={20.0} />
                        </Background>
                    }
                }))}
            </Element>
        </If>

        </Window>
    }
}