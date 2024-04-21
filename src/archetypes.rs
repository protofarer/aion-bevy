use bevy::{prelude::*, sprite::{Material2d, MaterialMesh2dBundle}};
use bevy_rapier2d::{dynamics::{GravityScale, RigidBody, Velocity}, geometry::{ActiveEvents, Collider}};
use rand::Rng;

use crate::{components::{AsteroidTag, Damage, Health}, utils::Heading, INIT_ASTEROID_DAMAGE, INIT_ASTEROID_HEALTH, INIT_ASTEROID_MOVESPEED};

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
    tag: AsteroidTag,
}

pub enum AsteroidSizes {
    Small,
    Medium,
    Large,
}

impl<M: Material2d> Asteroid<M> {
    pub fn new(
        mesh: Handle<Mesh>,
        material: Handle<M>,
        r: f32,
        x: f32,
        y: f32,
        velocity: Option<Velocity>,
        damage: Option<i32>,
    ) -> Self {
        let velocity = match velocity {
            Some(x) => x,
            None => Velocity {
                linvel: Heading::default().linvel(INIT_ASTEROID_MOVESPEED),
                ..default()
            },
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
            health: Health(INIT_ASTEROID_HEALTH),
            gravity: GravityScale(0.),
            tag: AsteroidTag,
        }
    }
}
