use avian2d::prelude::{Mass, SpatialQuery, SpatialQueryFilter};
use bevy::{
    core::Name,
    ecs::{
        entity::Entity,
        system::{Query, Res, ResMut},
    },
    math::Dir2,
};
use bevy_egui::{
    egui::{Pos2, Vec2, Window},
    EguiContexts,
};

use crate::{
    asteroid::components::Asteroid,
    health::components::Health,
    player_input::resources::{MouseScreenPosition, MouseWorldPosition},
    space_station::modules::components::SpaceStationModuleType,
    ui::helpers::progress_string,
};

use super::resources::MouseHoverContext;

pub fn update_mouse_hover_context_resource(
    mut mouse_hover_context: ResMut<MouseHoverContext>,
    mouse_world_position: Res<MouseWorldPosition>,
    ent_query: Query<Entity>,
    spatial_q: SpatialQuery,
) {
    // Raycast Mouse Position Into Viewport
    if let Some(ray_hit) = spatial_q.cast_ray(
        mouse_world_position.0.as_dvec2(),
        Dir2::Y,
        0.001,
        true,
        &SpatialQueryFilter::default(),
    ) {
        if let Ok(ent) = ent_query.get(ray_hit.entity) {
            mouse_hover_context.0 = Some(ent);
        } else {
            mouse_hover_context.0 = None;
        };
    } else {
        mouse_hover_context.0 = None;
    }
}

pub fn ui_mouse_hover_context(
    mut ctx: EguiContexts,
    mouse_hover_context: Res<MouseHoverContext>,
    mouse_screen_position: Res<MouseScreenPosition>,
    ent_query: Query<(
        Entity,
        &Name,
        Option<&Asteroid>,
        Option<&Health>,
        Option<&Mass>,
        Option<&SpaceStationModuleType>,
    )>,
) {
    if let Some(hover_context_ent) = mouse_hover_context.0 {
        let offset = Vec2 { x: 8.0, y: 8.0 };

        let screen_pos = Pos2 {
            x: mouse_screen_position.0.x + offset.x,
            y: mouse_screen_position.0.y + offset.y,
        };

        Window::new("Mouse Context")
            .fixed_pos(screen_pos)
            .title_bar(false)
            .resizable(false)
            .show(ctx.ctx_mut(), |ui| {
                if let Ok((_ent, name, asteroid, health, mass, module_type)) =
                    ent_query.get(hover_context_ent)
                {
                    ui.group(|ui| {
                        ui.heading(format!("{}", name));

                        if let Some(health) = health {
                            ui.label(format!("Health: {:.2}%", health.current_percent() * 100.0));

                            ui.label(progress_string(health.current_percent()));
                        }

                        if let Some(asteroid) = asteroid {
                            if let Some(m) = mass {
                                ui.label(format!("Mass: {}Kgs", m.0));
                            }

                            ui.label("Composition:");
                            ui.label(format!("{:?}", asteroid.composition));
                        }

                        if let Some(module_type) = module_type {
                            ui.label(format!("Module{:?}", module_type));
                        }
                    });
                };
            });
    }
}
