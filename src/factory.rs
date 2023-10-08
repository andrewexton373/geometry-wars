use std::time::Duration;

use bevy::prelude::*;

use crate::events::CraftEvent;
use crate::{
    base_station::BaseStation,
    inventory::{Amount, Inventory, InventoryItem},
    item_producer::ItemProducer,
    recipe::Recipe,
    refinery::MetalIngot,
    // widgets::factory::CraftEvent,
};

#[derive(Default, Debug, Clone, Copy, PartialEq, Hash)]
pub enum UpgradeComponent {
    #[default]
    Cog,
    IronPlate,
    SilverConduit,
    GoldLeaf,
}

#[derive(Resource)]

pub struct FactoryTimer(pub Option<Timer>);

// A component you can add to the base station in order to smelt ore.
#[derive(Component, Default, Debug, Clone, PartialEq)]
pub struct Factory {
    pub recipes: Vec<Recipe>,
    pub currently_processing: Option<Recipe>,
    pub remaining_processing_time: f32,
}

impl ItemProducer for Factory {
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

    fn recipes(&self) -> Vec<Recipe> {
        self.recipes.clone()
    }

    fn currently_processing(&self) -> Option<Recipe> {
        self.currently_processing.clone()
    }

    fn remaining_processing_time(&self) -> Option<f32> {
        if self.currently_processing.is_none() {
            return None;
        };
        Some(self.remaining_processing_time)
    }

    fn new() -> Self {
        let mut recipes = Vec::new();

        let mut items_required = Vec::new();
        items_required.push(InventoryItem::Ingot(
            MetalIngot::IronIngot,
            Amount::Quantity(2),
        ));

        let cog_recipe = Recipe {
            items_required,
            item_created: InventoryItem::Component(UpgradeComponent::Cog, Amount::Quantity(1)),
            time_required: 4.0,
        };

        let mut items_required = Vec::new();
        items_required.push(InventoryItem::Ingot(
            MetalIngot::IronIngot,
            Amount::Quantity(5),
        ));

        let iron_plate_recipe = Recipe {
            items_required,
            item_created: InventoryItem::Component(
                UpgradeComponent::IronPlate,
                Amount::Quantity(1),
            ),
            time_required: 10.0,
        };

        let mut items_required = Vec::new();
        items_required.push(InventoryItem::Ingot(
            MetalIngot::SilverIngot,
            Amount::Quantity(1),
        ));

        let silver_conduit_recipe = Recipe {
            items_required,
            item_created: InventoryItem::Component(
                UpgradeComponent::SilverConduit,
                Amount::Quantity(1),
            ),
            time_required: 8.0,
        };

        let mut items_required = Vec::new();
        items_required.push(InventoryItem::Ingot(
            MetalIngot::GoldIngot,
            Amount::Quantity(1),
        ));

        let gold_leaf_recipe = Recipe {
            items_required,
            item_created: InventoryItem::Component(UpgradeComponent::GoldLeaf, Amount::Quantity(1)),
            time_required: 15.0,
        };

        recipes.push(cog_recipe);
        recipes.push(iron_plate_recipe);
        recipes.push(silver_conduit_recipe);
        recipes.push(gold_leaf_recipe);

        Self {
            recipes,
            currently_processing: None,
            remaining_processing_time: 0.0,
        }
    }
}

pub struct FactoryPlugin;

impl Plugin for FactoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CraftEvent>()
            .insert_resource(FactoryTimer(None))
            .add_system(Self::on_craft_event)
            .add_system(Self::update_factory_processing);
    }
}

impl FactoryPlugin {
    /// Returns true if the inventory provided has the materials availible to smelt the recipe.
    fn have_materials_to_craft(inventory: &Inventory, recipe: &Recipe) -> bool {
        for material_needed in recipe.items_required.iter() {
            // FIXME: this fells messy and error prone.. not even sure its right haha... maybe use the macro from discord
            match material_needed {
                InventoryItem::Ingot(material_needed, quantity_needed) => {
                    if let Some(inventory_material) =
                        inventory.items.iter().find_map(|item| match item {
                            InventoryItem::Ingot(m, _) if *m == *material_needed => Some(item),
                            _ => None,
                        })
                    {
                        if inventory_material.amount() < *quantity_needed {
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
    fn craft_from_materials(
        inventory: Mut<Inventory>,
        recipe: &Recipe,
        mut factory: Mut<Factory>,
        timer: &mut ResMut<FactoryTimer>,
    ) {
        if Self::have_materials_to_craft(inventory.as_ref(), &recipe) {
            println!("We have the materials!");

            // Set currently processing to the recipe, finish processing after the timer.
            factory.currently_processing = Some(recipe.clone());
            timer.0 = Some(Timer::new(
                Duration::from_secs_f32(recipe.time_required),
                TimerMode::Once,
            ));
        } else {
            println!("We do not have the materials!");
        }
    }

    /// Watch the factory processing timer,
    /// perfom actions when timer elapses.
    fn update_factory_processing(
        mut base_station_query: Query<
            (&BaseStation, &mut Inventory, &mut Factory),
            With<BaseStation>,
        >,
        mut timer: ResMut<FactoryTimer>,
        time: Res<Time>,
    ) {
        if let Some(timer) = timer.0.as_mut() {
            let (_base_station, mut inventory, mut factory) = base_station_query.single_mut();

            timer.tick(time.delta());

            // update processing_time_remaining
            if let Some(currently_processing) = factory.currently_processing.clone() {
                let remaining_time = currently_processing.time_required - timer.elapsed_secs();
                factory.remaining_processing_time = remaining_time;
            }

            if timer.just_finished() {
                if let Some(currently_processing) = factory.currently_processing.clone() {
                    for required_item in currently_processing.items_required.iter() {
                        inventory.remove_from_inventory(required_item);
                    }

                    inventory.add_to_inventory(&currently_processing.item_created);
                }

                factory.currently_processing = None;
            }
        }
    }

    /// Perfom a smelt action with a recipe provided by the SmeltEvent.
    fn on_craft_event(
        mut reader: EventReader<CraftEvent>,
        mut base_station_query: Query<
            (&BaseStation, &mut Inventory, &mut Factory),
            With<BaseStation>,
        >,
        mut factory_timer: ResMut<FactoryTimer>,
    ) {
        for event in reader.iter() {
            println!("Craft Event Detected!");
            let (_base_station, inventory, factory) = base_station_query.single_mut();

            let recipe = event.0.clone();
            println!("{:?}", recipe);

            Self::craft_from_materials(inventory, &recipe, factory, &mut factory_timer);
        }
    }

    pub fn attach_factory_to_entity(commands: &mut Commands, ent: Entity) {
        commands.entity(ent).insert(Factory::new());
    }
}
