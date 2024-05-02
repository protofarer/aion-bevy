use bevy::{
    prelude::*,
    sprite::{Material2d, MaterialMesh2dBundle},
};
use bevy_rapier2d::{
    dynamics::{
        AdditionalMassProperties, Damping, ExternalForce, GravityScale, RigidBody, Velocity,
    },
    geometry::{ActiveEvents, Collider, ColliderMassProperties, Restitution},
};

use crate::{
    avatars::{ProjectileEmitterBundle, Thrust, ThrusterBundle},
    components::{AsteroidTag, CollisionRadius, Damage, FireType, Health, ProjectileTag, TurnRate},
    game::{
        ParticlePixelTexture, PlayerShipTexture, Speed, AMBIENT_ANGULAR_FRICTION_COEFFICIENT,
        AMBIENT_LINEAR_FRICTION_COEFFICIENT, BOTTOM_WALL, DEFAULT_HEALTH, DEFAULT_MOVESPEED,
        DEFAULT_RESTITUTION, DEFAULT_ROTATION, DEFAULT_THRUST_FORCE_MAGNITUDE,
        INIT_ASTEROID_DAMAGE, INIT_ASTEROID_MOVESPEED, INIT_ASTEROID_RESTITUTION, INIT_SHIP_HEALTH,
        INIT_SHIP_PROJECTILE_SPEED, INIT_SHIP_TURN_RATE, LEFT_WALL, RIGHT_WALL, TOP_WALL,
    },
    utils::Heading,
};

#[derive(Bundle)]
pub struct Ship {
    sprite_bundle: SpriteBundle,
    // mesh_bundle: MaterialMesh2dBundle<M>,
    turn_rate: TurnRate,
    collider: Collider,
    collision_events: ActiveEvents,
    health: Health,
    rigidbody: RigidBody,
    velocity: Velocity,
    primary_thrust_force: ExternalForce,
    restitution: Restitution,
    gravity: GravityScale,
    damping: Damping,
}

impl Ship {
    pub fn new(
        x: f32,
        y: f32,
        heading: Option<Heading>,
        texture: &PlayerShipTexture,
        // mesh: Handle<Mesh>,
        // material: Handle<M>,
        particle_pixel_texture: &ParticlePixelTexture,
    ) -> (Self, (ProjectileEmitterBundle, ThrusterBundle)) {
        (
            Self {
                sprite_bundle: SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(1., 0., 0., 1.),
                        ..default()
                    },
                    texture: texture.0.clone().into(),
                    transform: Transform {
                        translation: Vec3::new(x, y, 1.),
                        scale: Vec2::splat(0.8).extend(1.),
                        // rotation: heading.unwrap_or_default().into(),
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
                restitution: Restitution::coefficient(0.7),
                gravity: GravityScale(0.),
                damping: Damping {
                    linear_damping: AMBIENT_LINEAR_FRICTION_COEFFICIENT,
                    angular_damping: AMBIENT_ANGULAR_FRICTION_COEFFICIENT,
                },
            },
            (
                ProjectileEmitterBundle::new(22., heading, Some(FireType::Primary)),
                ThrusterBundle::new(
                    0.,
                    0.,
                    DEFAULT_THRUST_FORCE_MAGNITUDE,
                    particle_pixel_texture.0.clone().into(),
                ),
            ),
        )
    }
}

#[derive(Bundle)]
pub struct ParticleBundle {
    sprite: SpriteBundle,
    velocity: Velocity,
}

impl ParticleBundle {
    pub fn new(
        x: f32,
        y: f32,
        heading: Option<Heading>,
        move_speed: Option<Speed>,
        color: Option<Color>,
        scale: Option<f32>,
    ) -> Self {
        let move_speed = match move_speed {
            Some(x) => x,
            None => DEFAULT_MOVESPEED,
        };
        let heading = match heading {
            Some(x) => x,
            None => Heading::default(),
        };

        let scale = match scale {
            Some(x) => x,
            None => 1.0,
        };

        Self {
            sprite: SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(x, y, 0.),
                    rotation: heading.into(),
                    scale: Vec3::splat(scale),
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
        }
    }
}

#[derive(Bundle)]
pub struct ProjectileBundle {
    sprite: SpriteBundle,
    damage: Damage,
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
        restitution_coeff: Option<f32>,
        gravity_scale: Option<f32>,
        scale: Option<f32>,
    ) -> Self {
        let projectile_speed = match projectile_speed {
            Some(x) => x,
            None => INIT_SHIP_PROJECTILE_SPEED,
        };

        let particle = ParticleBundle::new(x, y, heading, Some(projectile_speed), color, scale);
        let sprite = particle.sprite;
        let velocity = particle.velocity;

        let damage = match damage {
            Some(x) => Damage(x),
            None => Damage::default(),
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
    collider_mass_properties: ColliderMassProperties,
    collision_radius: CollisionRadius,
    collision_events: ActiveEvents,
    // TODO scale animations with force (as opposed to bruting size) https://rapier.rs/docs/user_guides/bevy_plugin/advanced_collision_detection#the-contact-graph
    // collision_force_events: ActiveEvent::CONTACT_FORCE_EVENTS,
    velocity: Velocity,
    health: Health,
    damage: Damage,
    restitution: Restitution,
    gravity: GravityScale,
    tag: AsteroidTag,
}

#[derive(Copy, Clone, Default)]
pub enum AsteroidSizes {
    Small,
    #[default]
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

        // let mut rng = rand::thread_rng();
        // let angvel = (rng.gen::<f32>() * 0.1) - 0.05;

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
            collider_mass_properties: ColliderMassProperties::Density(5.0),
            collision_radius: CollisionRadius(r),
            collision_events: ActiveEvents::COLLISION_EVENTS,
            // collision_force_events: ActiveEvents::COLLISION_FORCE_EVENTS,
            health,
            restitution: Restitution::coefficient(INIT_ASTEROID_RESTITUTION),
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
