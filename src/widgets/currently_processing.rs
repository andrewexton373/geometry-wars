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
use crate::widgets::progress_bar::{ProgressBar, ProgressBarProps};

#[derive(WidgetProps, Clone, Debug, Default, PartialEq)]
pub struct CurrentlyProcessingProps {
    pub currently_processing: Option<Recipe>,
    pub time_remaining: f32,
    pub percent_remaining: Option<f32>
}

#[widget]
pub fn CurrentlyProcessing(props: CurrentlyProcessingProps) {
    let CurrentlyProcessingProps { currently_processing, time_remaining, percent_remaining } = props.clone();

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
                <ProgressBar percent={percent_remaining} />
                <Text content={"Currently Processing:".to_string()} size={14.0} />
                <Text content={format!("{:?}\n Into {:?}", currently_processing.clone().unwrap().items_required, currently_processing.clone().unwrap().item_created)} size={16.0} />
                <Text content={format!("{:.1} Seconds Remaining", time_remaining)} size={16.0} />

            // </Background>
        </If>
    }
}