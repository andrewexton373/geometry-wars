use bevy::{
    prelude::*,
    render::{render_resource::WgpuFeatures, settings::WgpuSettings},
};
use bevy_hanabi::prelude::*;
pub struct ParticlePlugin;

#[derive(Component)]
pub struct PlayerShipTrailParticles;

#[derive(Component)]
pub struct ProjectileImpactParticles;

#[derive(Component)]
pub struct ShipAstroidImpactParticles;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let mut options = WgpuSettings::default();
        options
            .features
            .set(WgpuFeatures::VERTEX_WRITABLE_STORAGE, true);

        app.insert_resource(options)
            .add_plugin(HanabiPlugin)
            .add_startup_system(Self::setup_projectile_impact_particle_system)
            .add_startup_system(Self::setup_player_ship_trail_particle_system)
            .add_startup_system(Self::setup_ship_astroid_impact_particle_system);
    }
}

impl ParticlePlugin {
    fn setup_projectile_impact_particle_system(
        mut commands: Commands,
        mut effects: ResMut<Assets<EffectAsset>>,
    ) {
        let mut gradient = Gradient::new();
        gradient.add_key(0.0, Vec4::new(1.0, 0.0, 0.0, 1.0));
        gradient.add_key(1.0, Vec4::new(1.0, 0.5, 0.0, 0.0));

        let spawner = Spawner::once(100.0.into(), false);
        let effect = effects.add(
            EffectAsset {
                name: "Projectile Impact".into(),
                capacity: 32768,
                spawner,
                ..Default::default()
            }
            .init(PositionSphereModifier {
                radius: 0.05 * crate::PIXELS_PER_METER,
                speed: (5.0 * crate::PIXELS_PER_METER).into(),
                dimension: ShapeDimension::Surface,
                ..Default::default()
            })
            .init(ParticleLifetimeModifier { lifetime: 1.5 })
            .render(SizeOverLifetimeModifier {
                gradient: Gradient::constant(Vec2::splat(0.25 * crate::PIXELS_PER_METER)),
            })
            .render(ColorOverLifetimeModifier { gradient }),
        );

        commands
            .spawn_bundle(ParticleEffectBundle {
                // Assign the Z layer so it appears in the egui inspector and can be modified at runtime
                effect: ParticleEffect::new(effect).with_z_layer_2d(Some(50.0)),
                ..default()
            })
            .insert(ProjectileImpactParticles)
            .insert(Name::new("ProjectileImpactParticleEffect"));
    }

    fn setup_player_ship_trail_particle_system(
        mut commands: Commands,
        mut effects: ResMut<Assets<EffectAsset>>,
    ) {
        let mut gradient = Gradient::new();
        gradient.add_key(0.0, Vec4::new(1.0, 1.0, 1.0, 0.2));
        gradient.add_key(1.0, Vec4::new(1.0, 1.0, 1.0, 0.0));

        let mut size_gradient = Gradient::new();
        size_gradient.add_key(0.0, Vec2 { x: 0.0, y: 0.0 });
        size_gradient.add_key(
            1.0,
            Vec2 {
                x: 1.0 * crate::PIXELS_PER_METER,
                y: 1.0 * crate::PIXELS_PER_METER,
            },
        );

        let spawner = Spawner::once(50.0.into(), false);
        let effect = effects.add(
            EffectAsset {
                name: "Ship Trail".into(),
                capacity: 32768,
                spawner,
                ..Default::default()
            }
            .init(PositionSphereModifier {
                radius: 0.1 * crate::PIXELS_PER_METER,
                speed: (1.2 * crate::PIXELS_PER_METER).into(),
                dimension: ShapeDimension::Surface,
                ..Default::default()
            })
            .init(ParticleLifetimeModifier { lifetime: 2.0 })
            .render(SizeOverLifetimeModifier {
                // gradient: Gradient::constant(Vec2::splat(0.5 * crate::PIXELS_PER_METER)),
                gradient: size_gradient,
            })
            .render(ColorOverLifetimeModifier { gradient }),
        );

        commands
            .spawn_bundle(ParticleEffectBundle {
                // Assign the Z layer so it appears in the egui inspector and can be modified at runtime
                effect: ParticleEffect::new(effect).with_z_layer_2d(Some(50.0)),
                ..default()
            })
            .insert(PlayerShipTrailParticles)
            .insert(Name::new("PlayerShipTrailParticleEffect"));
    }

    fn setup_ship_astroid_impact_particle_system(
        mut commands: Commands,
        mut effects: ResMut<Assets<EffectAsset>>,
    ) {
        let mut gradient = Gradient::new();
        gradient.add_key(0.0, Vec4::new(1.0, 1.0, 0.0, 1.0));
        gradient.add_key(1.0, Vec4::new(1.0, 0.5, 0.0, 0.0));

        let spawner = Spawner::once(20.0.into(), false);
        let effect = effects.add(
            EffectAsset {
                name: "Ship-Astroid Impact".into(),
                capacity: 32768,
                spawner,
                ..Default::default()
            }
            .init(PositionSphereModifier {
                radius: 0.05 * crate::PIXELS_PER_METER,
                speed: (4.0 * crate::PIXELS_PER_METER).into(),
                dimension: ShapeDimension::Surface,
                ..Default::default()
            })
            .init(ParticleLifetimeModifier { lifetime: 1.0 })
            .render(SizeOverLifetimeModifier {
                gradient: Gradient::constant(Vec2::splat(0.25 * crate::PIXELS_PER_METER)),
            })
            .render(ColorOverLifetimeModifier { gradient }),
        );

        commands
            .spawn_bundle(ParticleEffectBundle {
                // Assign the Z layer so it appears in the egui inspector and can be modified at runtime
                effect: ParticleEffect::new(effect).with_z_layer_2d(Some(200.0)),
                ..default()
            })
            .insert(ShipAstroidImpactParticles)
            .insert(Name::new("ShipAstroidImpactParticleEffect"));
    }
}
