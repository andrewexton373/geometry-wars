use bevy_stat_bars::*;
use bevy::{prelude::*, diagnostic::{FrameTimeDiagnosticsPlugin, Diagnostics}};
use bevy_debug_text_overlay::{screen_print, OverlayPlugin};
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

use kayak_ui::core::Color as KayakColor;

use kayak_ui::bevy::{BevyContext, BevyKayakUIPlugin, FontMapping, UICameraBundle};
use kayak_ui::core::{
    render, rsx,
    styles::{Style, StyleProp, Units},
    widget,
    use_state
};
use kayak_ui::widgets::{App as KayakApp, OnChange, SpinBox, SpinBoxStyle, TextBox, Window};

mod player;
use player::{ PlayerPlugin, Player };

mod astroid;
use astroid::{AstroidPlugin};

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
        .add_system(screen_print_debug_text)
        .run();
}

#[derive(Component)]
pub struct InventoryText;


#[widget]
fn TextBoxExample() {
    let (value, set_value, _) = use_state!("I started with a value!".to_string());
    let (empty_value, set_empty_value, _) = use_state!("".to_string());
    let (red_value, set_red_value, _) = use_state!("This text is red".to_string());
    let (spin_value, set_spin_value, _) = use_state!("3".to_string());

    let input_styles = Style {
        top: StyleProp::Value(Units::Pixels(10.0)),
        ..Default::default()
    };

    let red_text_styles = Style {
        color: StyleProp::Value(KayakColor::new(1., 0., 0., 1.)),
        ..input_styles.clone()
    };

    let on_change = OnChange::new(move |event| {
        set_value(event.value);
    });

    let on_change_empty = OnChange::new(move |event| {
        set_empty_value(event.value);
    });

    let on_change_red = OnChange::new(move |event| {
        set_red_value(event.value);
    });

    let on_change_spin = OnChange::new(move |event| {
        set_spin_value(event.value);
    });

    let vert = SpinBoxStyle::Vertical;

    rsx! {
        <Window position={(50.0, 50.0)} size={(500.0, 300.0)} title={"TextBox Example".to_string()}>
            <TextBox styles={Some(input_styles)} value={value} on_change={Some(on_change)} />
            <TextBox
                styles={Some(input_styles)}
                value={empty_value}
                on_change={Some(on_change_empty)}
                placeholder={Some("This is a placeholder".to_string())}
            />
            <TextBox styles={Some(red_text_styles)} value={red_value} on_change={Some(on_change_red)} />
            <SpinBox
                styles={Some(input_styles)}
                value={spin_value}
                on_change={Some(on_change_spin)}
                min_val={0.0}
                max_val={10.0}
            />
            <SpinBox
                spin_button_style={vert}
                styles={Some(input_styles)}
                value={spin_value}
                on_change={Some(on_change_spin)}
                min_val={0.0}
                max_val={10.0}
            />
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

    let context = BevyContext::new(|context| {
        render! {
            <KayakApp>
                <TextBoxExample />
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

fn screen_print_debug_text(
    diagnostics: Res<Diagnostics>,
    player_query: Query<&Player>
) {
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(average) = fps.average() {
            // Update the value of the second section
            screen_print!(col: Color::WHITE, "fps: {average}");
        }
    }

    let player = player_query.single();
    let inventory = &player.inventory;
    screen_print!(col: Color::LIME_GREEN, "inventory: {inventory:?}");
}
