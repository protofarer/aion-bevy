use std::f32::consts::PI;

use bevy::{
    prelude::*,
    sprite::{Material2d, MaterialMesh2dBundle},
    utils::Duration,
};
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::{
    components::{
        Damage, FireType, FireTypes, Health, MoveSpeed, Player, PrimaryThrustMagnitude,
        ProjectileEmission, ProjectileTag, TransientExistence, TurnRate,
    },
    Speed, AMBIENT_ANGULAR_FRICTION_COEFFICIENT, AMBIENT_LINEAR_FRICTION_COEFFICIENT, BOTTOM_WALL,
    DEFAULT_HEADING, DEFAULT_MOVESPEED, DEFAULT_RESTITUTION, DEFAULT_ROTATION,
    DEFAULT_THRUST_FORCE_MAGNITUDE, INIT_ASTEROID_DAMAGE, INIT_ASTEROID_HEALTH,
    INIT_ASTEROID_MOVE_SPEED, INIT_SHIP_HEALTH, INIT_SHIP_MOVE_SPEED, INIT_SHIP_PROJECTILE_SPEED,
    INIT_SHIP_TURN_RATE, LEFT_WALL, RIGHT_WALL, TOP_WALL,
};

#[derive(Bundle)]
pub struct PlayerShip<M: Material2d> {
    mesh_bundle: MaterialMesh2dBundle<M>,
    move_speed: MoveSpeed,
    turn_rate: TurnRate,
    collider: Collider,
    collision_events: ActiveEvents,
    health: Health,
    player: Player,
    rigidbody: RigidBody,
    velocity: Velocity,
    primary_thrust_force: ExternalForce,
    primary_thrust_magnitude: PrimaryThrustMagnitude,
    restitution: Restitution,
    gravity: GravityScale,
    damping: Damping,
}

impl<M: Material2d> PlayerShip<M> {
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
                player: Player::A,
                move_speed: MoveSpeed(INIT_SHIP_MOVE_SPEED),
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
pub struct Asteroid<M: Material2d> {
    mesh_bundle: MaterialMesh2dBundle<M>,
    rigidbody: RigidBody,
    collider: Collider,
    collision_events: ActiveEvents,
    velocity: Velocity,
    health: Health,
    damage: Damage,
    gravity: GravityScale,
}

impl<M: Material2d> Asteroid<M> {
    pub fn new(
        x: f32,
        y: f32,
        r: f32,
        mesh: Handle<Mesh>,
        material: Handle<M>,
        heading: Option<Heading>,
        speed: Option<Speed>,
        damage: Option<i32>,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let angvel = (rng.gen::<f32>() * 0.1) - 0.05;
        let speed = match speed {
            Some(x) => x,
            None => INIT_ASTEROID_MOVE_SPEED,
        };
        let damage = match damage {
            Some(x) => Damage(x),
            None => Damage(INIT_ASTEROID_DAMAGE),
        };
        let heading = match heading {
            Some(x) => x,
            None => Heading::default(),
        };
        Self {
            mesh_bundle: MaterialMesh2dBundle {
                mesh: mesh.into(),
                material,
                transform: Transform {
                    translation: Vec3::new(x, y, 2.),
                    rotation: heading.into(),
                    scale: Vec2::splat(1.).extend(1.),
                    ..default()
                },
                ..default()
            },
            rigidbody: RigidBody::Dynamic,
            velocity: Velocity {
                linvel: heading.linvel(speed),
                angvel,
            },
            damage,

            collider: Collider::ball(r),
            collision_events: ActiveEvents::COLLISION_EVENTS,
            health: Health(INIT_ASTEROID_HEALTH),
            gravity: GravityScale(0.),
        }
    }
}

#[derive(Bundle)]
pub struct Boxoid {
    sprite: SpriteBundle,
    collider: Collider,
    health: Health,
}

impl Boxoid {
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

impl Default for Boxoid {
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Heading(pub f32); // degrees

impl Heading {
    pub fn to_vec3(&self) -> Vec3 {
        let angle_radians = self.0.to_radians();
        let x = angle_radians.cos();
        let y = angle_radians.sin();
        Vec3::new(x, y, 0.)
    }
    pub fn linvel(&self, speed: Speed) -> Vec2 {
        Vec2::new(self.0.cos(), self.0.sin()) * speed
    }
}

impl Default for Heading {
    fn default() -> Self {
        Heading(DEFAULT_HEADING.0)
    }
}

impl Into<Quat> for Heading {
    fn into(self) -> Quat {
        let angle_radians = self.0 * PI / 360.;
        Quat::from_rotation_z(angle_radians)
    }
}

impl From<Quat> for Heading {
    fn from(quat: Quat) -> Self {
        Heading(quat.to_axis_angle().1)
    }
}

#[derive(Bundle)]
pub struct Particle {
    sprite: SpriteBundle,
    velocity: Velocity,
    transient_existence: TransientExistence,
}

impl Particle {
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

impl Default for Particle {
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
pub struct Projectile {
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

impl Projectile {
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

        let particle = Particle::new(x, y, heading, Some(projectile_speed), color, None);
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

impl Default for Projectile {
    fn default() -> Self {
        let particle = Particle::new(0., 0., None, None, None, None);
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
pub struct ProjectileEmitterBundle {
    emitter: ProjectileEmission,
    transform: TransformBundle,
    fire_type: FireType, // transform hierarchy via both Transform and GlobalTransform, so it can "attached" to a ship or other avatar
                         // sprite: SpriteBundle,
}

impl ProjectileEmitterBundle {
    pub fn new(r: f32, heading: Heading, fire_type: Option<FireType>) -> Self {
        let mut vec2 = Vec2::new(heading.0.cos(), heading.0.sin());
        info!("emitter rel to ship, x:{:?} y:{:?}", vec2.x, vec2.y);
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
