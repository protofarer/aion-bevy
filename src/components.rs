use bevy::prelude::*;
use bevy::utils::{Duration, Instant};

use crate::game::{
    Speed, TurnSpeed, DEFAULT_DAMAGE, DEFAULT_DURATION_SECS, DEFAULT_HEALTH,
    DEFAULT_PROJECTILE_EMISSION_COOLDOWN, DEFAULT_THRUST_FORCE_MAGNITUDE, DEFAULT_TURNRATE,
    INIT_SHIP_MOVE_SPEED,
};

// MARKERS

#[derive(Component)]
pub struct ProjectileTag;

#[derive(Component)]
pub struct AsteroidTag;

#[derive(Component)]
pub struct PlayerShipTag;

// DATA

#[derive(Component)]
pub enum Player {
    A,
}

#[derive(Component, Deref, DerefMut)]
pub struct Health(pub i32);

impl Default for Health {
    fn default() -> Self {
        Self(DEFAULT_HEALTH)
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct TurnRate(pub TurnSpeed);

impl Default for TurnRate {
    fn default() -> Self {
        Self(DEFAULT_TURNRATE)
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct Damage(pub i32);

impl Default for Damage {
    fn default() -> Self {
        Self(DEFAULT_DAMAGE)
    }
}

#[derive(Component)]
pub struct ScoreboardUi;

#[derive(Component)]
pub enum FireType {
    // #[default]
    Primary,
    Secondary,
}

// #[derive(Component)]
// pub struct FireType {
//     pub fire_type: FireTypes,
// }

#[derive(Component, Deref, DerefMut)]
pub struct PrimaryThrustMagnitude(pub f32);

impl Default for PrimaryThrustMagnitude {
    fn default() -> Self {
        Self(DEFAULT_THRUST_FORCE_MAGNITUDE)
    }
}

#[derive(Component)]
pub struct ProjectileEmission {
    pub projectile_speed: Speed,
    pub cooldown_ms: i32,
    pub projectile_duration: Duration,
    pub damage: i32,
    pub is_friendly: bool,
    pub last_emission_time: Instant,
}

impl ProjectileEmission {
    pub fn new(
        projectile_speed: Speed,
        cooldown: i32,
        projectile_duration: Duration,
        damage: i32,
    ) -> Self {
        Self {
            projectile_speed,
            cooldown_ms: cooldown,
            projectile_duration,
            damage,
            is_friendly: false,
            last_emission_time: Instant::now(),
        }
    }
}

impl Default for ProjectileEmission {
    fn default() -> Self {
        Self {
            projectile_speed: INIT_SHIP_MOVE_SPEED + 200.,
            cooldown_ms: DEFAULT_PROJECTILE_EMISSION_COOLDOWN,
            projectile_duration: Duration::from_secs(DEFAULT_DURATION_SECS),
            damage: DEFAULT_DAMAGE,
            is_friendly: false,
            last_emission_time: Instant::now(),
        }
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct Score(pub usize);

#[derive(Component, Copy, Clone, Deref, DerefMut)]
pub struct CollisionRadius(pub f32);

#[derive(Component, Deref, DerefMut)]
pub struct DespawnDelay(pub Timer);
