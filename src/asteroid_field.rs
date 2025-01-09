use avian2d::prelude::{LinearVelocity, Position};
use bevy::{prelude::*, utils::HashSet};
use rand::{thread_rng, Rng};
use rand_pcg::Pcg64;
use rand_seeder::Seeder;

use crate::{
    asteroid::{
        components::{Asteroid, AsteroidComposition},
        events::SpawnAsteroidEvent,
    },
    background::systems::SECTOR_SIZE,
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
        let mut rng: Pcg64 = Seeder::from(sector).make_rng();
        let asteroid_count = rng.gen_range(0..5);

        for _ in 0..asteroid_count {
            let radius = rng.gen_range(30.0..100.0);
            let offset_x = rng.gen_range(-0.5..0.5) * SECTOR_SIZE;
            let offset_y = rng.gen_range(-0.5..0.5) * SECTOR_SIZE;

            let asteroid_t = Transform::from_xyz(
                transform.translation.x + offset_x,
                transform.translation.y + offset_y,
                0.0,
            );

            spawn_events.send(SpawnAsteroidEvent(
                Asteroid::new_with(radius, AsteroidComposition::new_with_distance(0.0)),
                asteroid_t,
                LinearVelocity::ZERO,
            ));
        }

        visited_sectors.insert(*sector);
    }
}

#[derive(Component)]
pub struct AsteroidField;
