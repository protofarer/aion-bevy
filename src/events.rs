use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;
use bevy_particle_systems::*;
use bevy_rapier2d::{dynamics::Velocity, pipeline::CollisionEvent};

use crate::{
    audio::{ProjectileImpactSound, ShipDamagedSound},
    components::{AsteroidTag, CollisionRadius, DespawnDelay, PlayerShipTag, ProjectileTag},
    game::ParticlePixelTexture,
};

#[derive(Event)]
pub struct CollisionAsteroidAsteroidEvent(pub Entity, pub Entity);

#[derive(Event)]
pub struct CollisionProjectileEvent {
    pub projectile_ent: Entity,
    pub other_ent: Entity,
}

#[derive(Event)]
pub struct CollisionPlayerShipEvent;

#[derive(Event)]
pub struct CollisionProjectileAsteroidEvent;

#[derive(Default)]
pub enum Avatars {
    PlayerShip,
    Asteroid,
    Projectile,
    #[default]
    Other,
}