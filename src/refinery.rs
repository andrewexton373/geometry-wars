use std::time::Duration;

use bevy::prelude::*;

use crate::{base_station::BaseStation, game_ui_widgets::SmeltEvent, inventory::{Inventory, InventoryItem, Amount}, astroid::AstroidMaterial};

pub struct RefineryTimer(pub Option<Timer>);

// A component you can add to the base station in order to smelt ore.
#[derive(Component, Default, Debug, Clone, PartialEq)]
pub struct Refinery {
    pub recipes: Vec<RefineryRecipe>,
    pub currently_processing: Option<RefineryRecipe>,
}

impl Refinery {
    pub fn new() -> Self {
        let mut recipes = Vec::new();
        
        let mut items_required = Vec::new();
        items_required.push(InventoryItem::Material(AstroidMaterial::Iron, Amount::Weight(20.0)));

        let iron_recipe = RefineryRecipe {
            items_required,
            item_created: MetalIngot::IronIngot
        };

        let mut items_required = Vec::new();
        items_required.push(InventoryItem::Material(AstroidMaterial::Silver, Amount::Weight(50.0)));


        let silver_recipe = RefineryRecipe {
            items_required,
            item_created: MetalIngot::SilverIngot
        };

        let mut items_required = Vec::new();
        items_required.push(InventoryItem::Material(AstroidMaterial::Gold, Amount::Weight(100.0)));


        let gold_recipe = RefineryRecipe {
            items_required,
            item_created: MetalIngot::GoldIngot
        };

        recipes.push(iron_recipe);
        recipes.push(silver_recipe);
        recipes.push(gold_recipe);


        Self {
            recipes,
            currently_processing: None,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct RefineryRecipe {
    pub items_required: Vec<InventoryItem>,
    pub item_created: MetalIngot
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum MetalIngot {
    #[default]
    IronIngot,
    SilverIngot,
    GoldIngot
}
pub struct RefineryPlugin;

impl Plugin for RefineryPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SmeltEvent>()
            .insert_resource(RefineryTimer(None))
            .add_system(Self::on_smelt_event)
            .add_system(Self::update_refinery_processing);
    }
}

impl RefineryPlugin {

     /// Returns true if the inventory provided has the materials availible to smelt the recipe.
     fn have_materials_to_smelt(inventory: &Inventory, recipe: &RefineryRecipe) -> bool {

        for material_needed in recipe.items_required.iter() {

            // FIXME: this fells messy and error prone.. not even sure its right haha... maybe use the macro from discord
            match material_needed {
                InventoryItem::Material(material_needed, weight_needed) => {
                    if let Some(inventory_material) = inventory.items.iter().find_map(|item| {
                        match item {
                            InventoryItem::Material(m, _) if *m == *material_needed => {
                                Some(item)
                            },
                            _ => { None }
                        }
                    }) {
                        if inventory_material.amount() < *weight_needed {
                            return false;
                        }
                    } else {
                        return false;
                    }

                },
                _ => { return false },
            }

        }
    
        true
    }

    /// If the base_station inventory has the required materials for the recipe,
    /// Start processing the recipe by setting currently processing to the recipe,
    /// and starting a timer.
    fn smelt_materials(mut inventory: Mut<Inventory>, recipe: &RefineryRecipe, mut refinery: Mut<Refinery>, mut timer: &mut ResMut<RefineryTimer>) {
        if Self::have_materials_to_smelt(inventory.as_ref(), &recipe) {
            println!("We have the materials!");

            // Set currently processing to the recipe, finish processing after the timer.
            refinery.currently_processing = Some(recipe.clone());
            timer.0 = Some(Timer::new(Duration::from_secs(5), false));
        } else {
            println!("We do not have the materials!");
        }
    }

    /// Watch the refinery processing timer,
    /// perfom actions when timer elapses.
    fn update_refinery_processing(
        mut base_station_query: Query<(&BaseStation, &mut Inventory, &mut Refinery), With<BaseStation>>,
        mut timer: ResMut<RefineryTimer>,
        time: Res<Time>
    ) {
        if let Some(mut timer) = timer.0.as_mut() {
            timer.tick(time.delta());

            if timer.just_finished() {

                let (base_station, mut inventory, mut refinery) = base_station_query.single_mut();

                if let Some(currently_processing) = refinery.currently_processing.clone() {
                    for required_item in currently_processing.items_required.iter() {
                        inventory.remove_from_inventory(*required_item);
                    }
        
                    inventory.add_to_inventory(InventoryItem::Ingot(currently_processing.item_created, Amount::Quantity(1)));
                }

                refinery.currently_processing = None;

            }

        }
    }

    /// Perfom a smelt action with a recipe provided by the SmeltEvent.
    fn on_smelt_event(
        mut reader: EventReader<SmeltEvent>,
        mut base_station_query: Query<(&BaseStation, &mut Inventory, &mut Refinery), With<BaseStation>>,
        mut refinery_timer: ResMut<RefineryTimer>,
        mut time: Res<Time>
    ) {

        for event in reader.iter() {
            println!("Smelt Event Detected!");
            let (base_station, inventory, mut refinery) = base_station_query.single_mut();

            let recipe = event.0.clone();
            println!("{:?}", recipe);

            Self::smelt_materials(inventory, &recipe, refinery, &mut refinery_timer);
        }
    }

    pub fn attach_refinery_to_entity(mut commands: &mut Commands, refinery: Refinery, ent: Entity) {
        commands.entity(ent)
            .insert(Refinery::new());
    }

}