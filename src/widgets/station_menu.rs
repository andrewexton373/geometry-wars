use std::f32::consts::PI;

use kayak_ui::core::styles::PositionType;
use kayak_ui::core::{rsx, use_state, widget, Handler};

use kayak_ui::core::{
    color::Color,
    render_command::RenderCommand,
    styles::{Corner, Edge, LayoutType, Style, StyleProp, Units},
    CursorIcon, EventType, OnEvent, WidgetProps,
};
use kayak_ui::core::{constructor, Binding, Bound, VecTracker};
use kayak_ui::widgets::{Background, Element, Text, Window, NinePatch, Clip, If};

use bevy::prelude::{*};

use crate::{HEIGHT, RESOLUTION, WIDTH};
use crate::game_ui::{UIItems, ContextClue};
use crate::item_producer::ItemProducer;
use crate::recipe::Recipe;
use crate::widgets::currently_processing::CurrentlyProcessing;

#[widget]
pub fn UIStationMenu() {

    let (show, set_show, ..) = use_state!(false);

    let ui_items =
        context.query_world::<Res<Binding<UIItems>>, _, _>(move |ui_items| ui_items.clone());
    context.bind(&ui_items);

    let context_clues = ui_items.get().context_clues; // TODO!
    let near_base_station = context_clues.contains(&ContextClue::NearBaseStation);
    
    if !near_base_station {
        set_show(false);
    }

    let container_styles = Some(Style {
        width: StyleProp::Value(Units::Percentage(100.0)),
        height: StyleProp::Value(Units::Percentage(100.0)),
        position_type: StyleProp::Value(PositionType::SelfDirected),
        ..Style::default()
    });

    let station_menu_styles = Some(Style {
        left: StyleProp::Value(Units::Percentage(20.0)),
        right: StyleProp::Value(Units::Percentage(20.0)),
        top: StyleProp::Value(Units::Percentage(20.0)),
        bottom: StyleProp::Value(Units::Percentage(20.0)),

        width: StyleProp::Value(Units::Percentage(60.0)),
        height: StyleProp::Value(Units::Percentage(60.0)),
        padding: StyleProp::Value(Edge::all(Units::Pixels(10.0))),
        background_color: StyleProp::Value(Color::new(0.4, 0.4, 0.4, 1.0)),
        ..Default::default()
    });

    let on_menu_button_event = Some(OnEvent::new(move |ctx, event| match event.event_type {
        EventType::Click(..) => {
            println!("STATION MENU BUTTON CLICKED!");
            set_show(!show);
        }
        _ => (),
    }));


    rsx! {
        <>
            <If condition={near_base_station}>
                <StationMenuButton on_event={on_menu_button_event}/>
                <If condition={show}>
                    <Background styles={container_styles}>
                        <Background styles={station_menu_styles}>
                            <Text content={"Base Station Menu".to_string()} size={14.0} />
                            <Text content={"Upgrades".to_string()} size={12.0} />
                            <Text content={"Crafting".to_string()} size={12.0} />
                            <Text content={"Cargo Bay Inventory".to_string()} size={12.0} />
                        </Background>
                    </Background>
                </If>
            </If>
        </>
    }
   
}

#[derive(WidgetProps, Clone, Debug, Default, PartialEq)]
pub struct StationMenuButtonProps {
    #[prop_field(Styles)]
    styles: Option<Style>,
    #[prop_field(OnEvent)]
    pub on_event: Option<OnEvent>,
}

#[widget]
pub fn StationMenuButton(props: StationMenuButtonProps) {
    let (color, set_color, ..) = use_state!(Color::new(0.0781, 0.0898, 0.101, 1.0));

    let base_styles = props.styles.clone().unwrap_or_default();
    props.styles = Some(Style {
        render_command: StyleProp::Value(RenderCommand::Layout),
        height: StyleProp::Value(Units::Pixels(120.0)),
        width: StyleProp::Value(Units::Pixels(120.0)),
        // left: StyleProp::Value(Units::Stretch(1.0)),
        left: StyleProp::Value(Units::Percentage(0.0)),
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
            <Text content={"Station Menu".to_string()} size={16.0} styles={text_styles} />
        </Background>
    }
}