use bevy::ecs::system::Query;
use bevy_egui::{
    egui::{Align2, Slider, Vec2, Window},
    EguiContexts,
};
use bevy_xpbd_2d::components::LinearVelocity;

use crate::{player::components::Player, ui::helpers::progress_string};

pub fn ui_ship_information(
    player_query: Query<(&mut Player, &mut LinearVelocity)>,
    mut ctx: EguiContexts,
) {
    let (player, velocity) = player_query.single();

    Window::new("Ship Information")
        .anchor(Align2::LEFT_TOP, Vec2 { x: 0.0, y: 0.0 })
        .show(ctx.ctx_mut(), |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.horizontal(|ui| {
                    ui.group(|ui| {
                        ui.label(format!(
                            "Health: {:.2}%",
                            player.health.current_percent() * 100.0
                        ));
                        let health_percent = player.health.current_percent();
                        ui.label(progress_string(health_percent));
                    });

                    ui.group(|ui| {
                        ui.label(format!("Battery: {:.2}KWh", player.battery.current()));
                        let battery_percent = player.battery.current() / 1000.0;
                        ui.label(progress_string(battery_percent));
                    });
                });

                ui.horizontal(|ui| {
                    ui.add(
                        Slider::new(&mut player.engine.power_level.clone(), 0.0..=100.0)
                            .text("Engine Power"),
                    );
                    ui.label(format!("Speed: {:.2}", velocity.0.length()));

                    let direction_radians = velocity.0.angle_between(hexx::Vec2::X);

                    // TODO: procedure to convert direction_radians to cardinal directions (nice to have)
                    ui.label(format!("Direction: {:.2}", direction_radians));
                });
            });
        });
}
