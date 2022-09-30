#![feature(array_methods)]


use bevy_stat_bars::*;
use bevy::{prelude::*, diagnostic::{FrameTimeDiagnosticsPlugin, Diagnostics}};
use bevy_debug_text_overlay::{screen_print, OverlayPlugin};
use bevy_rapier2d::{prelude::*, parry::simba::scalar::SupersetOf};
use bevy_prototype_lyon::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

use kayak_ui::core::{Color as KayakColor, VecTracker, constructor, Binding, Bound, MutableBound};

use kayak_ui::bevy::{BevyContext, BevyKayakUIPlugin, FontMapping, UICameraBundle};
use kayak_ui::core::{
    render, rsx,
    styles::{Style as KayakStyle, StyleProp, Units},
    widget,
    use_state,
    bind
};
use kayak_ui::widgets::{App as KayakApp, OnChange, SpinBox, SpinBoxStyle, Text, TextBox, Window, Element};

mod player;
use player::{ PlayerPlugin, Player, Inventory, ItemAndWeight };

mod astroid;
use astroid::{AstroidPlugin, AstroidMaterial};

mod projectile;
use projectile::{ProjectilePlugin};

mod crosshair;
use crosshair::CrosshairPlugin;

mod healthbar;
use healthbar::HealthBarPlugin;

mod base_station;
use base_station::BaseStationPlugin;

// Defines the amount of time that should elapse between each physics step.
const TIME_STEP: f32 = 1.0 / 60.0;

pub const PIXELS_PER_METER : f32 = 10.0;

const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);

#[derive(Component)]
struct Collider;

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
struct FpsText;

#[derive(Component)]
pub struct GameCamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(ShapePlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(BaseStationPlugin)
        .add_plugin(AstroidPlugin)
        .add_plugin(ProjectilePlugin)
        .add_plugin(CrosshairPlugin)
        .add_plugin(StatBarsPlugin)
        .add_plugin(HealthBarPlugin)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_system(camera_follows_player)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(PIXELS_PER_METER))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(OverlayPlugin { font_size: 18.0, ..default() })
        .add_plugin(BevyKayakUIPlugin)
        // .add_system(screen_print_debug_text)
        // .add_system(update_inventory_ui)
        .run();
}

#[derive(Component)]
pub struct InventoryText;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct UIItems {
    pub inventory_items: [Option<ItemAndWeight>; 20],
}

// fn update_inventory_ui(
//     inventory_query: Query<
//         &Inventory,
//         Changed<Inventory>
//     >,
//     ui_items: Res<Binding<UIItems>>,
// ) {

//     if let inventory = inventory_query.get_single().unwrap() {
//         println!("SET INVENTORY UI");

//         // get inventory items for ui
//         // let inventory_items: Vec<ItemAndWeight> = inventory.items.into_iter().filter(|item| item.is_some()).map().collect::<Vec<ItemAndWeight>>().clone();
//             // .into_iter()
//             // .filter(|ic| ic.item != ItemType::None)
//             // .collect();

//         println!("SET INVENTORY UI");

//         // update ui by updating binding object
//         ui_items.set(UIItems {
//             inventory_items: inventory.items
//         });
//     }
// }

#[widget]
fn UIInventory() {
    // let (values, set_value, _) = use_state!(vec![]);
    // let (empty_value, set_empty_value, _) = use_state!("".to_string());

    // let input_styles = Style {
    //     top: StyleProp::Value(Units::Pixels(10.0)),
    //     ..Default::default()
    // };

    let inventory = context.query_world::<Res<Binding<Inventory>>, _, _>(move |inventory| inventory.clone());
    // let ui_items = context.query_world::<Res<Binding<UIItems>>, _, _>(move |ui_items| ui_items.clone());
    context.bind(&inventory);

    let inventory_items = inventory.get().items;

    // info!("{:?}", inventory_items);


    let data = vec![
        "Text 1", "Text 2", "Text 3", "Text 4", "Text 5", "Text 6", "Text 7", "Text 8",
        "Text 9", "Text 10",
    ];
    
    rsx! {
        <Window position={(1080.0, 0.0)} size={(200.0, 300.0)} title={"Inventory".to_string()}>
            <Element>
                {VecTracker::from(inventory_items.iter().filter(|item| item.is_some()).map(|item| {
                    constructor! {
                        <Text content={format!("{:?}", item.clone())} size={16.0} />
                    }
                }))}
            </Element>
        </Window>
    }
}


fn setup(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
    asset_server: Res<AssetServer>,
    mut font_mapping: ResMut<FontMapping>,
) {
    let ui_camera = commands.spawn_bundle(UICameraBundle::new())
                                    .insert(Name::new("UICamera"));

    font_mapping.set_default(asset_server.load("roboto.kayak_font"));
    commands.insert_resource(bind(UIItems::default()));

    let context = BevyContext::new(|context| {
        render! {
            <KayakApp>
                <UIInventory />
            </KayakApp>
        }
    });

    commands.insert_resource(context);
    
    let game_camera = commands.spawn_bundle(Camera2dBundle::default())
                            .insert(GameCamera)
                            .insert(Name::new("GameCamera"))
                            .id();

    HealthBarPlugin::attach_player_health_bar(&mut commands, game_camera);
    rapier_config.gravity = Vec2::new(0.0, 0.0);

    commands.insert_resource(bind(Inventory {items: [None; 20]}));
}

fn camera_follows_player(
    mut camera_query: Query<(&Camera, &mut GlobalTransform), With<GameCamera>>,
    player_query: Query<&Transform, (With<Player>, Without<GameCamera>)>,
){
    let (_camera, mut camera_trans) = camera_query.single_mut().into();
    let player_trans = player_query.single();

        // TODO: seems sloppy, is there another way?
        let player_to_camera = camera_trans.translation() - player_trans.translation;
        let mut_trans = camera_trans.translation_mut();
        mut_trans.x -= player_to_camera.x;
        mut_trans.y -= player_to_camera.y;
}

// fn screen_print_debug_text(
//     diagnostics: Res<Diagnostics>,
//     player_query: Query<&Player>
// ) {
//     if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
//         if let Some(average) = fps.average() {
//             // Update the value of the second section
//             screen_print!(col: Color::WHITE, "fps: {average}");
//         }
//     }

//     let player = player_query.single();
//     let inventory = &player.inventory;
//     screen_print!(col: Color::LIME_GREEN, "inventory: {inventory:?}");
// }
