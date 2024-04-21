use std::f32::consts::PI;

use bevy::{
    prelude::*,
    sprite::{Material2d, MaterialMesh2dBundle},
    utils::Duration,
};
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::{
    archetypes::{AsteroidBundle, AsteroidSizes},
    components::{
        AsteroidTag, Damage, FireType, FireTypes, Health, MoveSpeed, Player, PlayerShipTag,
        PrimaryThrustMagnitude, ProjectileEmission, ProjectileTag, TransientExistence, TurnRate,
    },
    utils::Heading,
    Speed, AMBIENT_ANGULAR_FRICTION_COEFFICIENT, AMBIENT_LINEAR_FRICTION_COEFFICIENT, BOTTOM_WALL,
    DEFAULT_HEADING, DEFAULT_MOVESPEED, DEFAULT_RESTITUTION, DEFAULT_ROTATION,
    DEFAULT_THRUST_FORCE_MAGNITUDE, INIT_ASTEROID_DAMAGE, INIT_ASTEROID_MOVESPEED,
    INIT_SHIP_HEALTH, INIT_SHIP_MOVE_SPEED, INIT_SHIP_PROJECTILE_SPEED, INIT_SHIP_RESTITUTION,
    INIT_SHIP_TURN_RATE, LARGE_ASTEROID_HEALTH, LARGE_ASTEROID_R, LEFT_WALL,
    MEDIUM_ASTEROID_HEALTH, MEDIUM_ASTEROID_R, RIGHT_WALL, SMALL_ASTEROID_HEALTH, SMALL_ASTEROID_R,
    TOP_WALL,
};

#[derive(Bundle)]
pub struct PlayerShip {
    mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    turn_rate: TurnRate,
    collider: Collider,
    collision_events: ActiveEvents,
    health: Health,
    rigidbody: RigidBody,
    velocity: Velocity,
    primary_thrust_force: ExternalForce,
    primary_thrust_magnitude: PrimaryThrustMagnitude,
    restitution: Restitution,
    gravity: GravityScale,
    damping: Damping,
    tag: PlayerShipTag,
}

pub fn gen_playership(
    mesh_handle: Handle<Mesh>,
    material_handle: Handle<ColorMaterial>,
    x: f32,
    y: f32,
    heading: Option<Heading>,
) -> (PlayerShip, (ProjectileEmitterBundle, Thruster)) {
    (
        PlayerShip {
            mesh_bundle: MaterialMesh2dBundle {
                mesh: mesh_handle.into(),
                material: material_handle.into(),
                transform: Transform {
                    translation: Vec3::new(x, y, 1.),
                    rotation: heading.unwrap_or_default().into(),
                    ..default()
                },
                ..default()
            },
            collider: Collider::triangle(
                Vec2::new(-15., -15.),
                Vec2::X * 22.,
                Vec2::new(-15., 15.),
            ),
            collision_events: ActiveEvents::COLLISION_EVENTS,
            health: Health(INIT_SHIP_HEALTH),
            turn_rate: TurnRate(INIT_SHIP_TURN_RATE),
            rigidbody: RigidBody::Dynamic,
            velocity: Velocity {
                linvel: Vec2::ZERO,
                angvel: 0.,
            },
            primary_thrust_force: ExternalForce {
                force: Vec2::ZERO,
                torque: 0.,
            },
            primary_thrust_magnitude: PrimaryThrustMagnitude::default(),
            restitution: Restitution::coefficient(INIT_SHIP_RESTITUTION),
            gravity: GravityScale(0.),
            damping: Damping {
                linear_damping: AMBIENT_LINEAR_FRICTION_COEFFICIENT,
                angular_damping: AMBIENT_ANGULAR_FRICTION_COEFFICIENT,
            },
            tag: PlayerShipTag,
        },
        (
            ProjectileEmitterBundle::new(
                22.,
                heading.unwrap_or_default(),
                Some(FireType {
                    fire_type: FireTypes::Primary,
                }),
            ),
            Thruster::default(),
        ),
    )
}

