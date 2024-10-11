use bevy::{
    asset::AssetServer, color::{palettes::css::RED, Color, Srgba}, ecs::{
        event::EventReader,
        system::{Commands, Res},
    }, hierarchy::DespawnRecursiveExt, text::{Text, Text2dBundle, TextStyle}, transform, utils::default
};
use bevy_tweening::{
    lens::TextColorLens, Animator, EaseFunction, RepeatCount, Tween, TweenCompleted,
};

use super::events::DamageIndicatorEvent;

pub fn damage_indicator_events(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut events: EventReader<DamageIndicatorEvent>,
) {
    for evt in events.read() {
        let font = asset_server.load("fonts/FiraMono-Regular.ttf");

        let damage_amount = evt.damage;
        let damage_text = format!("-{}HP", damage_amount);
        let transform = evt.traslation;

        let tween = Tween::new(
            EaseFunction::ExponentialInOut,
            std::time::Duration::from_millis(3000),
            TextColorLens {
                start: Color::from(Srgba {
                    red: 255.0,
                    green: 0.0,
                    blue: 0.0,
                    alpha: 1.0,
                }),
                end: Color::from(Srgba {
                    red: 255.0,
                    green: 0.0,
                    blue: 0.0,
                    alpha: 0.0,
                }),
                section: 0,
            },
        )
        .with_repeat_count(RepeatCount::Finite(1))
        .with_completed_event(111);

        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    damage_text,
                    TextStyle {
                        font: font.clone(),
                        font_size: 32.0,
                        color: Color::from(RED),
                    },
                ),
                transform,
                ..default()
            },
            Animator::new(tween),
        ));
    }
}

pub fn remove_post_animation_text(
    mut commands: Commands,
    mut tween_completed: EventReader<TweenCompleted>,
) {
    for evt in tween_completed.read() {
        if evt.user_data == 111 {
            commands.entity(evt.entity).despawn_recursive();
        }
    }
}
