use bevy::utils::HashMap;
use kayak_ui::core::{
    rsx,
    widget, use_state, Handler,
};

use kayak_ui::{core::{VecTracker, constructor, Binding, Bound}, widgets::If};
use kayak_ui::widgets::{Text, Window, Element, Button, Background};
use kayak_ui::core::{
    color::Color,
    render_command::RenderCommand,
    styles::{Corner, Style, StyleProp, Units, LayoutType, Edge},
    EventType, OnEvent, WidgetProps,
    CursorIcon
};

use bevy::prelude::*;

use crate::astroid::AstroidMaterial;
use crate::base_station::{RefineryRecipe, MetalIngot};
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

    let (color, set_color, ..) = use_state!(Color::new(0.0781, 0.0898, 0.101, 1.0));
    
    let background_styles = Some(Style {
        border_radius: StyleProp::Value(Corner::all(5.0)),
        background_color: StyleProp::Value(color),
        cursor: CursorIcon::Hand.into(),
        padding_left: StyleProp::Value(Units::Pixels(9.0)),
        padding_bottom: StyleProp::Value(Units::Pixels(6.0)),
        ..Style::default()
    });

    let on_event = OnEvent::new(move |_, event| match event.event_type {
        EventType::MouseIn(..) => {
            set_color(Color::new(0.0791, 0.0998, 0.201, 1.0));
        }
        EventType::MouseOut(..) => {
            set_color(Color::new(0.0781, 0.0898, 0.101, 1.0));
        }
        _ => {}
    });


    let ui_items = context.query_world::<Res<Binding<UIItems>>, _, _>(move |ui_items| ui_items.clone());
    context.bind(&ui_items);

    let refinery = ui_items.get().refinery;

    let handle_create = Handler::new(move |refineable_id: usize| {
        println!("CRAFT REFINEABLE! {}", refineable_id);
    });
    
    rsx! {
        <Window position={(0.0, 450.0)} size={(400.0, 300.0)} title={"Station Refinery".to_string()}>

        // <If condition={refinery.currently_processing.is_some()}>
        //     <Text content={format!("Currently Processing: {:?}\n Into BLANK Ingot", refinery.currently_processing.clone())} size={16.0} />
        // </If>

        // <If condition={refinery.currently_processing.is_none()}>

            <Refineables refineables={refinery.recipes.clone().to_vec()} on_create={handle_create} />

        // </If>

        </Window>
    }
}

#[derive(WidgetProps, Clone, Debug, Default, PartialEq)]
pub struct RefineablesProps {
    refineables: Vec<RefineryRecipe>,
    pub on_create: Handler<usize>,
}

#[widget]
pub fn Refineables(props: RefineablesProps) {
    let RefineablesProps { refineables, on_create } = props.clone();

    rsx! {
    <Element>
        {VecTracker::from(refineables.clone().into_iter().enumerate().map(|(index, recipe)| {
            constructor! {
                <Refineable refineable_id={index} ore_required={recipe.items_required.clone()} ingot_produced={recipe.item_created.clone()} on_create={on_create.clone()}/>
            }
        }))}

    </Element>

    }
}

#[derive(WidgetProps, Clone, Debug, Default, PartialEq)]
pub struct RefineableProps {
    pub refineable_id: usize,
    pub ore_required: HashMap<AstroidMaterial, f32>,
    pub ingot_produced: MetalIngot,
    pub on_create: Handler<usize>,
}

#[widget]
pub fn Refineable(props: RefineableProps) {
    let RefineableProps {
        refineable_id,
        ore_required,
        ingot_produced,
        on_create,
    } = props.clone();

    let background_styles = Style {
        layout_type: StyleProp::Value(LayoutType::Row),
        background_color: StyleProp::Value(Color::new(0.176, 0.196, 0.215, 1.0)),
        height: StyleProp::Value(Units::Auto),
        top: StyleProp::Value(Units::Pixels(10.0)),
        padding: StyleProp::Value(Edge::all(Units::Pixels(5.0))),
        ..Style::default()
    };

    let on_create = on_create.clone();
    let on_event = OnEvent::new(move |_, event| match event.event_type {
        EventType::Click(..) => {
            println!("SMELT BUTTON CLICKED!");
            on_create.call(refineable_id);
        }
        _ => (),
    });

    rsx! {
        <Background styles={Some(background_styles)}>
            <Text line_height={Some(26.0)} size={14.0} content={format!("{:?}\n Materials Required: {:?}", ingot_produced.clone(), ore_required.clone())} />
            <SmeltButton on_event={Some(on_event)} />
        </Background>
    }
}

#[derive(WidgetProps, Clone, Debug, Default, PartialEq)]
pub struct SmeltButtonProps {
    #[prop_field(Styles)]
    styles: Option<Style>,
    #[prop_field(OnEvent)]
    pub on_event: Option<OnEvent>,
}


#[widget]
pub fn SmeltButton(props: SmeltButtonProps) {
    let (color, set_color, ..) = use_state!(Color::new(0.0781, 0.0898, 0.101, 1.0));

    let base_styles = props.styles.clone().unwrap_or_default();
    props.styles = Some(Style {
        render_command: StyleProp::Value(RenderCommand::Layout),
        height: StyleProp::Value(Units::Pixels(32.0)),
        width: StyleProp::Value(Units::Pixels(80.0)),
        left: StyleProp::Value(Units::Stretch(1.0)),
        ..base_styles
    });

    let background_styles = Some(Style {
        border_radius: StyleProp::Value(Corner::all(5.0)),
        background_color: StyleProp::Value(color),
        cursor: CursorIcon::Hand.into(),
        padding_left: StyleProp::Value(Units::Pixels(8.0)),
        ..Style::default()
    });

    let text_styles = Some(Style {
        cursor: StyleProp::Inherit,
        ..Style::default()
    });

    let on_event = OnEvent::new(move |_, event| match event.event_type {
        EventType::MouseIn(..) => {
            set_color(Color::new(0.0791, 0.0998, 0.201, 1.0));
        }
        EventType::MouseOut(..) => {
            set_color(Color::new(0.0781, 0.0898, 0.101, 1.0));
        }
        _ => {}
    });

    rsx! {
        <Background styles={background_styles} on_event={Some(on_event)}>
            <Text content={"SMELT".to_string()} size={16.0} styles={text_styles} />
        </Background>
    }
}