#[derive(Bundle)]
pub struct ProjectileEmitterBundle {
    emitter: ProjectileEmission,
    transform: TransformBundle,
    fire_type: FireType, // transform hierarchy via both Transform and GlobalTransform, so it can "attached" to a ship or other avatar
                         // sprite: SpriteBundle,
}

impl ProjectileEmitterBundle {
    pub fn new(r: f32, heading: Heading, fire_type: Option<FireType>) -> Self {
        let mut vec2 = Vec2::new(heading.0.cos(), heading.0.sin());
        let fire_type = match fire_type {
            Some(x) => x,
            None => FireType {
                fire_type: FireTypes::Primary,
            },
        };

        Self {
            emitter: ProjectileEmission::default(),
            transform: TransformBundle {
                local: Transform {
                    translation: Vec3::new(vec2.x, vec2.y, 0.) * 1.05 * r,
                    ..default()
                },
                ..default()
            },
            fire_type,
        }
    }
}

impl Default for ProjectileEmitterBundle {
    fn default() -> Self {
        Self {
            emitter: ProjectileEmission::default(),
            transform: TransformBundle::default(),
            fire_type: FireType {
                fire_type: FireTypes::Primary,
            }, // sprite: SpriteBundle {
               //     transform: Transform {
               //         translation: Vec3::new(0., 0., 2.),
               //         scale: Vec3::new(10., 10., 1.),
               //         rotation: Heading::default().into(),
               //         ..default()
               //     },
               //     sprite: Sprite {
               //         color: Color::RED,
               //         ..default()
               //     },
               //     ..default()
               // },
        }
    }
}

// thrusters apply only linear force onto rigidbody (conventionally onto center of mass)
// used as a child, so that there can be many thrusters for 1 parent entity
#[derive(Component)]
pub struct Thruster(pub f32);

impl Default for Thruster {
    fn default() -> Self {
        Self(DEFAULT_THRUST_FORCE_MAGNITUDE)
    }
}

pub fn gen_asteroid(
    size: AsteroidSizes,
    n_sides: usize,
    mesh_handles: Vec<Handle<Mesh>>,
    material_handles: Vec<Handle<ColorMaterial>>,
    x: f32,
    y: f32,
    velocity: Velocity,
) -> AsteroidBundle<ColorMaterial> {
    let r = match size {
        AsteroidSizes::Small => SMALL_ASTEROID_R,
        AsteroidSizes::Medium => MEDIUM_ASTEROID_R,
        AsteroidSizes::Large => LARGE_ASTEROID_R,
    };
    let (handle_mesh, health) = match r {
        SMALL_ASTEROID_R => (
            match n_sides {
                5 => mesh_handles[0].clone(),
                6 => mesh_handles[3].clone(),
                8 => mesh_handles[6].clone(),
                _ => mesh_handles[0].clone(),
            },
            SMALL_ASTEROID_HEALTH,
        ),
        MEDIUM_ASTEROID_R => (
            match n_sides {
                5 => mesh_handles[1].clone(),
                6 => mesh_handles[4].clone(),
                8 => mesh_handles[7].clone(),
                _ => mesh_handles[0].clone(),
            },
            MEDIUM_ASTEROID_HEALTH,
        ),
        LARGE_ASTEROID_R => (
            match n_sides {
                5 => mesh_handles[2].clone(),
                6 => mesh_handles[5].clone(),
                8 => mesh_handles[8].clone(),
                _ => mesh_handles[0].clone(),
            },
            LARGE_ASTEROID_HEALTH,
        ),
        _ => (mesh_handles[0].clone(), SMALL_ASTEROID_HEALTH),
    };
    AsteroidBundle::new(
        handle_mesh,
        material_handles[0].clone(),
        r,
        x,
        y,
        Some(velocity),
        Some(health),
        None,
    )
}
