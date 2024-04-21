use std::time::Duration;

use bevy::{
    prelude::*,
    sprite::{Material2d, MaterialMesh2dBundle},
};
use bevy_rapier2d::{
    dynamics::{
        AdditionalMassProperties, Damping, ExternalForce, GravityScale, RigidBody, Velocity,
    },
    geometry::{ActiveEvents, Collider, Restitution},
};
use rand::Rng;

use crate::{
    avatars::{ProjectileEmitterBundle, Thruster},
    components::{
        AsteroidTag, Damage, FireType, FireTypes, Health, PlayerShipTag, PrimaryThrustMagnitude,
        ProjectileTag, TransientExistence, TurnRate,
    },
    utils::Heading,
    Speed, AMBIENT_ANGULAR_FRICTION_COEFFICIENT, AMBIENT_LINEAR_FRICTION_COEFFICIENT, BOTTOM_WALL,
    DEFAULT_HEALTH, DEFAULT_MOVESPEED, DEFAULT_RESTITUTION, DEFAULT_ROTATION, INIT_ASTEROID_DAMAGE,
    INIT_ASTEROID_MOVESPEED, INIT_SHIP_HEALTH, INIT_SHIP_PROJECTILE_SPEED, INIT_SHIP_TURN_RATE,
    LEFT_WALL, RIGHT_WALL, TOP_WALL,
};

#[derive(Bundle)]
pub struct Ship<M: Material2d> {
    mesh_bundle: MaterialMesh2dBundle<M>,
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
}

impl<M: Material2d> Ship<M> {
    pub fn new(
        x: f32,
        y: f32,
        heading: Option<Heading>,
        mesh: Handle<Mesh>,
        material: Handle<M>,
    ) -> (Self, (ProjectileEmitterBundle, Thruster)) {
        (
            Self {
                mesh_bundle: MaterialMesh2dBundle {
                    mesh: mesh.into(),
                    material,
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
                restitution: Restitution::coefficient(0.7),
                gravity: GravityScale(0.),
                damping: Damping {
                    linear_damping: AMBIENT_LINEAR_FRICTION_COEFFICIENT,
                    angular_damping: AMBIENT_ANGULAR_FRICTION_COEFFICIENT,
                },
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
}

#[derive(Bundle)]
pub struct ParticleBundle {
    sprite: SpriteBundle,
    velocity: Velocity,
    transient_existence: TransientExistence,
}

impl ParticleBundle {
    pub fn new(
        x: f32,
        y: f32,
        heading: Option<Heading>,
        move_speed: Option<Speed>,
        color: Option<Color>,
        duration: Option<Duration>,
    ) -> Self {
        let move_speed = match move_speed {
            Some(x) => x,
            None => DEFAULT_MOVESPEED,
        };
        let transient_existence = match duration {
            Some(x) => TransientExistence::new(x),
            None => TransientExistence::default(),
        };
        let heading = match heading {
            Some(x) => x,
            None => Heading::default(),
        };

        Self {
            sprite: SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(x, y, 0.),
                    rotation: heading.into(),
                    ..default()
                },
                sprite: Sprite {
                    color: color.unwrap_or_default(),
                    ..default()
                },
                ..default()
            },
            velocity: Velocity {
                linvel: heading.linvel(move_speed),
                ..default()
            },
            transient_existence,
        }
    }
}

impl Default for ParticleBundle {
    fn default() -> Self {
        Self {
            sprite: SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(
                        LEFT_WALL + (RIGHT_WALL - LEFT_WALL) / 2.,
                        BOTTOM_WALL + (TOP_WALL - BOTTOM_WALL) / 2.,
                        0.,
                    ),
                    rotation: *DEFAULT_ROTATION,
                    ..default()
                },
                ..default()
            },
            velocity: Velocity::default(),
            transient_existence: TransientExistence::default(),
        }
    }
}

#[derive(Bundle)]
pub struct ProjectileBundle {
    sprite: SpriteBundle,
    damage: Damage,
    transient_existence: TransientExistence,
    rigidbody: RigidBody,
    collider: Collider,
    collision_events: ActiveEvents,
    velocity: Velocity,
    restitution: Restitution,
    gravity: GravityScale,
    mass: AdditionalMassProperties,
    tag: ProjectileTag,
}

