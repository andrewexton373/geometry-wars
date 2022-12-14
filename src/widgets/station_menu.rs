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
        layout_type: StyleProp::Value(LayoutType::Column),

        left: StyleProp::Value(Units::Percentage(20.0)),
        right: StyleProp::Value(Units::Percentage(20.0)),
        top: StyleProp::Value(Units::Percentage(20.0)),
        bottom: StyleProp::Value(Units::Percentage(20.0)),

        width: StyleProp::Value(Units::Percentage(60.0)),
        height: StyleProp::Value(Units::Percentage(60.0)),
        padding: StyleProp::Value(Edge::all(Units::Pixels(10.0))),
        background_color: StyleProp::Value(Color::new(0.4, 0.4, 0.4, 0.6)),
        ..Default::default()
    });

    let columns = Some(Style {
        layout_type: StyleProp::Value(LayoutType::Row),
        col_between: StyleProp::Value(Units::Pixels(8.0)),
        width: StyleProp::Value(Units::Percentage(100.0)),
        height: StyleProp::Value(Units::Percentage(100.0)),
        left: StyleProp::Value(Units::Auto),
        right: StyleProp::Value(Units::Auto),
        ..Default::default()
    });

    let fill_vertical = Some(Style {
        height: StyleProp::Value(Units::Stretch(1.0)),
        width: StyleProp::Value(Units::Stretch(1.0)),
        left: StyleProp::Value(Units::Auto),
        right: StyleProp::Value(Units::Auto),
        // padding: StyleProp::Value(Edge::all(Units::Pixels(8.0))),
        ..Default::default()
    });

    let on_menu_button_event = Some(OnEvent::new(move |_ctx, event| match event.event_type {
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
                            <Element styles={columns}>
                                <Element styles={fill_vertical}>
                                    <UIUpgradesMenu upgrades={upgrades}/>
                                </Element>

                                <Element styles={fill_vertical}>
                                    <UICraftingTabsView />
                                </Element>

                                <Element styles={fill_vertical}>
                                    <UIBaseInventory />
                                </Element>
                            </Element>
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
        left: StyleProp::Value(Units::Percentage(0.0)),
        top: StyleProp::Value(Units::Stretch(1.0)),
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

    let styles = Some(Style {
        background_color: StyleProp::Value(Color::new(0.4, 0.4, 0.4, 1.0)),
        height: StyleProp::Value(Units::Percentage(100.0)),
        ..Default::default()
    });

    rsx! {
        <Background styles={styles}>
            {VecTracker::from(upgrades.clone().into_iter().enumerate().map(|(_index, upgrade)| {
                constructor! {
                    <UIUpgrade upgrade_type={Some(upgrade)} />
                }
            }))}
        </Background>
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

#[derive(Debug, Clone, PartialEq, Inspectable)]
pub struct UpgradeRequirements {
    pub requirements: Vec<InventoryItem>,
}

#[derive(FromRepr, EnumIter, Debug, Clone, Copy, Default, PartialEq, Inspectable)]
#[repr(u8)]
pub enum UpgradeLevel {
    #[default]
    Level0 = 0,
    Level1 = 1,
    Level2 = 2,
    Level3 = 3,
    Level4 = 4,
    MaxLevel = 5,
}

impl UpgradeLevel {
    pub fn as_u8(&self) -> u8 {
        match self {
            UpgradeLevel::Level0 => 0,
            UpgradeLevel::Level1 => 1,
            UpgradeLevel::Level2 => 2,
            UpgradeLevel::Level3 => 3,
            UpgradeLevel::Level4 => 4,
            UpgradeLevel::MaxLevel => 5,
        }
    }

    pub fn next(&self) -> Option<UpgradeLevel> {
        let current_lvl = self.as_u8();
        UpgradeLevel::from_repr(current_lvl + 1)
    }
}

#[derive(Default, EnumIter, Clone, Copy, Debug, PartialEq, Inspectable)]
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

impl UpgradeType {
    pub fn requirements(&self) -> Option<UpgradeRequirements> {
        let mut requirements = vec![];

        match self {
            UpgradeType::None => {
                return None;
            }
            UpgradeType::Health(level) => {
                requirements = match level {
                    UpgradeLevel::Level0 => vec![],
                    UpgradeLevel::Level1 => vec![
                        InventoryItem::Component(UpgradeComponent::Cog, Amount::Quantity(1)),
                        InventoryItem::Component(UpgradeComponent::IronPlate, Amount::Quantity(2)),
                    ],
                    UpgradeLevel::Level2 => vec![
                        InventoryItem::Component(UpgradeComponent::Cog, Amount::Quantity(2)),
                        InventoryItem::Component(UpgradeComponent::IronPlate, Amount::Quantity(3)),
                    ],
                    UpgradeLevel::Level3 => vec![
                        InventoryItem::Component(UpgradeComponent::Cog, Amount::Quantity(1)),
                        InventoryItem::Component(UpgradeComponent::IronPlate, Amount::Quantity(2)),
                        InventoryItem::Component(
                            UpgradeComponent::SilverConduit,
                            Amount::Quantity(1),
                        ),
                    ],
                    UpgradeLevel::Level4 => vec![
                        InventoryItem::Component(UpgradeComponent::Cog, Amount::Quantity(3)),
                        InventoryItem::Component(UpgradeComponent::IronPlate, Amount::Quantity(5)),
                        InventoryItem::Component(
                            UpgradeComponent::SilverConduit,
                            Amount::Quantity(3),
                        ),
                        InventoryItem::Component(UpgradeComponent::GoldLeaf, Amount::Quantity(1)),
                    ],
                    UpgradeLevel::MaxLevel => vec![
                        InventoryItem::Component(UpgradeComponent::Cog, Amount::Quantity(10)),
                        InventoryItem::Component(UpgradeComponent::IronPlate, Amount::Quantity(5)),
                        InventoryItem::Component(
                            UpgradeComponent::SilverConduit,
                            Amount::Quantity(5),
                        ),
                        InventoryItem::Component(UpgradeComponent::GoldLeaf, Amount::Quantity(3)),
                    ],
                }
            }
            UpgradeType::ShipCargoBay(level) => {
                requirements = match level {
                    UpgradeLevel::Level0 => vec![],
                    UpgradeLevel::Level1 => vec![
                        InventoryItem::Component(UpgradeComponent::Cog, Amount::Quantity(2)),
                        InventoryItem::Component(UpgradeComponent::IronPlate, Amount::Quantity(3)),
                    ],
                    UpgradeLevel::Level2 => todo!(),
                    UpgradeLevel::Level3 => todo!(),
                    UpgradeLevel::Level4 => todo!(),
                    UpgradeLevel::MaxLevel => todo!(),
                }
            }
        }

        return Some(UpgradeRequirements { requirements });
    }

    pub fn next(&self) -> Self {
        match self {
            UpgradeType::None => {
                return self.clone();
            }
            UpgradeType::Health(level) => {
                if let Some(next_level) = level.next() {
                    UpgradeType::Health(next_level)
                } else {
                    self.clone()
                }
            }
            UpgradeType::ShipCargoBay(level) => {
                if let Some(next_level) = level.next() {
                    UpgradeType::ShipCargoBay(next_level)
                } else {
                    self.clone()
                }
            }
        }
    }
}

pub struct UpgradeEvent(pub UpgradeType);

#[widget]
pub fn UIUpgrade(props: UIUpgradeProps) {
    let UIUpgradeProps {
        styles: _,
        on_event: _,
        upgrade_type,
    } = props.clone();

    let styles = Some(Style {
        render_command: StyleProp::Value(RenderCommand::Quad),
        height: StyleProp::Value(Units::Auto),
        layout_type: StyleProp::Value(LayoutType::Column),
        padding: StyleProp::Value(Edge::all(Units::Pixels(8.0))),
        ..Default::default()
    });

    let bg_color = Some(Style{
        background_color: StyleProp::Value(Color::new(0.176, 0.196, 0.215, 1.0)),
        height: StyleProp::Value(Units::Auto),
        padding: StyleProp::Value(Edge::all(Units::Pixels(8.0))),
        ..Default::default()
    });


    let cloned = upgrade_type.clone();

    let on_event = OnEvent::new(move |ctx, event| match event.event_type {
        EventType::Click(..) => {
            println!("UPGRADE BUTTON CLICKED! {:?}", cloned);
            ctx.query_world::<EventWriter<UpgradeEvent>, _, ()>(|mut writer| {
                writer.send(UpgradeEvent(cloned.clone().unwrap()));
            });
        }
        _ => (),
    });

    if let Some(upgrade_type) = upgrade_type.clone() {
        match upgrade_type.clone() {
            upgrade_type @ (UpgradeType::Health(level) | UpgradeType::ShipCargoBay(level)) => {
                let next_upgrade_level = upgrade_type.next();

                rsx! {
                    <Element styles={styles}>
                        <Background styles={bg_color}>
                            <Text content={format!("Upgrade Type: {}", upgrade_type.clone().to_string())} />
                            <UIRequirements requirements={next_upgrade_level.requirements().clone()} />
                            <UpgradeButton on_event={Some(on_event)} />

                            <UIUpgradeLevel upgrade_level={level.clone()} />
                        </Background>
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
