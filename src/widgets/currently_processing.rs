use kayak_ui::core::{rsx, widget};

use kayak_ui::core::{
    color::Color,
    styles::{Edge, LayoutType, Style, StyleProp, Units},
    WidgetProps,
};
use kayak_ui::widgets::{Element, If, Text};

use crate::recipe::Recipe;
use crate::widgets::progress_bar::ProgressBar;

#[derive(WidgetProps, Clone, Debug, Default, PartialEq)]
pub struct CurrentlyProcessingProps {
    pub currently_processing: Option<Recipe>,
    pub time_remaining: f32,
    pub percent_remaining: Option<f32>,
}

#[widget]
pub fn CurrentlyProcessing(props: CurrentlyProcessingProps) {
    let CurrentlyProcessingProps {
        currently_processing,
        time_remaining,
        percent_remaining,
    } = props.clone();

    let background_styles = Style {
        layout_type: StyleProp::Value(LayoutType::Column),
        background_color: StyleProp::Value(Color::new(0.999, 0.196, 0.215, 1.0)),
        height: StyleProp::Value(Units::Auto),
        // top: StyleProp::Value(Units::Pixels(10.0)),
        // padding: StyleProp::Value(Edge::all(Units::Pixels(5.0))),
        ..Style::default()
    };

    rsx! {
        <If styles={Some(background_styles)} condition={currently_processing.is_some()}>
            <ProgressBar percent={percent_remaining} />
            <Text content={format!("{:.1} Seconds Remaining", time_remaining)} size={11.0} />

            <Text content={"Currently Processing:".to_string()} size={14.0} />
            <Text content={format!("{:?}\n Into {:?}", currently_processing.clone().unwrap().items_required, currently_processing.clone().unwrap().item_created)} size={16.0} />
        </If>
    }
}