impl ProjectileBundle {
    pub fn new(
        x: f32,
        y: f32,
        heading: Option<Heading>,
        projectile_speed: Option<Speed>,
        color: Option<Color>,
        damage: Option<i32>,
        duration: Option<Duration>,
        restitution_coeff: Option<f32>,
        gravity_scale: Option<f32>,
        tag: ProjectileTag,
    ) -> Self {
        let projectile_speed = match projectile_speed {
            Some(x) => x,
            None => INIT_SHIP_PROJECTILE_SPEED,
        };

        let particle = ParticleBundle::new(x, y, heading, Some(projectile_speed), color, None);
        let sprite = particle.sprite;
        let velocity = particle.velocity;

        let damage = match damage {
            Some(x) => Damage(x),
            None => Damage::default(),
        };
        let transient_existence = match duration {
            Some(x) => TransientExistence::new(x),
            None => TransientExistence::default(),
        };
        let restitution = match restitution_coeff {
            Some(x) => Restitution::coefficient(x),
            None => Restitution::coefficient(DEFAULT_RESTITUTION),
        };
        let gravity = match gravity_scale {
            Some(x) => GravityScale(x),
            None => GravityScale(0.),
        };

        Self {
            sprite,
            rigidbody: RigidBody::Dynamic,
            collider: Collider::ball(0.5),
            collision_events: ActiveEvents::COLLISION_EVENTS,
            damage,
            transient_existence,
            velocity,
            restitution,
            gravity,
            mass: AdditionalMassProperties::Mass(100.),
            tag: ProjectileTag,
        }
    }
}

impl Default for ProjectileBundle {
    fn default() -> Self {
        let particle = ParticleBundle::new(0., 0., None, None, None, None);
        let sprite = particle.sprite;
        let velocity = particle.velocity;
        Self {
            sprite,
            damage: Damage::default(),
            transient_existence: TransientExistence::default(),
            rigidbody: RigidBody::Dynamic,
            collider: Collider::ball(1.),
            collision_events: ActiveEvents::COLLISION_EVENTS,
            velocity,
            restitution: Restitution::coefficient(DEFAULT_RESTITUTION),
            gravity: GravityScale(0.),
            mass: AdditionalMassProperties::Mass(100.),
            tag: ProjectileTag,
        }
    }
}

#[derive(Bundle)]
pub struct AsteroidBundle<M: Material2d> {
    mesh_bundle: MaterialMesh2dBundle<M>,
    rigidbody: RigidBody,
    collider: Collider,
    collision_events: ActiveEvents,
    velocity: Velocity,
    health: Health,
    damage: Damage,
    gravity: GravityScale,
    tag: AsteroidTag,
}

pub enum AsteroidSizes {
    Small,
    Medium,
    Large,
}

impl<M: Material2d> AsteroidBundle<M> {
    pub fn new(
        mesh: Handle<Mesh>,
        material: Handle<M>,
        r: f32,
        x: f32,
        y: f32,
        velocity: Option<Velocity>,
        health: Option<i32>,
        damage: Option<i32>,
    ) -> Self {
        let velocity = match velocity {
            Some(x) => x,
            None => Velocity {
                linvel: Heading::default().linvel(INIT_ASTEROID_MOVESPEED),
                ..default()
            },
        };
        let health = match health {
            Some(x) => Health(x),
            None => Health(DEFAULT_HEALTH),
        };

        let damage = match damage {
            Some(x) => Damage(x),
            None => Damage(INIT_ASTEROID_DAMAGE),
        };

        let mut rng = rand::thread_rng();
        let angvel = (rng.gen::<f32>() * 0.1) - 0.05;

        Self {
            mesh_bundle: MaterialMesh2dBundle {
                mesh: mesh.into(),
                material,
                transform: Transform {
                    translation: Vec3::new(x, y, 2.),
                    rotation: Heading::default().into(),
                    scale: Vec2::splat(1.).extend(1.),
                    ..default()
                },
                ..default()
            },
            rigidbody: RigidBody::Dynamic,
            velocity,
            damage,
            collider: Collider::ball(r),
            collision_events: ActiveEvents::COLLISION_EVENTS,
            health,
            gravity: GravityScale(0.),
            tag: AsteroidTag,
        }
    }
}

#[derive(Bundle)]
pub struct BoxoidBundle {
    sprite: SpriteBundle,
    collider: Collider,
    health: Health,
}

impl BoxoidBundle {
    pub fn new(x: f32, y: f32, half_x: f32, half_y: f32) -> Self {
        Self {
            sprite: SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(LEFT_WALL + x, BOTTOM_WALL + y, 0.),
                    scale: Vec3::new(half_x * 2., half_y * 2., 0.0),
                    rotation: *DEFAULT_ROTATION,
                },
                sprite: Sprite {
                    color: Color::ORANGE_RED,
                    ..default()
                },
                ..default()
            },
            collider: Collider::cuboid(half_x, half_y),
            health: Health(1),
        }
    }
}

impl Default for BoxoidBundle {
    fn default() -> Self {
        Self {
            sprite: SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(
                        LEFT_WALL + (RIGHT_WALL - LEFT_WALL) / 2.,
                        BOTTOM_WALL + (TOP_WALL - BOTTOM_WALL) / 2.,
                        0.,
                    ),
                    scale: Vec3::new(50., 50., 0.0),
                    rotation: *DEFAULT_ROTATION,
                },
                ..default()
            },
            collider: Collider::cuboid(25., 25.),
            health: Health(1),
        }
    }
}
