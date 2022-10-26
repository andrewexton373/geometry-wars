use kayak_ui::core::styles::PositionType;
use kayak_ui::core::{rsx, use_state, widget, Handler};

use kayak_ui::core::{
    color::Color,
    render_command::RenderCommand,
    styles::{Corner, Edge, LayoutType, Style, StyleProp, Units},
    CursorIcon, EventType, OnEvent, WidgetProps,
};
use kayak_ui::widgets::{Background, Element, Text, Window};
use kayak_ui::{
    core::{constructor, Binding, Bound, VecTracker},
    widgets::If,
};

use bevy::prelude::*;

use crate::game_ui::UIItems;
use crate::inventory::{Amount, InventoryItem};
use crate::{HEIGHT, RESOLUTION};

#[widget]
pub fn UIShipInventory() {
    let ui_items =
        context.query_world::<Res<Binding<UIItems>>, _, _>(move |ui_items| ui_items.clone());
    context.bind(&ui_items);

    let inventory = ui_items.get().ship_inventory_items;
    let container_styles = Some(Style {
        width: StyleProp::Value(Units::Percentage(100.0)),
        height: StyleProp::Value(Units::Percentage(100.0)),
        position_type: StyleProp::Value(PositionType::SelfDirected),
        // background_color: StyleProp::Value(Color::new(1.0, 0.0, 0.0, 0.8)),
        ..Style::default()
    });

    let ship_inventory_styles = Some(Style {
        top: StyleProp::Value(Units::Stretch(1.0)),
        left: StyleProp::Value(Units::Stretch(1.0)),
        width: StyleProp::Value(Units::Pixels(200.0)),
        height: StyleProp::Value(Units::Auto),
        padding: StyleProp::Value(Edge::all(Units::Pixels(10.0))),
        background_color: StyleProp::Value(Color::new(0.4, 0.4, 0.4, 1.0)),
        ..Default::default()
    });

    rsx! {
        <Element styles={container_styles}>
            <Background styles={ship_inventory_styles}>
                <Text content={"Ship Inventory".to_string()} size={14.0} />
                <InventoryItems items={inventory} />
            </Background>
        </Element>
    }
}

#[widget]
pub fn UIBaseInventory() {
    let ui_items =
        context.query_world::<Res<Binding<UIItems>>, _, _>(move |ui_items| ui_items.clone());
    context.bind(&ui_items);

    let inventory = ui_items.get().station_inventory_items;

    let inventory = ui_items.get().station_inventory_items;
    let container_styles = Some(Style {
        width: StyleProp::Value(Units::Percentage(100.0)),
        height: StyleProp::Value(Units::Percentage(100.0)),
        position_type: StyleProp::Value(PositionType::SelfDirected),
        // background_color: StyleProp::Value(Color::new(1.0, 0.0, 0.0, 0.8)),
        ..Style::default()
    });

    let base_inventory_styles = Some(Style {
        top: StyleProp::Value(Units::Stretch(1.0)),
        left: StyleProp::Value(Units::Stretch(0.0)),
        width: StyleProp::Value(Units::Pixels(200.0)),
        height: StyleProp::Value(Units::Auto),
        padding: StyleProp::Value(Edge::all(Units::Pixels(10.0))),
        background_color: StyleProp::Value(Color::new(0.4, 0.4, 0.4, 1.0)),
        ..Default::default()
    });

    rsx! {
        <Element styles={container_styles}>
            <Background styles={base_inventory_styles}>
                <Text content={"Station Inventory".to_string()} size={14.0} />
                <InventoryItems items={inventory} />
            </Background>
        </Element>
    }
}

#[derive(WidgetProps, Clone, Debug, Default, PartialEq)]
pub struct InventoryItemsProps {
    pub items: Vec<InventoryItem>,
}

#[widget]
pub fn InventoryItems(props: InventoryItemsProps) {
    let InventoryItemsProps { items } = props.clone();

    let styles = Some(Style {
        layout_type: StyleProp::Value(LayoutType::Column),
        background_color: StyleProp::Value(Color::new(1.0, 0.196, 0.215, 1.0)),
        height: StyleProp::Value(Units::Auto),
        // top: StyleProp::Value(Units::Pixels(10.0)),
        // padding: StyleProp::Value(Edge::all(Units::Pixels(5.0))),
        ..Style::default()
    });

    rsx! {
        <Element styles={styles}>
            {VecTracker::from(items.clone().into_iter().enumerate().map(|(index, item)| {
                constructor! {
                    <UIInventoryItem item_id={index} item={item.clone()} />
                }
            }))}
        </Element>
    }
}

#[derive(WidgetProps, Clone, Debug, Default, PartialEq)]
pub struct UIInventoryItemProps {
    pub item_id: usize,
    pub item: InventoryItem,
}

#[widget]
pub fn UIInventoryItem(props: UIInventoryItemProps) {
    let UIInventoryItemProps { item_id, item } = props.clone();

    let background_styles = Style {
        layout_type: StyleProp::Value(LayoutType::Column),
        background_color: StyleProp::Value(Color::new(0.176, 0.196, 0.215, 1.0)),
        height: StyleProp::Value(Units::Auto),
        top: StyleProp::Value(Units::Pixels(10.0)),
        padding: StyleProp::Value(Edge::all(Units::Pixels(5.0))),
        ..Style::default()
    };

    match item {
        InventoryItem::Material(material, Amount::Weight(weight)) => {
            rsx! {
                <Background styles={Some(background_styles)}>
                    <Text content={format!("Material: {:?}", material)} size={16.0} />
                    <Text content={format!("Net Mass: {:.2} Kgs", weight)} size={14.0} />
                </Background>
            }
        }
        InventoryItem::Ingot(ingot, Amount::Quantity(quantity)) => {
            rsx! {
                <Background styles={Some(background_styles)}>
                    <Text content={format!("{:?}", ingot)} size={16.0} />
                    <Text content={format!("x{}", quantity)} size={14.0} />
                </Background>
            }
        }
        InventoryItem::Component(component, Amount::Quantity(quantity)) => {
            rsx! {
                <Background styles={Some(background_styles)}>
                    <Text content={format!("{:?}", component)} size={16.0} />
                    <Text content={format!("x{}", quantity)} size={14.0} />
                </Background>
            }
        }
        _ => {}
    }
}
