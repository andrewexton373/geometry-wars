use std::{default, time::Duration};

use bevy::{prelude::*, utils::{HashSet, HashMap}, time::Timer};
use bevy_prototype_lyon::prelude::{self as lyon};
use bevy_rapier2d::{prelude::{Velocity, Collider, Sleeping, Sensor, ActiveEvents, RapierContext}};

use crate::{astroid::{Astroid, AstroidMaterial}, PIXELS_PER_METER, player::Player, inventory::{Inventory, Capacity, InventoryPlugin, InventoryItem, Amount}, game_ui_widgets::SmeltEvent};

pub const BASE_STATION_SIZE: f32 = 20.0;

#[derive(Component)]
pub struct BaseStationDirectionIndicator;

pub struct BaseStationPlugin;

#[derive(Component)]
pub struct BaseStation;

pub struct CanDeposit(pub bool);

pub struct RefineryTimer(pub Option<Timer>);


// A component you can add to the base station in order to smelt ore.
#[derive(Component, Default, Debug, Clone, PartialEq)]
pub struct Refinery {
    pub recipes: Vec<RefineryRecipe>,
    pub currently_processing: Option<RefineryRecipe>,
    // refinery_timer: Option<Timer>
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

        recipes.push(iron_recipe);

        Self {
            recipes,
            currently_processing: None,
            // refinery_timer: None
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

impl Plugin for BaseStationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(Self::spawn_base_station)
            .add_startup_system(Self::spawn_player_base_guide_arrow)
            .add_system(Self::guide_player_to_base_station)
            .add_system(Self::repel_astroids_from_base_station)
            .add_system(Self::handle_base_station_sensor_collision_event)
            .add_event::<SmeltEvent>()
            .add_system(Self::on_smelt_event)
            .add_system(Self::update_refinery_processing)
            .insert_resource(CanDeposit(true))
            .insert_resource(RefineryTimer(None));
    }
}

impl BaseStationPlugin {
    fn spawn_base_station(
        mut commands: Commands
    ) {
        let base_shape = lyon::shapes::RegularPolygon {
            sides: 6,
            feature: lyon::shapes::RegularPolygonFeature::Radius(crate::PIXELS_PER_METER * BASE_STATION_SIZE),
            ..lyon::shapes::RegularPolygon::default()
        };
    
        let base_station = commands.spawn()
            .insert_bundle(lyon::GeometryBuilder::build_as(
                &base_shape,
                lyon::DrawMode::Outlined {
                    fill_mode: lyon::FillMode::color(Color::MIDNIGHT_BLUE),
                    outline_mode: lyon::StrokeMode::new(Color::WHITE, 5.0),
                },
                Transform { translation: Vec3::new(0.0, 0.0, -1.0), ..Default::default() }
            ))
            .insert(Sleeping::disabled())
            .insert(Collider::ball(crate::PIXELS_PER_METER * BASE_STATION_SIZE))
            .insert(Sensor)
            .insert(Transform::default())
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(BaseStation)
            .insert(Name::new("Base Station"))
            .id();

        commands.entity(base_station)
            .insert(Refinery::new());

        InventoryPlugin::attach_inventory_to_entity(&mut commands, Inventory {items: Vec::new(), capacity: Capacity {maximum: 1000.0}}, base_station);

    }


    fn spawn_player_base_guide_arrow(
        mut commands: Commands
    ) {
        let direction_indicator_shape = lyon::shapes::RegularPolygon {
            sides: 3,
            feature: lyon::shapes::RegularPolygonFeature::Radius(crate::PIXELS_PER_METER * 2.0),
            ..lyon::shapes::RegularPolygon::default()
        };
    
        let _direction_indicator = commands.spawn()
            .insert(BaseStationDirectionIndicator)
            .insert_bundle(lyon::GeometryBuilder::build_as(
                &direction_indicator_shape,
                lyon::DrawMode::Outlined {
                    fill_mode: lyon::FillMode::color(Color::RED),
                    outline_mode: lyon::StrokeMode::new(Color::WHITE, 1.0),
                },
                Default::default()
            ))
            // .insert(Collider::triangle(player_shape.feature, b, c)) // Need points of triangle
            .insert(Name::new("BaseStationDirectionIndicator"))
            .id();
    }
       

