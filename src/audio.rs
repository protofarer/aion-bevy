use bevy::{prelude::*, time::Stopwatch};

#[derive(Resource)]
pub struct BackgroundMusic(pub Handle<AudioSource>);

#[derive(Resource)]
pub struct ProjectileEmitSound(pub Handle<AudioSource>);

#[derive(Resource)]
pub struct ShipThrustSound(pub Handle<AudioSource>);

#[derive(Resource)]
pub struct ProjectileImpactSound(pub Handle<AudioSource>);

#[derive(Resource)]
pub struct AsteroidDestroyedSound(pub Handle<AudioSource>);

// #[derive(Resource)]
// pub struct AsteroidImpactSound(Handle<AudioSource>);

#[derive(Resource)]
pub struct ShipDamagedSound(pub Handle<AudioSource>);

// #[derive(Resource)]
// pub struct ShipImpactSound(Handle<AudioSource>);

#[derive(Resource, Deref, DerefMut)]
pub struct ShipThrustSoundStopwatch(pub Stopwatch);
