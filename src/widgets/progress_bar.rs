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

use crate::{HEIGHT, RESOLUTION};
use crate::game_ui::{UIItems};
use crate::inventory::{InventoryItem, Amount};

#[derive(WidgetProps, Clone, Debug, Default, PartialEq)]
pub struct ProgressBarProps {
    pub percent: Option<f32>
}

#[widget]
pub fn ProgressBar(props: ProgressBarProps) {
    let ProgressBarProps { percent } = props.clone();

    let progress_bar_background_style = Style {
        layout_type: StyleProp::Value(LayoutType::Column),
        width: StyleProp::Value(Units::Percentage(100.0)),
        height: StyleProp::Value(Units::Auto),
        background_color: StyleProp::Value(Color::new(1.0, 0.0, 0.0, 1.0)),
        ..Default::default()
    };


    // The background style of element growing/shrink
    let mut progress_bar_fill_style = Style {
        width: StyleProp::Value(Units::Pixels(0.0)),
        height: StyleProp::Value(Units::Pixels(8.0)),
        layout_type: StyleProp::Value(LayoutType::Column),
        background_color: StyleProp::Value(Color::new(0.0, 1.0, 0.0, 1.0)),
        ..Default::default()
    };

    if let Some(percent) = percent {
        progress_bar_fill_style.width = StyleProp::Value(Units::Percentage(percent * 100.0));
    }

    rsx! {
        <Background styles={Some(progress_bar_background_style)}>
                <Background styles={Some(progress_bar_fill_style)} />
        </Background>
    }
}
