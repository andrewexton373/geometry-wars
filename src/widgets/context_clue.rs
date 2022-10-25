use kayak_ui::core::{constructor, rsx, widget, VecTracker};

use kayak_ui::core::{
    styles::{Style, StyleProp},
    WidgetProps,
};
use kayak_ui::core::{Binding, Bound};
use kayak_ui::widgets::{Background, Text, Window};

use bevy::prelude::*;

use crate::game_ui::UIItems;
use crate::{HEIGHT, RESOLUTION};

#[widget]
pub fn UIContextClueView(props: UIContextClueProps) {
    let ui_items =
        context.query_world::<Res<Binding<UIItems>>, _, _>(move |ui_items| ui_items.clone());
    context.bind(&ui_items);

    let context_clues = ui_items.get().context_clues.clone();

    let size = Vec2 { x: 400.0, y: 100.0 };
    let offset = 200.0; // width of station inventory
    let ui_context_clue_pos = (HEIGHT * RESOLUTION / 2. - size.x / 2.0, 100.0);

    if !context_clues.is_empty() {
        rsx! {
            <Window position={ui_context_clue_pos} size={(size.x, size.y)} title={"Context Clue".to_string()}>

                {VecTracker::from(context_clues.clone().into_iter().enumerate().map(|(index, context_clue)| {
                    constructor! {
                        <UIContextClue context_clue={context_clue.text()} />

                    }
                }))}

            </Window>
        }
    }

    // match context_clue {
    //     None => {},
    //     Some(context_clue) => {

    //         rsx! {
    //             <Window position={ui_context_clue_pos} size={(size.x, size.y)} title={"Context Clue".to_string()}>
    //                 <UIContextClue context_clue={context_clue.text()} />
    //             </Window>
    //         }
    //     }

    // }
}

#[derive(WidgetProps, Clone, Debug, Default, PartialEq)]
pub struct UIContextClueProps {
    context_clue: String,
}

#[widget]
pub fn UIContextClue(props: UIContextClueProps) {
    let UIContextClueProps { context_clue } = props.clone();

    // let background_styles = Some(Style {
    //     border_radius: StyleProp::Value(Corner::all(5.0)),
    //     background_color: StyleProp::Value(Color::WHITE),
    //     cursor: CursorIcon::Hand.into(),
    //     padding_left: StyleProp::Value(Units::Pixels(8.0)),
    //     ..Style::default()
    // });

    let text_styles = Some(Style {
        cursor: StyleProp::Inherit,
        ..Style::default()
    });

    rsx! {
        // <Background styles={background_styles}>
        <Background>
            <Text content={context_clue} size={16.0} styles={text_styles} />
        </Background>
    }
}