    fn guide_player_to_base_station(
        mut dir_indicator_query: Query<(&mut Transform, &mut GlobalTransform), (With<BaseStationDirectionIndicator>, Without<BaseStation>, Without<Player>)>,
        player_query: Query<(&Player, &GlobalTransform), (With<Player>, Without<BaseStation>)>,
        base_query: Query<(&BaseStation, &GlobalTransform), (With<BaseStation>, Without<Player>)>
    ) {
        let (mut dir_indicator_transform, dir_indicator_g_transform) = dir_indicator_query.single_mut();
        let (player, player_trans) = player_query.single();
        let (base_station, base_station_trans) = base_query.single();

        let player_pos = player_trans.translation().truncate();
        let base_station_pos = base_station_trans.translation().truncate();

        let direction_to_base = (base_station_pos - player_pos).normalize();
        let rotation = Vec2::Y.angle_between(direction_to_base);

        dir_indicator_transform.rotation = Quat::from_rotation_z(rotation);
        dir_indicator_transform.translation = (player_trans.translation().truncate() + direction_to_base * 100.0).extend(999.0);
        dir_indicator_transform.scale = Vec3::new(0.3, 1.0, 1.0);
    }
    

    fn repel_astroids_from_base_station(
        base_query: Query<(&BaseStation, &GlobalTransform), With<BaseStation>>,
        mut astroid_query: Query<(&Astroid, &GlobalTransform, &mut Velocity), With<Astroid>>
    ) {
        const REPEL_RADIUS: f32 = 120.0 * PIXELS_PER_METER;
        const REPEL_STRENGTH: f32 = 25.0;

        let (base_station, base_station_transform) = base_query.single();

        for (astroid, astroid_transform, mut astroid_velocity) in astroid_query.iter_mut() {
            let base_station_pos = base_station_transform.translation().truncate();
            let astroid_pos = astroid_transform.translation().truncate();

            let distance = base_station_pos.distance(astroid_pos);
            let distance_weight = 1.0 - (distance / REPEL_RADIUS);

            if distance < REPEL_RADIUS {
                let repel_vector = (astroid_pos - base_station_pos).normalize();
                astroid_velocity.linvel += repel_vector * distance_weight * REPEL_STRENGTH;
            }
        }
    }

    fn handle_base_station_sensor_collision_event(
        rapier_context: Res<RapierContext>,
        mut can_deposit_res: ResMut<CanDeposit>,
        player_query: Query<(Entity, &mut Player), With<Player>>,
        base_station_query: Query<(Entity, &BaseStation), With<BaseStation>>,
    ) {
        let (player_ent, player) = player_query.single();
        let (base_station_ent, base_station) = base_station_query.single();

        if rapier_context.intersection_pair(player_ent, base_station_ent) == Some(true) {
            *can_deposit_res = CanDeposit(true);
        } else {
            *can_deposit_res = CanDeposit(false);
        }

    }

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

    fn smelt_materials(mut inventory: Mut<Inventory>, recipe: &RefineryRecipe, mut refinery: Mut<Refinery>, mut timer: &mut ResMut<RefineryTimer>) {
        if Self::have_materials_to_smelt(inventory.as_ref(), &recipe) {
            println!("We have the materials!");
            refinery.currently_processing = Some(recipe.clone());
            timer.0 = Some(Timer::new(Duration::from_secs(5), false));

            // for required_item in recipe.items_required.iter() {
            //     inventory.remove_from_inventory(*required_item);
            // }

            // inventory.add_to_inventory(InventoryItem::Ingot(recipe.item_created, Amount::Quantity(1)));

        } else {
            println!("We do not have the materials!");
        }
    }

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
}