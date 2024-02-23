use bevy::{
    core::Name,
    ecs::{entity::Entity, query::With, system::Query},
    render::camera::Camera,
    transform::components::GlobalTransform,
};
use bevy_egui::{
    egui::{Align2, Window},
    EguiContexts,
};
use bevy_xpbd_2d::plugins::spatial_query::{SpatialQuery, SpatialQueryFilter};

use crate::{asteroid::components::Asteroid, ui::helpers::progress_string, GameCamera};

pub fn ui_mouse_hover_context(
    mut ctx: EguiContexts,
    window_query: Query<&bevy::window::Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    ent_query: Query<(Entity, &Name, Option<&Asteroid>)>,
    spatial_q: SpatialQuery,
) {
    let window = window_query.single();
    let (camera, camera_transform) = camera_q.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        Window::new("Mouse Context")
            .anchor(
                Align2::CENTER_TOP,
                bevy_inspector_egui::egui::Vec2 { x: 0.0, y: 0.0 },
            )
            .show(ctx.ctx_mut(), |ui| {
                ui.group(|ui| {
                    ui.label(format!(
                        "X:{:.2} Y:{:.2}",
                        world_position.x, world_position.y
                    ));
                });

                // Raycast Mouse Position Into Viewport
                if let Some(ray_hit) = spatial_q.cast_ray(
                    world_position,
                    bevy::prelude::Vec2::Y,
                    0.001,
                    true,
                    SpatialQueryFilter::default(),
                ) {
                    if let Ok((_ent, name, asteroid)) = ent_query.get(ray_hit.entity) {
                        ui.group(|ui| {
                            ui.heading(format!("{}", name));

                            if let Some(asteroid) = asteroid {
                                ui.label(format!(
                                    "Health: {:.2}%",
                                    asteroid.health.current_percent() * 100.0
                                ));
                                let health_percent = asteroid.health.current_percent();
                                ui.label(progress_string(health_percent));

                                ui.label("Composition:");
                                ui.label(format!("{:?}", asteroid.composition));
                            }
                        });
                    };
                }
            });
    };
}
