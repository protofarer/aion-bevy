use bevy::prelude::*;

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
