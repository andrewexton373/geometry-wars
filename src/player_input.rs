use crate::events::EnginePowerEvent;
use crate::GameCamera;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::{
    input::mouse::MouseWheel,
    prelude::{EventReader, EventWriter, Plugin},
};

pub struct PlayerInputPlugin;

#[derive(Resource)]
pub struct MouseWorldPosition(pub(crate) Vec2);

#[derive(Resource)]
pub struct MouseScreenPosition(pub(crate) Vec2);

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<EnginePowerEvent>()
            .insert_resource(MouseWorldPosition(Vec2::ZERO))
            .insert_resource(MouseScreenPosition(Vec2::ZERO))
            .add_systems(Update, (
                Self::update_mouse_world_position_resource,
                Self::update_mouse_screen_position_resource,
                Self::scroll_events
            ));
    }
}

impl PlayerInputPlugin {
    pub fn update_mouse_world_position_resource(
        mut mouse_position: ResMut<MouseWorldPosition>,
        window_query: Query<&Window, With<PrimaryWindow>>,
        _cursor_event: EventReader<CursorMoved>,
        q_camera: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    ) {
        let window = window_query.single();

        let (camera, camera_transform) = q_camera.single();

        if let Some(world_position) = window.cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            *mouse_position = MouseWorldPosition(world_position)
        }
    }

    pub fn update_mouse_screen_position_resource(
        mut mouse_position: ResMut<MouseScreenPosition>,
        window_query: Query<&Window, With<PrimaryWindow>>,
        _cursor_event: EventReader<CursorMoved>,
    ) {
        let window = window_query.single();
        if let Some(pos) = window.cursor_position() {
            *mouse_position = MouseScreenPosition(pos);
        }
    }

    pub fn scroll_events(
        mut scroll_events: EventReader<MouseWheel>,
        mut engine_events: EventWriter<EnginePowerEvent>,
    ) {
        use bevy::input::mouse::MouseScrollUnit;

        for event in scroll_events.read() {
            match event.unit {
                MouseScrollUnit::Line => {
                    engine_events.send(EnginePowerEvent(event.y));
                }
                MouseScrollUnit::Pixel => {
                    engine_events.send(EnginePowerEvent(event.y));
                }
            }
        }
    }
}
