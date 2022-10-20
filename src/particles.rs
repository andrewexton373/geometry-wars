use bevy::{prelude::*, render::{
    render_resource::WgpuFeatures,
    settings::WgpuSettings,
}};
use bevy_hanabi::prelude::*;
pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {

        let mut options = WgpuSettings::default();
        options
            .features
            .set(WgpuFeatures::VERTEX_WRITABLE_STORAGE, true);
    

        app
            .insert_resource(options)
            .add_plugin(HanabiPlugin)
            .add_startup_system(Self::setup_projectile_impact_particle_system)
            .add_startup_system(Self::setup_player_ship_trail_particle_system);
    }
}

impl ParticlePlugin {
    fn setup_projectile_impact_particle_system(
        mut commands: Commands,
        mut effects: ResMut<Assets<EffectAsset>>,
    ) {

        let mut gradient = Gradient::new();
        gradient.add_key(0.0, Vec4::new(1.0, 1.0, 1.0, 1.0));
        gradient.add_key(1.0, Vec4::new(1.0, 1.0, 1.0, 0.0));
    
        let spawner = Spawner::once(30.0.into(), false);
        let effect = effects.add(
            EffectAsset {
                name: "Impact".into(),
                capacity: 32768,
                spawner,
                ..Default::default()
            }
            .init(PositionSphereModifier {
                radius: 0.05 * crate::PIXELS_PER_METER,
                speed: (0.2 * crate::PIXELS_PER_METER).into(),
                dimension: ShapeDimension::Surface,
                ..Default::default()
            })
            .render(SizeOverLifetimeModifier {
                gradient: Gradient::constant(Vec2::splat(0.25 * crate::PIXELS_PER_METER)),
            })
            .render(ColorOverLifetimeModifier { gradient }),
        );
    
        // Spawn an instance of the particle effect, and override its Z layer to
        // be above the reference white square previously spawned.
        // commands
        //     .spawn_bundle(ParticleEffectBundle {
        //         // Assign the Z layer so it appears in the egui inspector and can be modified at runtime
        //         effect: ParticleEffect::new(effect).with_z_layer_2d(Some(0.1)),
        //         ..default()
        //     })
        //     .insert(Name::new("effect:2d"));
    
        commands
            .spawn_bundle(ParticleEffectBundle::new(effect).with_spawner(spawner))
            .insert(Name::new("particle-effect"));

    }

    fn setup_player_ship_trail_particle_system(
        commands: Commands
    ) {

    }
}