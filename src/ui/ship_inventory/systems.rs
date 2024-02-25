use bevy::ecs::system::Query;
use bevy_egui::{
    egui::{Align2, Window},
    EguiContexts,
};

use crate::{inventory::components::Inventory, player::components::Player, ui::helpers::progress_string};

pub fn ui_ship_inventory(
    // world: &mut World,
    mut ctx: EguiContexts,
    mut inventory_query: Query<(&Player, &mut Inventory)>,
) {
    Window::new("Ship Inventory")
        .auto_sized()
        .title_bar(false)
        .resizable(false)
        .anchor(
            Align2::LEFT_BOTTOM,
            bevy_inspector_egui::egui::Vec2 { x: 0.0, y: 0.0 },
        )
        .show(ctx.ctx_mut(), |ui| {
            let (_, inventory) = inventory_query.single_mut();

            let inventory_capacity_percent =
                (1.0 - inventory.remaining_capacity().0 / inventory.capacity.maximum.0) * 100.0;
            ui.label(format!("Capacity: {:.2}%", inventory_capacity_percent));
            ui.label(progress_string(inventory_capacity_percent / 100.0));

            ui.group(|ui| {
                for item in inventory.items.clone() {
                    ui.label(format!("{:?}", item));
                }
            })
        });
}
