use avian2d::prelude::LinearVelocity;
use bevy::{prelude::*, utils::HashSet};

use crate::{
    asteroid::{
        components::{Asteroid, AsteroidComposition},
        events::SpawnAsteroidEvent,
    },
    sector::Sector,
};

pub struct AsteroidFieldPlugin;

impl Plugin for AsteroidFieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (init_asteroid_field))
            .add_observer(generate_asteroids_in_sector);
    }
}

pub fn init_asteroid_field(mut commands: Commands) {
    commands.spawn(AsteroidField);
}

pub fn generate_asteroids_in_sector(
    trigger: Trigger<OnAdd, Sector>,
    mut spawn_events: EventWriter<SpawnAsteroidEvent>,
    sectors: Query<(&Sector, &Transform)>,
    mut visited_sectors: Local<HashSet<Sector>>,
) {
    let (sector, transform) = sectors.get(trigger.entity()).unwrap();

    if !visited_sectors.contains(sector) {
        spawn_events.send(SpawnAsteroidEvent(
            Asteroid::new_with(100.0, AsteroidComposition::new_with_distance(0.0)),
            *transform,
            LinearVelocity::ZERO,
        ));
        visited_sectors.insert(*sector);
    }
}

#[derive(Component)]
pub struct AsteroidField;
