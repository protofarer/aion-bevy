use std::f32::consts::PI;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_particle_systems::{
    CircleSegment, ColorOverTime, Curve, CurvePoint, JitteredValue, ParticleSystem,
    ParticleSystemBundle, ParticleTexture,
};
use bevy_rapier2d::prelude::*;

use crate::{
    archetypes::{AsteroidBundle, AsteroidSizes},
    components::{
        FireType, Health, PlayerShipTag, PrimaryThrustMagnitude, ProjectileEmission, TurnRate,
    },
    game::{
        ParticlePixelTexture, PlayerShipTexture, AMBIENT_ANGULAR_FRICTION_COEFFICIENT,
        AMBIENT_LINEAR_FRICTION_COEFFICIENT, DEFAULT_THRUST_FORCE_MAGNITUDE, INIT_SHIP_HEALTH,
        INIT_SHIP_RESTITUTION, INIT_SHIP_TURN_RATE, LARGE_ASTEROID_HEALTH, LARGE_ASTEROID_R,
        MEDIUM_ASTEROID_HEALTH, MEDIUM_ASTEROID_R, SMALL_ASTEROID_HEALTH, SMALL_ASTEROID_R,
    },
    utils::Heading,
};
#[derive(Bundle)]
pub struct PlayerShip {
    sprite_bundle: SpriteBundle,
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
    texture: &PlayerShipTexture,
    x: f32,
    y: f32,
    heading: Option<Heading>,
    particle_pixel_texture: &ParticlePixelTexture,
) -> (PlayerShip, (ProjectileEmitterBundle, ThrusterBundle)) {
    (
        PlayerShip {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(0., 1., 0., 1.),
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
            primary_thrust_magnitude: PrimaryThrustMagnitude::default(),
            restitution: Restitution {
                coefficient: INIT_SHIP_RESTITUTION,
                combine_rule: CoefficientCombineRule::Multiply, // extra bouncy for player's sake to not get quickly dribbled to death
            },
            gravity: GravityScale(0.),
            damping: Damping {
                linear_damping: AMBIENT_LINEAR_FRICTION_COEFFICIENT,
                angular_damping: AMBIENT_ANGULAR_FRICTION_COEFFICIENT,
            },
            tag: PlayerShipTag,
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

#[derive(Bundle)]
pub struct ProjectileEmitterBundle {
    emitter: ProjectileEmission,
    transform: TransformBundle,
    fire_type: FireType, // transform hierarchy via both Transform and GlobalTransform, so it can "attached" to a ship or other avatar
                         // sprite: SpriteBundle,
}

impl ProjectileEmitterBundle {
    pub fn new(r: f32, heading: Option<Heading>, fire_type: Option<FireType>) -> Self {
        let heading = heading.unwrap_or_default();
        let fire_type = match fire_type {
            Some(x) => x,
            None => FireType::Primary,
        };

        Self {
            emitter: ProjectileEmission::default(),
            transform: TransformBundle {
                local: Transform {
                    translation: Vec3::new(heading.x(), heading.y(), 0.) * 1.05 * r,
                    rotation: heading.into(),
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
            fire_type: FireType::Primary,
            // sprite: SpriteBundle {
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
pub struct Thrust(pub f32);

impl Default for Thrust {
    fn default() -> Self {
        Self(DEFAULT_THRUST_FORCE_MAGNITUDE)
    }
}

#[derive(Bundle)]
pub struct ThrusterBundle {
    thrust: Thrust,
    particles: ParticleSystemBundle,
}

impl ThrusterBundle {
    pub fn new(_x: f32, _y: f32, thrust: f32, particle_texture: ParticleTexture) -> ThrusterBundle {
        ThrusterBundle {
            thrust: Thrust(thrust),
            particles: ParticleSystemBundle {
                particle_system: ParticleSystem {
                    max_particles: 1000,
                    texture: particle_texture,
                    spawn_rate_per_second: 50.0.into(),
                    initial_speed: JitteredValue::jittered(200.0, -25.0..25.0),
                    lifetime: JitteredValue::jittered(2.0, -1.0..1.0),
                    color: ColorOverTime::Gradient(Curve::new(vec![
                        CurvePoint::new(Color::BLUE, 0.0),
                        CurvePoint::new(Color::ORANGE, 0.025),
                        CurvePoint::new(Color::RED, 0.1),
                        CurvePoint::new(Color::RED, 1.),
                    ])),
                    emitter_shape: CircleSegment {
                        radius: 30.0.into(),
                        opening_angle: std::f32::consts::PI / 12.,
                        // direction_angle: Heading::default().to_radians() + PI,
                        direction_angle: -PI / 2.0,
                    }
                    .into(),
                    looping: true,
                    rotate_to_movement_direction: true,
                    initial_rotation: (0.0_f32).to_radians().into(),
                    system_duration_seconds: 10.0,
                    max_distance: Some(100.0),
                    scale: 1.0.into(),
                    ..ParticleSystem::default()
                },
                transform: Transform::from_xyz(0., 20., 0.0),
                ..ParticleSystemBundle::default()
            },
        }
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

// can be used for any avatar that has a mesh and material
#[derive(Bundle)]
pub struct PlayerShipMaterialMesh {
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

pub fn gen_playership_from_materialmesh(
    mesh_handle: Handle<Mesh>,
    material_handle: Handle<ColorMaterial>,
    x: f32,
    y: f32,
    heading: Option<Heading>,
    thruster_particle_texture: Handle<Image>,
) -> (
    PlayerShipMaterialMesh,
    (ProjectileEmitterBundle, ThrusterBundle),
) {
    (
        PlayerShipMaterialMesh {
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
            restitution: Restitution {
                coefficient: INIT_SHIP_RESTITUTION,
                combine_rule: CoefficientCombineRule::Multiply, // extra bouncy for player's sake to not get quickly dribbled to death
            },
            gravity: GravityScale(0.),
            damping: Damping {
                linear_damping: AMBIENT_LINEAR_FRICTION_COEFFICIENT,
                angular_damping: AMBIENT_ANGULAR_FRICTION_COEFFICIENT,
            },
            tag: PlayerShipTag,
        },
        (
            ProjectileEmitterBundle::new(22., heading, Some(FireType::Primary)),
            ThrusterBundle::new(
                0.,
                0.,
                DEFAULT_THRUST_FORCE_MAGNITUDE,
                thruster_particle_texture.into(),
            ),
        ),
    )
}
