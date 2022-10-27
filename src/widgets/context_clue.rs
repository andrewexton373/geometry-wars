use bevy::utils::HashSet;
use kayak_ui::core::styles::{Units, PositionType};
use kayak_ui::core::{constructor, rsx, widget, VecTracker};

use kayak_ui::core::{
    color::Color,
    styles::{Style, StyleProp, Edge},
    WidgetProps,
};
use kayak_ui::core::{Binding, Bound};
use kayak_ui::widgets::{Background, Text, Window, Element};

use bevy::prelude::*;

use crate::game_ui::{UIItems, ContextClue};
use crate::{HEIGHT, RESOLUTION};

#[widget]
pub fn UIContextClueView(props: UIContextClueProps) {
    let ui_items =
        context.query_world::<Res<Binding<UIItems>>, _, _>(move |ui_items| ui_items.clone());
    context.bind(&ui_items);

    let context_clues = ui_items.get().context_clues.clone();

    let container_styles = Some(Style {
        width: StyleProp::Value(Units::Percentage(100.0)),
        height: StyleProp::Value(Units::Percentage(100.0)),
        position_type: StyleProp::Value(PositionType::SelfDirected),
        ..Style::default()
    });

    if !context_clues.is_empty() {
        rsx! {
            <Element styles={container_styles}>
                <UIContextClues context_clues={context_clues.clone()} />
            </Element>
        }
    }

}

#[derive(WidgetProps, Clone, Debug, Default, PartialEq)]
pub struct UIContextCluesProps {
    context_clues: HashSet<ContextClue>
}

#[widget]
pub fn UIContextClues(props: UIContextCluesProps) {
    let UIContextCluesProps { context_clues } = props.clone();

    let size = Vec2 {x: 400., y: 120.};

    let context_clues_styles = Some(Style {
        position_type: StyleProp::Value(PositionType::SelfDirected),
        left: StyleProp::Value(Units::Percentage(40.0)),
        top: StyleProp::Value(Units::Percentage(10.0)),
        width: StyleProp::Value(Units::Percentage(20.0)),
        height: StyleProp::Value(Units::Auto),
        padding: StyleProp::Value(Edge::all(Units::Pixels(10.0))),
        background_color: StyleProp::Value(Color::new(0.4, 0.4, 0.4, 1.0)),
        ..Default::default()
    });

    rsx! {
        <Background styles={context_clues_styles}>
            <Text content={"Context Clues".to_string()} size={14.0} />
            {VecTracker::from(context_clues.clone().into_iter().enumerate().map(|(index, context_clue)| {
                constructor! {
                    <UIContextClue context_clue={context_clue.text()} />

                }
            }))}
        </Background>
    }
}

#[derive(WidgetProps, Clone, Debug, Default, PartialEq)]
pub struct UIContextClueProps {
    context_clue: String,
}

#[widget]
pub fn UIContextClue(props: UIContextClueProps) {
    let UIContextClueProps { context_clue } = props.clone();

    let bg_styles = Some(Style {
        height: StyleProp::Value(Units::Auto),
        ..Style::default()
    });

    let text_styles = Some(Style {
        cursor: StyleProp::Inherit,
        // height: StyleProp::Value(Units::Auto),
        ..Style::default()
    });

    rsx! {
        <Background styles={bg_styles}>
            <Text content={context_clue} size={12.0} styles={text_styles} />
        </Background>
    }
}
