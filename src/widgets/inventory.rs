use kayak_ui::core::{
    rsx,
    widget, use_state, Handler,
};

use kayak_ui::{core::{VecTracker, constructor, Binding, Bound}, widgets::If};
use kayak_ui::widgets::{Text, Window, Element, Background};
use kayak_ui::core::{
    color::Color,
    render_command::RenderCommand,
    styles::{Corner, Style, StyleProp, Units, LayoutType, Edge},
    EventType, OnEvent, WidgetProps,
    CursorIcon
};

use bevy::prelude::*;

use crate::{HEIGHT, RESOLUTION};
use crate::game_ui::{UIItems};
use crate::inventory::{InventoryItem, Amount};

#[widget]
pub fn UIShipInventory() {
    let ui_items = context.query_world::<Res<Binding<UIItems>>, _, _>(move |ui_items| ui_items.clone());
    context.bind(&ui_items);

    let inventory = ui_items.get().ship_inventory_items;

    let size = Vec2 { x: 200.0, y: 500.0 };
    let ui_ship_inventory_pos = (HEIGHT * RESOLUTION - size.x, HEIGHT - size.y);

    rsx! {
        <Window position={ui_ship_inventory_pos} size={(size.x, size.y)} title={"Ship Inventory".to_string()}>
            <InventoryItems items={inventory} />
        </Window>
    }
}

#[widget]
pub fn UIBaseInventory() {
    let ui_items = context.query_world::<Res<Binding<UIItems>>, _, _>(move |ui_items| ui_items.clone());
    context.bind(&ui_items);

    let inventory = ui_items.get().station_inventory_items;
    
    let size = Vec2 { x: 200.0, y: 500.0 };
    let ui_base_inventory_pos = (0.0, HEIGHT - size.y);

    rsx! {
        <Window position={ui_base_inventory_pos} size={(size.x, size.y)} title={"Station Inventory".to_string()}>
            <InventoryItems items={inventory} />
        </Window>
    }
}

#[derive(WidgetProps, Clone, Debug, Default, PartialEq)]
pub struct InventoryItemsProps {
    pub items: Vec<InventoryItem>
}

#[widget]
pub fn InventoryItems(props: InventoryItemsProps) {
    let InventoryItemsProps { items } = props.clone();

    rsx! {
        <Element>
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
    pub item: InventoryItem
}

#[widget]
pub fn UIInventoryItem(props: UIInventoryItemProps) {

    let UIInventoryItemProps {
        item_id,
        item
    } = props.clone();

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
        },
        InventoryItem::Ingot(ingot, Amount::Quantity(quantity)) => {
            rsx! {
                <Background styles={Some(background_styles)}>
                    <Text content={format!("{:?}", ingot)} size={16.0} />
                    <Text content={format!("x{}", quantity)} size={14.0} />
                </Background>
            }
        },
        InventoryItem::Component(component, Amount::Quantity(quantity)) => {
            rsx! {
                <Background styles={Some(background_styles)}>
                    <Text content={format!("{:?}", component)} size={16.0} />
                    <Text content={format!("x{}", quantity)} size={14.0} />
                </Background>
            }
        },
        _ => {}
    }

    
}