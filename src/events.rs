use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_particle_systems::*;
use bevy_rapier2d::{dynamics::Velocity, pipeline::CollisionEvent};

use crate::{
    components::{AsteroidTag, CollisionRadius, PlayerShipTag, ProjectileTag},
    game::ThrustParticleTexture,
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

pub fn collide_projectile(
    mut commands: Commands,
    mut evr_collisions: EventReader<CollisionEvent>,
    q_proj: Query<(&Transform, &Velocity), (With<ProjectileTag>,)>,
    thrust_particle_texture: Res<ThrustParticleTexture>,
) {
    for event in evr_collisions.read() {
        match event {
            CollisionEvent::Started(ent_a, ent_b, _flags) => {
                if let Ok((transform, velocity)) = q_proj.get(*ent_a) {
                    emit_projectile_particles(
                        &mut commands,
                        &thrust_particle_texture,
                        transform,
                        velocity,
                    );
                }
                if let Ok((transform, velocity)) = q_proj.get(*ent_b) {
                    emit_projectile_particles(
                        &mut commands,
                        &thrust_particle_texture,
                        transform,
                        velocity,
                    );
                }
            }
            _ => {}
        }
    }
}

pub fn collide_asteroid_w_asteroid(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionAsteroidAsteroidEvent>,
    q_aster: Query<
        (&Transform, &CollisionRadius),
        (
            With<AsteroidTag>,
            Without<PlayerShipTag>,
            Without<ProjectileTag>,
        ),
    >,
    thrust_particle_texture: Res<ThrustParticleTexture>,
) {
    for CollisionAsteroidAsteroidEvent(aster_a, aster_b) in collision_events.read() {
        let aster_a_result = q_aster.get(*aster_a);
        let aster_b_result = q_aster.get(*aster_b);

        let (transform_a, r_a) = aster_a_result.unwrap();
        let (transform_b, _r_b) = aster_b_result.unwrap();
        dbg!(transform_a);

        let normalized = (transform_b.translation - transform_a.translation).normalize();
        let pos_perpendicular = Vec2::new(normalized.x, normalized.y).to_angle() + PI / 2.0;
        let neg_perpendicular = Vec2::new(normalized.x, normalized.y).to_angle() - PI / 2.0;
        let collision_pt = transform_a.translation + normalized * r_a.0;
        dbg!(collision_pt);

        commands
            .spawn(ParticleSystemBundle {
                particle_system: ParticleSystem {
                    max_particles: 200,
                    texture: thrust_particle_texture.0.clone().into(),
                    spawn_rate_per_second: 10.0.into(),
                    initial_speed: JitteredValue::jittered(20.0, -15.0..10.0),
                    lifetime: JitteredValue::jittered(2.0, -0.5..0.5),
                    color: ColorOverTime::Gradient(Curve::new(vec![
                        CurvePoint::new(Color::WHITE, 0.0),
                        CurvePoint::new(Color::rgba(1., 1., 1., 0.5), 0.2),
                        CurvePoint::new(Color::rgba(1.0, 1.0, 1.0, 0.1), 1.0),
                    ])),
                    emitter_shape: CircleSegment {
                        radius: 0.0.into(),
                        opening_angle: std::f32::consts::PI / 16.,
                        direction_angle: pos_perpendicular,
                    }
                    .into(),
                    looping: false,
                    rotate_to_movement_direction: true,
                    initial_rotation: (0_f32).to_radians().into(),
                    system_duration_seconds: 0.25,
                    max_distance: Some(100.0),
                    scale: 1.0.into(),
                    bursts: vec![ParticleBurst::new(0.0, 10)],
                    ..ParticleSystem::default()
                },
                transform: Transform::from_xyz(collision_pt.x, collision_pt.y, 0.0),
                global_transform: GlobalTransform::from_xyz(collision_pt.x, collision_pt.y, 0.0),
                ..ParticleSystemBundle::default()
            })
            .insert(Playing);
        // commands
        //     .spawn(ParticleSystemBundle {
        //         particle_system: ParticleSystem {
        //             max_particles: 200,
        //             texture: thrust_particle_texture.0.clone().into(),
        //             spawn_rate_per_second: 10.0.into(),
        //             initial_speed: JitteredValue::jittered(20.0, -15.0..10.0),
        //             lifetime: JitteredValue::jittered(2.0, -0.5..0.5),
        //             // color: ColorOverTime::Constant((Color::WHITE)),
        //             color: ColorOverTime::Gradient(Curve::new(vec![
        //                 CurvePoint::new(Color::WHITE, 0.0),
        //                 CurvePoint::new(Color::rgba(1., 1., 1., 0.5), 0.2),
        //                 CurvePoint::new(Color::rgba(1.0, 1.0, 1.0, 0.1), 1.0),
        //             ])),
        //             emitter_shape: CircleSegment {
        //                 radius: 0.0.into(),
        //                 opening_angle: std::f32::consts::PI / 16.,
        //                 direction_angle: neg_perpendicular,
        //             }
        //             .into(),
        //             looping: false,
        //             rotate_to_movement_direction: true,
        //             initial_rotation: (0_f32).to_radians().into(),
        //             system_duration_seconds: 0.25,
        //             max_distance: Some(100.0),
        //             scale: 1.0.into(),
        //             bursts: vec![ParticleBurst::new(0.0, 10)],
        //             ..ParticleSystem::default()
        //         },
        //         transform: Transform::from_xyz(collision_pt.x, collision_pt.y, 0.0),
        //         ..ParticleSystemBundle::default()
        //     })
        //     .insert(Playing);
    }
}

fn emit_projectile_particles(
    commands: &mut Commands,
    thrust_particle_texture: &Res<ThrustParticleTexture>,
    transform: &Transform,
    velocity: &Velocity,
) {
    let direction_angle = Vec2::from_angle(PI).rotate(velocity.linvel).to_angle();
    commands
        .spawn(ParticleSystemBundle {
            particle_system: ParticleSystem {
                max_particles: 6,
                texture: thrust_particle_texture.0.clone().into(),
                spawn_rate_per_second: 0.0.into(),
                // TODO scale with proj velocity
                initial_speed: JitteredValue::jittered(30.0, -10.0..0.0),
                lifetime: JitteredValue::jittered(0.5, -0.25..0.0),
                // color: ColorOverTime::Constant((Color::WHITE)),
                color: ColorOverTime::Gradient(Curve::new(vec![
                    CurvePoint::new(Color::WHITE, 0.0),
                    CurvePoint::new(Color::rgba(1., 1., 1., 0.5), 0.2),
                    CurvePoint::new(Color::rgba(1.0, 1.0, 1.0, 0.1), 1.0),
                ])),
                emitter_shape: CircleSegment {
                    radius: 0.0.into(),
                    opening_angle: std::f32::consts::PI / 2.0,
                    direction_angle,
                }
                .into(),
                looping: false,
                rotate_to_movement_direction: true,
                initial_rotation: (0_f32).to_radians().into(),
                system_duration_seconds: 0.25,
                max_distance: Some(100.0),
                scale: 1.0.into(),
                bursts: vec![ParticleBurst::new(0.0, 6)],
                ..ParticleSystem::default()
            },
            transform: Transform::from_xyz(transform.translation.x, transform.translation.y, 0.0),
            ..ParticleSystemBundle::default()
        })
        .insert(Playing);
}
