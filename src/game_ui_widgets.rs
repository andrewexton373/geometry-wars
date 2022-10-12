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

use crate::refinery::RefineryRecipe;
use crate::{HEIGHT, RESOLUTION};
use crate::game_ui::UIItems;
use crate::inventory::{InventoryItem, Amount};


#[widget]
pub fn UIShipInventory() {
    let ui_items = context.query_world::<Res<Binding<UIItems>>, _, _>(move |ui_items| ui_items.clone());
    context.bind(&ui_items);

    let inventory = ui_items.get().ship_inventory_items;
    let can_deposit = ui_items.get().can_deposit;

    let size = Vec2 { x: 200.0, y: 500.0 };
    let ui_ship_inventory_pos = (HEIGHT * RESOLUTION - size.x, HEIGHT - size.y);

    rsx! {
        <Window position={ui_ship_inventory_pos} size={(size.x, size.y)} title={"Ship Inventory".to_string()}>

            <If condition={can_deposit}>
                <Text content={"Press SPACE to deposit ore.".to_string()} size={16.0} />
            </If>

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
                    // <Text content={format!("Ingot: {:?} \n| Quantity: {:?}", ingot, amount)} size={16.0} />
                    <Text content={format!("{:?}", ingot)} size={16.0} />
                    <Text content={format!("x{}", quantity)} size={14.0} />
                </Background>
            }
        },
        _ => {}
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

    let refinery = ui_items.get().refinery.clone();

    let handle_create = Handler::new(move |refineable_id: usize| {
        println!("CRAFT REFINEABLE! {}", refineable_id);
    });

    let size = Vec2 { x: 400.0, y: 400.0 };
    let offset = 200.0; // width of station inventory
    let ui_refinery_view_pos = (0.0 + offset, HEIGHT - size.y);
    
    rsx! {
        <Window position={ui_refinery_view_pos} size={(size.x, size.y)} title={"Station Refinery".to_string()}>

            <CurrentlyProcessing currently_processing={refinery.currently_processing.clone()} />
            <Refineables refineables={refinery.recipes.clone()} on_create={handle_create} />

        </Window>
    }
}

#[derive(WidgetProps, Clone, Debug, Default, PartialEq)]
pub struct CurrentlyProcessingProps {
    pub currently_processing: Option<RefineryRecipe>
}

#[widget]
pub fn CurrentlyProcessing(props: CurrentlyProcessingProps) {
    let CurrentlyProcessingProps { currently_processing } = props.clone();

    let background_styles = Style {
        layout_type: StyleProp::Value(LayoutType::Row),
        background_color: StyleProp::Value(Color::new(0.999, 0.196, 0.215, 1.0)),
        height: StyleProp::Value(Units::Auto),
        top: StyleProp::Value(Units::Pixels(10.0)),
        padding: StyleProp::Value(Edge::all(Units::Pixels(5.0))),
        ..Style::default()
    };

    rsx! {
        <If condition={currently_processing.is_some()}>
            // <Background styles={Some(background_styles)}>
                <Text content={"Currently Processing:".to_string()} size={14.0} />
                <Text content={format!("{:?}\n Into {:?}", currently_processing.clone().unwrap().items_required, currently_processing.clone().unwrap().item_created)} size={16.0} />
            // </Background>
        </If>
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
                <Refineable refineable_id={index} refinery_recipe={recipe.clone()} on_create={on_create.clone()}/>
            }
        }))}

    </Element>

    }
}

#[derive(WidgetProps, Clone, Debug, Default, PartialEq)]
pub struct RefineableProps {
    pub refineable_id: usize,
    pub refinery_recipe: RefineryRecipe,
    pub on_create: Handler<usize>,
}

pub struct SmeltEvent(pub RefineryRecipe);

#[widget]
pub fn Refineable(props: RefineableProps) {
    let RefineableProps {
        refineable_id,
        refinery_recipe,
        on_create,
    } = props.clone();

    let clone = refinery_recipe.clone();

    let background_styles = Style {
        layout_type: StyleProp::Value(LayoutType::Row),
        background_color: StyleProp::Value(Color::new(0.176, 0.196, 0.215, 1.0)),
        height: StyleProp::Value(Units::Auto),
        top: StyleProp::Value(Units::Pixels(10.0)),
        padding: StyleProp::Value(Edge::all(Units::Pixels(5.0))),
        ..Style::default()
    };

    let on_create = on_create.clone();
    let on_event = OnEvent::new(move |ctx, event| match event.event_type {
        EventType::Click(..) => {
            println!("SMELT BUTTON CLICKED!");
            ctx.query_world::<EventWriter<SmeltEvent>, _, ()>(|mut writer| writer.send(SmeltEvent(clone.clone())));
            // on_create.call(refineable_id);
        }
        _ => (),
    });

    let item_created = refinery_recipe.clone().item_created;
    let items_required = refinery_recipe.clone().items_required.clone();

    rsx! {
        <Background styles={Some(background_styles)}>
            <Text line_height={Some(26.0)} size={14.0} content={format!("{:?}\n Materials Required: {:?}", item_created, items_required)} />
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