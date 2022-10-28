use std::ops::Index;

use kayak_ui::core::{render, rsx, use_state, widget, Fragment, Handler, KeyCode};

use kayak_ui::core::{
    color::Color,
    render_command::RenderCommand,
    styles::{Edge, LayoutType, PositionType, Style, StyleProp, Units},
    EventType, OnEvent, WidgetProps,
};
use kayak_ui::core::{constructor, Bound, VecTracker};
use kayak_ui::widgets::{Background, Element, ScrollBox, ScrollMode, Text, Window};

use bevy::prelude::*;

use crate::game_ui::UIItems;
use crate::widgets::factory::UIFactoryView;
use crate::widgets::refinery::UIRefineryView;
use crate::{HEIGHT, RESOLUTION};

#[widget]
pub fn UICraftingTabsView() {
    let theme = TabTheme {
        primary: Default::default(),
        bg: Color::new(0.176, 0.227, 0.255, 1.0),
        fg: Color::new(0.286, 0.353, 0.392, 1.0),
        focus: Color::new(0.388, 0.474, 0.678, 0.5),
        text: ColorState {
            normal: Color::new(0.949, 0.956, 0.968, 1.0),
            hovered: Color::new(0.650, 0.574, 0.669, 1.0),
            active: Color::new(0.949, 0.956, 0.968, 1.0),
        },
        active_tab: ColorState {
            normal: Color::new(0.286, 0.353, 0.392, 1.0),
            hovered: Color::new(0.246, 0.323, 0.352, 1.0),
            active: Color::new(0.196, 0.283, 0.312, 1.0),
        },
        inactive_tab: ColorState {
            normal: Color::new(0.176, 0.227, 0.255, 1.0),
            hovered: Color::new(0.16, 0.21, 0.23, 1.0),
            active: Color::new(0.196, 0.283, 0.312, 1.0),
        },
        tab_height: 22.0,
    };

    let size = Vec2 { x: 400.0, y: 400.0 };
    let offset = 200.0; // width of station inventory
    let view_pos = (0.0 + offset, HEIGHT - size.y);

    let container_styles = Some(Style {
        width: StyleProp::Value(Units::Percentage(100.0)),
        height: StyleProp::Value(Units::Percentage(100.0)),
        position_type: StyleProp::Value(PositionType::SelfDirected),
        // background_color: StyleProp::Value(Color::new(1.0, 0.0, 0.0, 0.8)),
        ..Style::default()
    });

    let ship_information_styles = Some(Style {
        top: StyleProp::Value(Units::Stretch(1.0)),
        left: StyleProp::Value(Units::Pixels(200.0)),
        width: StyleProp::Value(Units::Pixels(400.0)),
        height: StyleProp::Value(Units::Pixels(300.0)),
        padding: StyleProp::Value(Edge::all(Units::Pixels(10.0))),
        background_color: StyleProp::Value(Color::new(0.4, 0.4, 0.4, 1.0)),
        ..Default::default()
    });

    rsx! {
        <Element styles={container_styles}>
            <Background styles={ship_information_styles}>
                <Text content={"Station Production".to_string()}/>
                <TabThemeProvider initial_theme={theme}>
                    <TabDemo />
                </TabThemeProvider>
            </Background>
        </Element>
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct TabData {
    /// The name of this tab
    pub name: String,
    /// The content to display for this tab, wrapped in a [Fragment]
    pub content: Fragment,
}

#[derive(WidgetProps, Default, Debug, PartialEq, Clone)]
pub struct TabBoxProps {
    pub initial_tab: usize,
    pub tabs: Vec<TabData>,
    #[prop_field(Styles)]
    pub styles: Option<Style>,
}

#[derive(Clone, PartialEq)]
enum TabHoverState {
    None,
    Inactive,
    Active,
}

#[derive(WidgetProps, Default, Debug, PartialEq, Clone)]
pub struct TabProps {
    pub content: String,
    pub selected: bool,
    #[prop_field(Styles)]
    pub styles: Option<Style>,
    #[prop_field(OnEvent)]
    pub on_event: Option<OnEvent>,
}

#[derive(WidgetProps, Default, Debug, PartialEq, Clone)]
pub struct TabBarProps {
    pub tabs: Vec<String>,
    pub selected: usize,
    pub on_select_tab: Handler<usize>,
    #[prop_field(Styles)]
    pub styles: Option<Style>,
}

/// A widget displaying a collection of tabs in a horizontal bar
#[widget]
pub fn TabBar(props: TabBarProps) {
    let TabBarProps {
        on_select_tab,
        selected,
        tabs,
        ..
    } = props.clone();
    let theme = context.create_consumer::<TabTheme>().unwrap_or_default();

    let tabs = tabs.into_iter().enumerate().map(move |(index, tab)| {
        let on_select = on_select_tab.clone();
        let tab_event_handler = OnEvent::new(move |_, event| {
            match event.event_type {
                EventType::Click(..) =>  {
                    on_select.call(index);
                }
                EventType::KeyDown(evt) => {
                    if evt.key() == KeyCode::Return || evt.key() == KeyCode::Space {
                        // We want the focused tab to also be selected by `Enter` or `Space`
                        on_select.call(index);
                    }
                }
                _ => {}
            }
        });

        constructor! {
            <Tab content={tab.clone()} on_event={Some(tab_event_handler.clone())} selected={selected == index} />
        }
    });

    let background_styles = Style {
        layout_type: StyleProp::Value(LayoutType::Row),
        background_color: StyleProp::Value(theme.get().bg),
        height: StyleProp::Value(Units::Auto),
        width: StyleProp::Value(Units::Stretch(1.0)),
        ..props.styles.clone().unwrap_or_default()
    };

    rsx! {
        <Background styles={Some(background_styles)}>
            {VecTracker::from(tabs.clone())}
        </Background>
    }
}

/// The actual tab, displayed in a [TabBar](crate::tab_bar::TabBar)
#[widget]
pub fn Tab(props: TabProps) {
    let TabProps {
        content, selected, ..
    } = props.clone();

    let theme = context.create_consumer::<TabTheme>().unwrap_or_default();
    let (focus_state, set_focus_state, ..) = use_state!(false);
    let (hover_state, set_hover_state, ..) = use_state!(TabHoverState::None);
    match hover_state {
        TabHoverState::Inactive if selected => set_hover_state(TabHoverState::Active),
        TabHoverState::Active if !selected => set_hover_state(TabHoverState::Inactive),
        _ => {}
    };

    let event_handler = OnEvent::new(move |_, event| match event.event_type {
        EventType::Hover(..) => {
            if selected {
                set_hover_state(TabHoverState::Active);
            } else {
                set_hover_state(TabHoverState::Inactive);
            }
        }
        EventType::MouseOut(..) => {
            set_hover_state(TabHoverState::None);
        }
        EventType::Focus => {
            set_focus_state(true);
        }
        EventType::Blur => {
            set_focus_state(false);
        }
        _ => {}
    });

    let tab_color = match hover_state {
        TabHoverState::None if selected => theme.get().active_tab.normal,
        TabHoverState::None => theme.get().inactive_tab.normal,
        TabHoverState::Inactive => theme.get().inactive_tab.hovered,
        TabHoverState::Active => theme.get().active_tab.hovered,
    };

    let pad_x = Units::Pixels(2.0);
    let bg_styles = Style {
        background_color: StyleProp::Value(tab_color),
        layout_type: StyleProp::Value(LayoutType::Row),
        padding_left: StyleProp::Value(pad_x),
        padding_right: StyleProp::Value(pad_x),
        ..Default::default()
    };

    let border_width = Units::Pixels(2.0);
    let border_styles = Style {
        background_color: if focus_state {
            StyleProp::Value(theme.get().focus)
        } else {
            StyleProp::Value(tab_color)
        },
        padding: StyleProp::Value(Edge::all(border_width)),
        layout_type: StyleProp::Value(LayoutType::Row),
        ..Default::default()
    };

    let text_styles = Style {
        background_color: if focus_state {
            StyleProp::Value(theme.get().focus)
        } else {
            StyleProp::Value(tab_color)
        },
        color: StyleProp::Value(theme.get().text.normal),
        top: StyleProp::Value(Units::Stretch(0.1)),
        bottom: StyleProp::Value(Units::Stretch(1.0)),
        width: StyleProp::Value(Units::Stretch(1.0)),
        ..Default::default()
    };

    props.styles = Some(Style {
        render_command: StyleProp::Value(RenderCommand::Layout),
        height: StyleProp::Value(Units::Pixels(theme.get().tab_height)),
        max_width: StyleProp::Value(Units::Pixels(100.0)),
        ..props.styles.clone().unwrap_or_default()
    });

    rsx! {
        <Background focusable={Some(true)} on_event={Some(event_handler)} styles={Some(border_styles)}>
            <Background styles={Some(bg_styles)}>
                <Text content={content} size={12.0} styles={Some(text_styles)} />
            </Background>
        </Background>
    }
}

/// The actual tab container widget.
///
/// This houses both the tab bar and its content.
#[widget]
pub fn TabBox(props: TabBoxProps) {
    let TabBoxProps {
        initial_tab, tabs, ..
    } = props.clone();
    let theme = context.create_consumer::<TabTheme>().unwrap_or_default();
    let (selected, set_selected, ..) = use_state!(initial_tab);

    let tab_names = tabs
        .iter()
        .map(|tab| tab.name.clone())
        .collect::<Vec<String>>();
    let tab_content = tabs
        .iter()
        .map(|tab| tab.content.clone())
        .collect::<Vec<_>>();

    let on_select_tab = Handler::<usize>::new(move |index| {
        set_selected(index);
    });

    props.styles = Some(Style {
        render_command: StyleProp::Value(RenderCommand::Quad),
        background_color: StyleProp::Value(theme.get().fg),
        ..Default::default()
    });

    rsx! {
        <>
            <TabBar tabs={tab_names} selected={selected} on_select_tab={on_select_tab} />
            <TabContent tabs={tab_content} selected={selected}  />
        </>
    }
}

#[derive(WidgetProps, Default, Debug, PartialEq, Clone)]
pub struct TabContentProps {
    pub selected: usize,
    pub tabs: Vec<Fragment>,
    #[prop_field(Styles)]
    pub styles: Option<Style>,
}

/// A widget that displays the selected tab's content
#[widget]
pub fn TabContent(props: TabContentProps) {
    let TabContentProps { selected, tabs, .. } = props.clone();
    let theme = context.create_consumer::<TabTheme>().unwrap_or_default();

    if selected >= tabs.len() {
        // Invalid tab -> don't do anything
        return;
    }

    props.styles = Some(Style {
        render_command: StyleProp::Value(RenderCommand::Quad),
        background_color: StyleProp::Value(theme.get().fg),
        ..Default::default()
    });

    let tab = tabs.index(selected).clone();
    let tab = vec![tab.clone()];

    rsx! {
        <>
            {VecTracker::from(tab.clone().into_iter())}
        </>
    }
}

#[widget]
fn TabDemo() {
    let text_style = Style {
        width: StyleProp::Value(Units::Percentage(75.0)),
        top: StyleProp::Value(Units::Stretch(0.5)),
        left: StyleProp::Value(Units::Stretch(1.0)),
        bottom: StyleProp::Value(Units::Stretch(1.0)),
        right: StyleProp::Value(Units::Stretch(1.0)),
        ..Default::default()
    };

    // TODO: This is not the most ideal way to generate tabs. For one, the `content` has no access to its actual context
    // (i.e. where it actually exists in the hierarchy). Additionally, it would be better if tabs were created as
    // children of `TabBox`. These are issues that will be addressed in the future, so for now, this will work.
    let tabs = vec![
        TabData {
            name: "Refinery".to_string(),
            content: {
                let text_style = text_style.clone();
                constructor! {
                    <>
                        // <Text content={"Welcome to the refinery, smelt your ores here!".to_string()} size={14.0} styles={Some(text_style)} />
                        <UIRefineryView />
                    </>
                }
            },
        },
        TabData {
            name: "Factory".to_string(),
            content: {
                let text_style = text_style.clone();
                constructor! {
                    <>
                        // <Text content={"Welcome to the factory, refine your ingots into components here!".to_string()} size={14.0} styles={Some(text_style)} />
                        <UIFactoryView />
                    </>
                }
            },
        },
    ];

    rsx! {
        <TabBox tabs={tabs} />
    }
}

use kayak_ui::core::Children;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TabTheme {
    pub primary: Color,
    pub bg: Color,
    pub fg: Color,
    pub focus: Color,
    pub text: ColorState,
    pub active_tab: ColorState,
    pub inactive_tab: ColorState,
    pub tab_height: f32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ColorState {
    pub normal: Color,
    pub hovered: Color,
    pub active: Color,
}

#[derive(WidgetProps, Default, Debug, PartialEq, Clone)]
pub struct TabThemeProviderProps {
    pub initial_theme: TabTheme,
    #[prop_field(Children)]
    pub children: Option<Children>,
}

#[widget]
pub fn TabThemeProvider(props: TabThemeProviderProps) {
    let TabThemeProviderProps {
        initial_theme,
        children,
    } = props.clone();
    context.create_provider(initial_theme);
    rsx! { <>{children}</> }
}
