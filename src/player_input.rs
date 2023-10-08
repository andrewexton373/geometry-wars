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
            .add_system(Self::update_mouse_position_resource)
            .add_system(Self::scroll_events);
    }
}

impl PlayerInputPlugin {
    pub fn update_mouse_position_resource(
        mut mouse_position: ResMut<MousePostion>,
        window_query: Query<&Window, With<PrimaryWindow>>,
        q_camera: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    ) {
        let wnd = window_query.single();
        let (camera, camera_transform) = q_camera.single();

        if let Some(screen_pos) = wnd.cursor_position() {
            let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

            // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
            let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

            // matrix for undoing the projection and camera transform
            let ndc_to_world =
                camera_transform.compute_matrix() * camera.projection_matrix().inverse();

            // use it to convert ndc to world-space coordinates
            let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
            let cursor_pos: Vec2 = world_pos.truncate().clone();

            *mouse_position = MousePostion(cursor_pos);
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
