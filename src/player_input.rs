use bevy::{prelude::{Plugin, EventReader, EventWriter}, input::mouse::MouseWheel};

pub struct PlayerInputPlugin;

pub struct EnginePowerEvent(pub f32);


impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_event::<EnginePowerEvent>()
            .add_system(Self::scroll_events);
    }
}

impl PlayerInputPlugin {
    pub fn scroll_events(
        mut scroll_events: EventReader<MouseWheel>,
        mut engine_events: EventWriter<EnginePowerEvent>
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