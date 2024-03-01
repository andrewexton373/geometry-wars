use std::time::Duration;

use bevy::prelude::*;
use ordered_float::OrderedFloat;

use crate::{
    asteroid::components::AsteroidMaterial,
    inventory::components::{Inventory, InventoryItem},
    item_producer::ItemProducer,
    items::{Amount, MetalIngot},
    recipe::Recipe,
    space_station::components::SpaceStation,
};

#[derive(Resource)]
pub struct RefineryTimer(pub Option<Timer>);

// A component you can add to the base station in order to smelt ore.
#[derive(Component, Default, Debug, Clone, PartialEq)]
pub struct Refinery {
    pub recipes: Vec<Recipe>,
    pub currently_processing: Option<Recipe>,
    pub remaining_processing_time: f32,
}

impl ItemProducer for Refinery {
    fn new() -> Self {
        let mut recipes = Vec::new();

        let items_required = vec![InventoryItem::Material(
            AsteroidMaterial::Iron,
            Amount::Weight(OrderedFloat(20.0)),
        )];

        let iron_recipe = Recipe {
            items_required,
            item_created: InventoryItem::Ingot(MetalIngot::IronIngot, Amount::Quantity(1)),
            time_required: 2.0,
        };

        let items_required = vec![InventoryItem::Material(
            AsteroidMaterial::Silver,
            Amount::Weight(OrderedFloat(50.0)),
        )];

        let silver_recipe = Recipe {
            items_required,
            item_created: InventoryItem::Ingot(MetalIngot::SilverIngot, Amount::Quantity(1)),
            time_required: 5.0,
        };

        let items_required = vec![InventoryItem::Material(
            AsteroidMaterial::Gold,
            Amount::Weight(OrderedFloat(100.0)),
        )];

        let gold_recipe = Recipe {
            items_required,
            item_created: InventoryItem::Ingot(MetalIngot::GoldIngot, Amount::Quantity(1)),
            time_required: 10.0,
        };

        recipes.push(iron_recipe);
        recipes.push(silver_recipe);
        recipes.push(gold_recipe);

        Self {
            recipes,
            currently_processing: None,
            remaining_processing_time: 0.0,
        }
    }

    fn recipes(&self) -> Vec<Recipe> {
        self.recipes.clone()
    }

    fn currently_processing(&self) -> Option<Recipe> {
        self.currently_processing.clone()
    }

    fn remaining_processing_percent(&self) -> Option<f32> {
        if let Some(currently_processing) = self.currently_processing.clone() {
            return Some(
                ((currently_processing.time_required - self.remaining_processing_time)
                    / currently_processing.time_required)
                    .clamp(0.0, 1.0),
            );
        }
        None
    }

    fn remaining_processing_time(&self) -> Option<f32> {
        if self.currently_processing.is_none() {
            return None;
        }
        Some(self.remaining_processing_time)
    }
}
pub struct RefineryPlugin;

impl Plugin for RefineryPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SmeltEvent>()
            .insert_resource(RefineryTimer(None))
            .add_systems(
                Update,
                (Self::on_smelt_event, Self::update_refinery_processing),
            );
    }
}

impl RefineryPlugin {
    /// Returns true if the inventory provided has the materials availible to smelt the recipe.
    fn have_materials_to_smelt(inventory: &Inventory, recipe: &Recipe) -> bool {
        for material_needed in recipe.items_required.iter() {
            // FIXME: this fells messy and error prone.. not even sure its right haha... maybe use the macro from discord
            match material_needed {
                InventoryItem::Material(material_needed, weight_needed) => {
                    if let Some(inventory_material) =
                        inventory.items.iter().find_map(|item| match item {
                            InventoryItem::Material(m, _) if *m == *material_needed => Some(item),
                            _ => None,
                        })
                    {
                        if inventory_material.amount() < *weight_needed {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                _ => return false,
            }
        }

        true
    }

    /// If the base_station inventory has the required materials for the recipe,
    /// Start processing the recipe by setting currently processing to the recipe,
    /// and starting a timer.
    fn smelt_materials(
        inventory: Mut<Inventory>,
        recipe: &Recipe,
        mut refinery: Mut<Refinery>,
        timer: &mut ResMut<RefineryTimer>,
    ) {
        if Self::have_materials_to_smelt(inventory.as_ref(), &recipe) {
            println!("We have the materials!");

            // Set currently processing to the recipe, finish processing after the timer.
            refinery.currently_processing = Some(recipe.clone());
            timer.0 = Some(Timer::new(
                Duration::from_secs_f32(recipe.time_required),
                TimerMode::Once,
            ));
        } else {
            println!("We do not have the materials!");
        }
    }

    /// Watch the refinery processing timer,
    /// perfom actions when timer elapses.
    fn update_refinery_processing(
        mut base_station_query: Query<
            (&SpaceStation, &mut Inventory, &mut Refinery),
            With<SpaceStation>,
        >,
        mut timer: ResMut<RefineryTimer>,
        time: Res<Time>,
    ) {
        if let Some(timer) = timer.0.as_mut() {
            let (_base_station, mut inventory, mut refinery) = base_station_query.single_mut();

            timer.tick(time.delta());

            // update processing_time_remaining
            if let Some(currently_processing) = refinery.currently_processing.clone() {
                let remaining_time = currently_processing.time_required - timer.elapsed_secs();
                refinery.remaining_processing_time = remaining_time;
            }

            if timer.just_finished() {
                if let Some(currently_processing) = refinery.currently_processing.clone() {
                    for required_item in currently_processing.items_required.iter() {
                        inventory.remove_from_inventory(required_item);
                    }

                    inventory.add_to_inventory(&currently_processing.item_created);
                }

                refinery.currently_processing = None;
            }
        }
    }

    /// Perfom a smelt action with a recipe provided by the SmeltEvent.
    fn on_smelt_event(
        mut reader: EventReader<SmeltEvent>,
        mut base_station_query: Query<
            (&SpaceStation, &mut Inventory, &mut Refinery),
            With<SpaceStation>,
        >,
        mut refinery_timer: ResMut<RefineryTimer>,
    ) {
        for event in reader.read() {
            println!("Smelt Event Detected!");
            let (_base_station, inventory, refinery) = base_station_query.single_mut();

            let recipe = event.0.clone();
            println!("{:?}", recipe);

            Self::smelt_materials(inventory, &recipe, refinery, &mut refinery_timer);
        }
    }

    pub fn attach_refinery_to_entity(commands: &mut Commands, ent: Entity) {
        commands.entity(ent).insert(Refinery::new());
    }
}

#[derive(Event)]
pub struct SmeltEvent(pub Recipe);
