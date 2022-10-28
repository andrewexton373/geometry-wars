use kayak_ui::core::{rsx, use_state, widget, Handler};

use kayak_ui::core::{
    color::Color,
    render_command::RenderCommand,
    styles::{Corner, Edge, LayoutType, Style, StyleProp, Units},
    CursorIcon, EventType, OnEvent, WidgetProps,
};
use kayak_ui::core::{constructor, Binding, Bound, VecTracker};
use kayak_ui::widgets::{Background, Clip, Element, ScrollBox, ScrollMode, Text};

use bevy::prelude::*;

use crate::game_ui::UIItems;
use crate::inventory::InventoryItem;
use crate::item_producer::ItemProducer;
use crate::recipe::Recipe;
use crate::widgets::currently_processing::CurrentlyProcessing;

#[widget]
pub fn UIRefineryView() {
    let (color, set_color, ..) = use_state!(Color::new(0.0781, 0.0898, 0.101, 1.0));

    let background_styles = Some(Style {
        height: StyleProp::Value(Units::Auto),
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

    let ui_items =
        context.query_world::<Res<Binding<UIItems>>, _, _>(move |ui_items| ui_items.clone());
    context.bind(&ui_items);

    let refinery = ui_items.get().refinery.clone();

    let handle_create = Handler::new(move |refineable_id: usize| {
        println!("CRAFT REFINEABLE! {}", refineable_id);
    });

    rsx! {
        <>
            // <ScrollBox always_show_scrollbar={true}>

            <CurrentlyProcessing currently_processing={refinery.currently_processing.clone()} time_remaining={refinery.remaining_processing_time} percent_remaining={refinery.remaining_processing_percent()}/>

            <Refineables refineables={refinery.recipes.clone()} on_create={handle_create} />
            // </ScrollBox>

        </>
    }
}

#[derive(WidgetProps, Clone, Debug, Default, PartialEq)]
pub struct RefineablesProps {
    refineables: Vec<Recipe>,
    pub on_create: Handler<usize>,
}

#[widget]
pub fn Refineables(props: RefineablesProps) {
    let RefineablesProps {
        refineables,
        on_create,
    } = props.clone();

    let auto = Some(Style {
        height: StyleProp::Value(Units::Stretch(1.0)),
        render_command: StyleProp::Value(RenderCommand::Clip),
        ..Style::default()
    });

    let clamped = ScrollMode::Clamped;

    rsx! {
        <ScrollBox styles={auto} mode={clamped}>
        {VecTracker::from(refineables.clone().into_iter().enumerate().map(|(index, recipe)| {
            constructor! {
                <Refineable refineable_id={index} refinery_recipe={recipe.clone()} on_create={on_create.clone()}/>
            }
        }))}
        </ScrollBox>
    }
}

#[derive(WidgetProps, Clone, Debug, Default, PartialEq)]
pub struct RefineableProps {
    pub refineable_id: usize,
    pub refinery_recipe: Recipe,
    pub on_create: Handler<usize>,
}

pub struct SmeltEvent(pub Recipe);

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
            ctx.query_world::<EventWriter<SmeltEvent>, _, ()>(|mut writer| {
                writer.send(SmeltEvent(clone.clone()))
            });
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
pub struct UIRequirementsProps {
    pub required: Vec<InventoryItem>,
}

#[widget]
pub fn UIRequirements(props: UIRequirementsProps) {
    let UIRequirementsProps { required } = props.clone();

    rsx! {
        <Text size={14.0} content={"REQUIREMENTS: TODO!".to_string()} />

        // <Text size={14.0} content={format!("Requirements: {:?}", required)} />
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
