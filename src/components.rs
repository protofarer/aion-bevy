use bevy::prelude::*;
use bevy::utils::{Duration, Instant};

use crate::{
    Speed, TurnSpeed, DEFAULT_DAMAGE, DEFAULT_DURATION_SECS, DEFAULT_HEALTH, DEFAULT_MOVESPEED,
    DEFAULT_PROJECTILE_EMISSION_COOLDOWN, DEFAULT_THRUST_FORCE_MAGNITUDE, DEFAULT_TURNRATE,
    INIT_SHIP_MOVE_SPEED,
};

#[derive(Component)]
pub enum Player {
    A,
}

#[derive(Component)]
pub struct Health(pub i32);

impl Default for Health {
    fn default() -> Self {
        Self(DEFAULT_HEALTH)
    }
}

#[derive(Component)]
pub struct TurnRate(pub TurnSpeed);

impl Default for TurnRate {
    fn default() -> Self {
        Self(DEFAULT_TURNRATE)
    }
}

#[derive(Component)]
pub struct Damage(pub i32);

impl Default for Damage {
    fn default() -> Self {
        Self(DEFAULT_DAMAGE)
    }
}

#[derive(Component)]
pub struct TransientExistence {
    duration: Duration,
    start_time: Instant,
}

impl TransientExistence {
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            start_time: Instant::now(),
        }
    }
}

impl Default for TransientExistence {
    fn default() -> Self {
        Self {
            duration: Duration::from_secs(DEFAULT_DURATION_SECS),
            start_time: Instant::now(),
        }
    }
}

#[derive(Component)]
pub struct MoveSpeed(pub Speed);

impl Default for MoveSpeed {
    fn default() -> Self {
        Self(DEFAULT_MOVESPEED)
    }
}

#[derive(Component)]
pub struct ScoreboardUi;

#[derive(Default)]
pub enum FireTypes {
    #[default]
    Primary,
    Secondary,
}

#[derive(Component)]
pub struct FireType {
    pub fire_type: FireTypes,
}

#[derive(Component)]
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

#[derive(Resource, Default)]
pub struct Score(pub usize);

// MARKERS
#[derive(Component)]
pub struct ProjectileTag;

#[derive(Component)]
pub struct AsteroidTag;

#[derive(Component)]
pub struct PlayerShipTag;

#[derive(Component)]
pub struct CollisionRadius(pub f32);
