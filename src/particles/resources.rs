use bevy::{asset::Handle, ecs::resource::Resource};

#[derive(Resource)]
pub struct ProjectileImpactParticleEffect(pub Handle<bevy_hanabi::EffectAsset>);
