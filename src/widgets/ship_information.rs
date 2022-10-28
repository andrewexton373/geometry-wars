

use kayak_ui::core::styles::PositionType;
use kayak_ui::core::{rsx, widget};

use kayak_ui::core::{
    color::Color,
    styles::{Edge, Style, StyleProp, Units}, WidgetProps,
};
use kayak_ui::core::{Binding, Bound};
use kayak_ui::widgets::{Background, Text};

use bevy::prelude::*;

use crate::game_ui::UIItems;
use crate::item_producer::ItemProducer;




#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ShipInformation {
    pub net_weight: f32,
    pub speed: f32,
    pub direction: f32,
}

#[widget]
pub fn UIShipInformationView() {
    let ui_items =
        context.query_world::<Res<Binding<UIItems>>, _, _>(move |ui_items| ui_items.clone());
    context.bind(&ui_items);

    let ship_info = ui_items.get().ship_info; // TODO!

    let container_styles = Some(Style {
        width: StyleProp::Value(Units::Percentage(100.0)),
        height: StyleProp::Value(Units::Percentage(100.0)),
        position_type: StyleProp::Value(PositionType::SelfDirected),
        // background_color: StyleProp::Value(Color::new(1.0, 0.0, 0.0, 0.8)),
        ..Style::default()
    });

    let ship_information_styles = Some(Style {
        left: StyleProp::Value(Units::Stretch(1.0)),
        width: StyleProp::Value(Units::Pixels(200.0)),
        height: StyleProp::Value(Units::Auto),
        padding: StyleProp::Value(Edge::all(Units::Pixels(10.0))),
        background_color: StyleProp::Value(Color::new(0.4, 0.4, 0.4, 1.0)),
        ..Default::default()
    });

    rsx! {
            <Background styles={container_styles}>
                <Background styles={ship_information_styles}>
                    <Text content={"Ship Information".to_string()} size={14.0} />
                    <Text content={"Heading".to_string()} size={12.0} />
                    // FIXME: Need to correct for 0 - 360
                    <Text content={(ship_info.direction).to_degrees().to_string()} size={10.0} />
                    <Text content={"Speed".to_string()} size={12.0} />
                    <Text content={format!("{:.0} m/s", ship_info.speed)} size={10.0} />
                    <Text content={"Net Weight".to_string()} size={12.0} />
                    <Text content={format!("{:.2} kgs", ship_info.net_weight)} size={10.0} />
                </Background>
            </Background>
    }
}
