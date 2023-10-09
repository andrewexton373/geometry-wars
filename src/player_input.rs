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
pub struct MousePostion(pub(crate) Vec2);

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<EnginePowerEvent>()
            .insert_resource(MousePostion(Vec2::ZERO))
            .add_systems(Update, (
                Self::update_mouse_position_resource,
                Self::scroll_events
            ));
    }
}

impl PlayerInputPlugin {
    pub fn update_mouse_position_resource(
        mut mouse_position: ResMut<MousePostion>,
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
            *mouse_position = MousePostion(world_position)
        }
    }

    pub fn scroll_events(
        mut scroll_events: EventReader<MouseWheel>,
        mut engine_events: EventWriter<EnginePowerEvent>,
    ) {
        use bevy::input::mouse::MouseScrollUnit;

        for event in scroll_events.iter() {
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
