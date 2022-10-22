use kayak_ui::core::{
    rsx,
    widget, use_state, Handler, OnLayout,
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

use crate::refinery::Recipe;
use crate::{HEIGHT, RESOLUTION};
use crate::game_ui::{UIItems};
use crate::inventory::{InventoryItem, Amount};
use crate::widgets::currently_processing::{CurrentlyProcessing, CurrentlyProcessingProps};



#[widget]
pub fn UIFactoryView() {

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

    let factory = ui_items.get().factory.clone();

    let handle_create = Handler::new(move |craftable_id: usize| {
        println!("CRAFT Component! {}", craftable_id);
    });

    let size = Vec2 { x: 400.0, y: 400.0 };
    let offset = 600.0; // width of station inventory
    let ui_factory_view_pos = (0.0 + offset, HEIGHT - size.y);

    rsx! {
        <>
            <CurrentlyProcessing currently_processing={factory.currently_processing.clone()} time_remaining={factory.remaining_processing_time} percent_remaining={factory.remaining_processing_percent()} />
            <Craftables craftables={factory.recipes.clone()} on_create={handle_create} />
        </>
    }
}

#[derive(WidgetProps, Clone, Debug, Default, PartialEq)]
pub struct CraftablesProps {
    craftables: Vec<Recipe>,
    pub on_create: Handler<usize>,
}

#[widget]
pub fn Craftables(props: CraftablesProps) {
    let CraftablesProps { craftables, on_create } = props.clone();

    rsx! {
    <Element>
        {VecTracker::from(craftables.clone().into_iter().enumerate().map(|(index, recipe)| {
            constructor! {
                <Craftable craftable_id={index} factory_recipe={recipe.clone()} on_create={on_create.clone()}/>
            }
        }))}

    </Element>

    }
}

#[derive(WidgetProps, Clone, Debug, Default, PartialEq)]
pub struct CraftableProps {
    pub craftable_id: usize,
    pub factory_recipe: Recipe,
    pub on_create: Handler<usize>,
}

pub struct CraftEvent(pub Recipe);

#[widget]
pub fn Craftable(props: CraftableProps) {
    let CraftableProps {
        craftable_id,
        factory_recipe,
        on_create,
    } = props.clone();

    let clone = factory_recipe.clone();

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
            println!("CRAFT BUTTON CLICKED!");
            ctx.query_world::<EventWriter<CraftEvent>, _, ()>(|mut writer| writer.send(CraftEvent(clone.clone())));
            // on_create.call(refineable_id);
        }
        _ => (),
    });

    let item_created = factory_recipe.clone().item_created;
    let items_required = factory_recipe.clone().items_required.clone();

    rsx! {
        <Background styles={Some(background_styles)}>
            <Text line_height={Some(26.0)} size={14.0} content={format!("{:?}\n Materials Required: {:?}", item_created, items_required)} />
            <CraftButton on_event={Some(on_event)} />
        </Background>
    }
}

#[derive(WidgetProps, Clone, Debug, Default, PartialEq)]
pub struct CraftButtonProps {
    #[prop_field(Styles)]
    styles: Option<Style>,
    #[prop_field(OnEvent)]
    pub on_event: Option<OnEvent>,
}


#[widget]
pub fn CraftButton(props: CraftButtonProps) {
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
            <Text content={"CRAFT".to_string()} size={16.0} styles={text_styles} />
        </Background>
    }
}