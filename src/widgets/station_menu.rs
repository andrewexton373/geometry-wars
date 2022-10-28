use bevy_inspector_egui::Inspectable;
use kayak_ui::core::styles::{LayoutType, PositionType};
use kayak_ui::core::{constructor, rsx, use_state, widget};

use kayak_ui::core::{
    color::Color,
    render_command::RenderCommand,
    styles::{Corner, Edge, Style, StyleProp, Units},
    CursorIcon, EventType, OnEvent, WidgetProps,
};
use kayak_ui::core::{Binding, Bound, VecTracker};
use kayak_ui::widgets::{Background, Element, If, Text};

use bevy::prelude::*;

// use enum_iterator::{all, Sequence};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use strum_macros::FromRepr;

use crate::factory::UpgradeComponent;
use crate::game_ui::{ContextClue, UIItems};
use crate::inventory::{Amount, InventoryItem};
use crate::widgets::crafting::UICraftingTabsView;
use crate::widgets::inventory::UIBaseInventory;
use crate::widgets::refinery::UIRequirements;

#[widget]
pub fn UIStationMenu() {
    let (show, set_show, ..) = use_state!(false);

    let ui_items =
        context.query_world::<Res<Binding<UIItems>>, _, _>(move |ui_items| ui_items.clone());
    context.bind(&ui_items);

    let context_clues = ui_items.get().context_clues; // TODO!
    let near_base_station = context_clues.contains(&ContextClue::NearBaseStation);

    let upgrades = ui_items.get().upgrades;

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
                            <UIUpgradesMenu upgrades={upgrades}/>

                            <Text content={"Crafting".to_string()} size={12.0} />
                            <UICraftingTabsView />

                            <Text content={"Cargo Bay Inventory".to_string()} size={12.0} />
                            <UIBaseInventory />
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

#[derive(WidgetProps, Clone, Debug, Default, PartialEq)]
pub struct UIUpgradesMenuProps {
    pub upgrades: Vec<UpgradeType>,
}

#[widget]
pub fn UIUpgradesMenu(props: UIUpgradesMenuProps) {
    let UIUpgradesMenuProps { upgrades } = props.clone();

    rsx! {
        <>

            {VecTracker::from(upgrades.clone().into_iter().enumerate().map(|(index, upgrade)| {
                constructor! {
                    <UIUpgrade upgrade_type={Some(upgrade)} />
                }
            }))}
            // List Upgrades
            // <UIUpgrade />

        </>
    }
}

#[derive(WidgetProps, Clone, Debug, Default, PartialEq)]
pub struct UIUpgradeProps {
    #[prop_field(Styles)]
    styles: Option<Style>,
    #[prop_field(OnEvent)]
    pub on_event: Option<OnEvent>,
    pub upgrade_type: Option<UpgradeType>,
}

// #[derive(Clone, PartialEq)]
// pub enum UpgradeLevelRequriements {
//     Health(usize, Vec<InventoryItem>)
// }

#[derive(Debug, Clone, PartialEq, Inspectable)]
pub struct UpgradeRequirements {
    requirements: Vec<InventoryItem>,
}

#[derive(FromRepr, EnumIter, Debug, Clone, Default, PartialEq, Inspectable)]
#[repr(u8)]
pub enum UpgradeLevel {
    #[default]
    Level0 = 0,
    Level1(Option<UpgradeRequirements>) = 1,
    Level2(Option<UpgradeRequirements>) = 2,
    Level3(Option<UpgradeRequirements>) = 3,
    Level4(Option<UpgradeRequirements>) = 4,
    MaxLevel(Option<UpgradeRequirements>) = 5,
}

impl UpgradeLevel {
    pub fn as_u8(&self) -> u8 {
        match self {
            UpgradeLevel::Level0 => 0,
            UpgradeLevel::Level1(_) => 1,
            UpgradeLevel::Level2(_) => 2,
            UpgradeLevel::Level3(_) => 3,
            UpgradeLevel::Level4(_) => 4,
            UpgradeLevel::MaxLevel(_) => 5,
        }
    }

    pub fn next(&self) -> Option<UpgradeLevel> {
        let current_lvl = self.as_u8();
        UpgradeLevel::from_repr(current_lvl + 1)
    }
}

#[derive(Default, EnumIter, Clone, Debug, PartialEq, Inspectable)]
pub enum UpgradeType {
    #[default]
    None,
    Health(UpgradeLevel),
    ShipCargoBay(UpgradeLevel),
}

impl ToString for UpgradeType {
    fn to_string(&self) -> String {
        match self {
            UpgradeType::Health(_) => "Health",
            UpgradeType::ShipCargoBay(_) => "Ship Cargo Bay",
            UpgradeType::None => "NONE UPGRADE.",
        }
        .to_string()
    }
}

pub struct UpgradeEvent(pub UpgradeType);

#[widget]
pub fn UIUpgrade(props: UIUpgradeProps) {
    let UIUpgradeProps {
        styles,
        on_event,
        upgrade_type,
    } = props.clone();

    let styles = Some(Style {
        render_command: StyleProp::Value(RenderCommand::Quad),
        layout_type: StyleProp::Value(LayoutType::Column),
        padding: StyleProp::Value(Edge::all(Units::Pixels(8.0))),
        ..Default::default()
    });

    let cloned = upgrade_type.clone();

    let on_event = OnEvent::new(move |ctx, event| match event.event_type {
        EventType::Click(..) => {
            println!("UPGRADE BUTTON CLICKED! {:?}", cloned);

            // TODO Implement Events for Upgrades
            ctx.query_world::<EventWriter<UpgradeEvent>, _, ()>(|mut writer| {
                writer.send(UpgradeEvent(cloned.clone().unwrap()));
            });
            // on_create.call(refineable_id);
        }
        _ => (),
    });

    let requirements = vec![InventoryItem::Component(
        UpgradeComponent::Cog,
        Amount::Quantity(1),
    )];

    if let Some(upgrade_type) = upgrade_type.clone() {
        match upgrade_type.clone() {
            UpgradeType::Health(level) | UpgradeType::ShipCargoBay(level) => {
                rsx! {
                    <Element styles={styles}>
                        <Text content={format!("Upgrade Type: {}", upgrade_type.to_string())} />
                        <UIRequirements required={requirements} />
                        <UpgradeButton on_event={Some(on_event)} />

                        <UIUpgradeLevel upgrade_level={level.clone()} />
                    </Element>
                }
            }
            _ => {}
        }
    }
}

#[derive(WidgetProps, Clone, Debug, Default, PartialEq)]
pub struct UpgradeButtonProps {
    #[prop_field(Styles)]
    styles: Option<Style>,
    #[prop_field(OnEvent)]
    pub on_event: Option<OnEvent>,
}

#[widget]
pub fn UpgradeButton(props: UpgradeButtonProps) {
    let (color, set_color, ..) = use_state!(Color::new(0.0781, 0.0898, 0.101, 1.0));

    let base_styles = props.styles.clone().unwrap_or_default();
    props.styles = Some(Style {
        render_command: StyleProp::Value(RenderCommand::Layout),
        height: StyleProp::Value(Units::Pixels(32.0)),
        width: StyleProp::Value(Units::Pixels(80.0)),
        right: StyleProp::Value(Units::Stretch(1.0)),
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
            <Text content={"UPGRADE".to_string()} size={16.0} styles={text_styles} />
        </Background>
    }
}

#[derive(WidgetProps, Clone, Debug, Default, PartialEq)]
pub struct UIUpgradeLevelProps {
    pub upgrade_level: UpgradeLevel,
}

#[widget]
pub fn UIUpgradeLevel(props: UIUpgradeLevelProps) {
    let UIUpgradeLevelProps { upgrade_level } = props.clone();

    rsx! {
        <>
            // Upgrade Type
            <Text content={format!("Level: {}", upgrade_level.as_u8())} />
            <UIUpgradeLevelIndicator level={upgrade_level.as_u8()} />


            // Requires Components
            // Perform Upgrade Button
        </>
    }
}

#[derive(WidgetProps, Clone, Debug, Default, PartialEq)]
pub struct UILevelIndicatorProps {
    pub level: u8,
}

#[widget]
pub fn UIUpgradeLevelIndicator(props: UILevelIndicatorProps) {
    let UILevelIndicatorProps { level } = props.clone();

    let container_styles = Some(Style {
        layout_type: StyleProp::Value(LayoutType::Row),
        col_between: StyleProp::Value(Units::Pixels(5.0)),
        ..Default::default()
    });

    let level_block_styles = Some(Style {
        background_color: StyleProp::Value(Color::new(0.0, 1.0, 0.0, 1.0)),
        width: StyleProp::Value(Units::Pixels(10.0)),
        height: StyleProp::Value(Units::Pixels(10.0)),
        ..Default::default()
    });

    rsx! {
        <Element styles={container_styles}>

            {VecTracker::from((0..level).map(|_| {
                constructor! {
                    <Background styles={level_block_styles.clone()} />
                }
            }))}

        </Element>
    }
}
