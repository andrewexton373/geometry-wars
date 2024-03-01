use bevy::prelude::*;
use bevy_egui::{
    egui::{Align2, Vec2, Window},
    EguiContexts,
};

use crate::{
    player::components::Player,
    upgrades::{components::UpgradesComponent, events::UpgradeEvent},
};

pub fn ui_space_station_menu(
    mut ctx: EguiContexts,
    // cc_res: Res<ContextClues>,
    player_query: Query<(&Player, &UpgradesComponent)>,
    mut upgrade_events: EventWriter<UpgradeEvent>,
) {
    Window::new("Space Station Information")
        .anchor(Align2::RIGHT_BOTTOM, Vec2 { x: 0.0, y: 0.0 })
        .show(ctx.ctx_mut(), |ui| {
            ui.group(|ui| {
                let (_, upgrades) = player_query.single();

                ui.heading("Ship Upgrades:");

                for upgrade in &upgrades.upgrades {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.label(format!("{:?}", upgrade));
                                if ui.button("Upgrade").clicked() {
                                    upgrade_events.send(UpgradeEvent(*upgrade));
                                }
                            });

                            ui.vertical(|ui| {
                                ui.label("Requires: ");
                                if upgrade.requirements().is_some() {
                                    for requirement in upgrade.requirements().unwrap().requirements
                                    {
                                        // TODO: This seems unnecessairly convoluted..
                                        ui.label(format!("{:?}", requirement));
                                    }
                                }
                            });
                        });
                    });
                }
            });
        });
}
