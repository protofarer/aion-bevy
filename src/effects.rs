use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_particle_systems::{
    CircleSegment, ColorOverTime, Curve, CurvePoint, EmitterShape, JitteredValue, Lerp,
    ParticleBurst, ParticleSystem, ParticleSystemBundle, Playing, ValueOverTime,
};
use bevy_rapier2d::dynamics::Velocity;

use crate::{
    audio::{
        AsteroidClashSound, AsteroidDestroyedSound, ProjectileImpactSound, ShipDamagedSound,
        VesselDestroyedSound,
    },
    avatars::Thrust,
    components::{CollisionRadius, PlayerShipTag},
    events::Avatars,
    game::ParticlePixelTexture,
};

// Produce effects in update schedule: collision/death sounds and particles

// CollisionEffectEvent is a low broad early development event created to handle data passing between FixedUpdate collisions and Update collision effects
// These are generaly perceivable effects generated from the interaction (collision) between two avatars (entity consisting of a perceivable, interactable game object aka game actor)
// I expect this event to be refactored as needed, with higher or lower granularity as development proceeds
// For now (5/1/2024), I want to minimize making many systems and adding overt complications to game systems code
#[derive(Event)]
pub struct CollisionEffectEvent {
    pub avatar_a: Avatars,
    pub ent_a: Option<Entity>,
    pub transform_a: Option<Transform>,
    pub velocity_a: Option<Velocity>,
    pub collision_radius_a: Option<CollisionRadius>,
    pub avatar_b: Option<Avatars>,
    pub ent_b: Option<Entity>,
    pub transform_b: Option<Transform>,
}

impl Default for CollisionEffectEvent {
    fn default() -> Self {
        Self {
            avatar_a: Avatars::Other,
            ent_a: None,
            transform_a: None,
            velocity_a: None,
            collision_radius_a: None,
            avatar_b: None,
            ent_b: None,
            transform_b: None,
        }
    }
}

#[derive(Event)]
pub struct DestructionEffectEvent {
    pub transform: Transform,
    pub avatar: Avatars,
}

#[derive(Event)]
pub struct ThrustEffectEvent {
    pub id: Entity,
    pub is_thrusting: bool,
}

pub fn handle_collision_effects(
    mut commands: Commands,
    mut evr_coll_effects: EventReader<CollisionEffectEvent>,
    particle_pixel_texture: Res<ParticlePixelTexture>,
    proj_coll_sound: Res<ProjectileImpactSound>,
    damage_ship_sound: Res<ShipDamagedSound>,
    asteroid_clash_sound: Res<AsteroidClashSound>,
) {
    for event in evr_coll_effects.read() {
        match event.avatar_a {
            Avatars::Projectile => {
                // play sound, emit particles
                emit_projectile_collision_particles(
                    &mut commands,
                    &particle_pixel_texture,
                    &event.transform_a.unwrap_or_default(),
                    &event.velocity_a.unwrap_or_default(),
                );
                commands.spawn(AudioBundle {
                    source: proj_coll_sound.0.clone(),
                    settings: PlaybackSettings::DESPAWN,
                });
            }
            Avatars::PlayerShip => {
                emit_ship_collision_particles(
                    &mut commands,
                    &event.transform_a.unwrap_or_default(),
                    &particle_pixel_texture,
                );
                commands.spawn(AudioBundle {
                    source: damage_ship_sound.0.clone(),
                    settings: PlaybackSettings::DESPAWN,
                });
            }
            Avatars::Asteroid => {
                match event.avatar_b {
                    Some(Avatars::Asteroid) => {
                        // emit clash parts
                        commands.spawn(AudioBundle {
                            source: asteroid_clash_sound.0.clone(),
                            settings: PlaybackSettings::DESPAWN,
                        });
                        emit_asteroid_w_asteroid_collision_particles(
                            &mut commands,
                            &particle_pixel_texture,
                            &event.transform_a.unwrap(),
                            &event.collision_radius_a.unwrap(),
                            &event.transform_b.unwrap(),
                        );
                    }
                    _ => {}
                }
                // particles asteroid-asteroid collision
                // sound clash
            }
            _ => {}
        }
    }
}

fn emit_projectile_collision_particles(
    commands: &mut Commands,
    particle_pixel_texture: &Res<ParticlePixelTexture>,
    transform: &Transform,
    velocity: &Velocity,
) {
    let direction_angle = Vec2::from_angle(PI).rotate(velocity.linvel).to_angle();
    commands
        .spawn(ParticleSystemBundle {
            particle_system: ParticleSystem {
                max_particles: 6,
                texture: particle_pixel_texture.0.clone().into(),
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

pub fn handle_destruction_effects(
    mut commands: Commands,
    mut ev_w: EventReader<DestructionEffectEvent>,
    destroy_asteroid_sound: Res<AsteroidDestroyedSound>,
    destroy_vessel_sound: Res<VesselDestroyedSound>,
    damage_ship_sound: Res<ShipDamagedSound>,
    particle_pixel_texture: Res<ParticlePixelTexture>,
) {
    for event in ev_w.read() {
        match event.avatar {
            Avatars::PlayerShip => {
                // Post v1.0: astral/cosmic/etheral body gameplay
                // commands.spawn(AudioBundle {
                //     source: destroy_soul_sound.0.clone(),
                //     settings: PlaybackSettings::DESPAWN,
                // });
                commands.spawn(AudioBundle {
                    source: destroy_vessel_sound.0.clone(),
                    settings: PlaybackSettings::DESPAWN,
                });
                commands.spawn(AudioBundle {
                    source: damage_ship_sound.0.clone(),
                    settings: PlaybackSettings::DESPAWN,
                });
                emit_ship_destruction_particles(
                    &mut commands,
                    &event.transform,
                    &particle_pixel_texture,
                );
            }
            Avatars::Asteroid => {
                commands.spawn(AudioBundle {
                    source: destroy_asteroid_sound.0.clone(),
                    settings: PlaybackSettings::DESPAWN,
                });
            }
            _ => {}
        }
    }
}

pub fn handle_thrust_effects(
    mut commands: Commands,
    mut evr_thrust_effect: EventReader<ThrustEffectEvent>,
    q_ship_children: Query<&Children, With<PlayerShipTag>>,
    mut q_particle_system: Query<Entity, (With<Thrust>, With<ParticleSystem>)>,
) {
    for ThrustEffectEvent {
        id: ent_id,
        is_thrusting,
    } in evr_thrust_effect.read()
    {
        if let Ok(children) = q_ship_children.get(*ent_id) {
            for child in children.iter() {
                if let Ok(ent_id) = q_particle_system.get_mut(*child) {
                    if *is_thrusting {
                        commands.entity(ent_id).insert(Playing);
                    } else {
                        commands.entity(ent_id).remove::<Playing>();
                    }
                }
            }
        }
    }
}

// pub fn update_collide_asteroid_w_asteroid(
//     mut commands: Commands,
//     mut evr_collisions: EventReader<CollisionEvent>,
//     q_proj: Query<(&Transform, &Velocity), (With<ProjectileTag>,)>,
//     q_aster: Query<
//         (&Transform, &CollisionRadius),
//         (
//             With<AsteroidTag>,
//             Without<PlayerShipTag>,
//             Without<ProjectileTag>,
//         ),
//     >,
//     particle_pixel_texture: Res<ParticlePixelTexture>,
// ) {
//     for event in evr_collisions.read() {
//         match event {
//             CollisionEvent::Started(ent_a, ent_b, _flags) => {
//                 let aster_a_result = q_aster.get(*ent_a);
//                 let aster_b_result = q_aster.get(*ent_b);

//                 if [aster_a_result, aster_b_result].iter().all(|x| x.is_ok()) {
//                     let (transform_a, r_a) = aster_a_result.unwrap();
//                     let (transform_b, _r_b) = aster_b_result.unwrap();
//                     emit_asteroid_w_asteroid_collision_particles(
//                         &mut commands,
//                         &particle_pixel_texture,
//                         transform_a,
//                         r_a,
//                         transform_b,
//                     );
//                 }
//             }
//             _ => {}
//         }
//     }
// }

fn emit_asteroid_w_asteroid_collision_particles(
    commands: &mut Commands,
    particle_pixel_texture: &ParticlePixelTexture,
    transform_a: &Transform,
    r_a: &CollisionRadius,
    transform_b: &Transform,
) {
    let normalized = (transform_b.translation - transform_a.translation).normalize();
    let pos_perpendicular = Vec2::new(normalized.x, normalized.y).to_angle() + PI / 2.0;
    let neg_perpendicular = Vec2::new(normalized.x, normalized.y).to_angle() - PI / 2.0;
    let collision_pt = transform_a.translation + normalized * r_a.0;

    commands
        .spawn(ParticleSystemBundle {
            particle_system: ParticleSystem {
                max_particles: 200,
                texture: particle_pixel_texture.0.clone().into(),
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
    commands
        .spawn(ParticleSystemBundle {
            particle_system: ParticleSystem {
                max_particles: 200,
                texture: particle_pixel_texture.0.clone().into(),
                spawn_rate_per_second: 10.0.into(),
                initial_speed: JitteredValue::jittered(20.0, -15.0..10.0),
                lifetime: JitteredValue::jittered(2.0, -0.5..0.5),
                // color: ColorOverTime::Constant((Color::WHITE)),
                color: ColorOverTime::Gradient(Curve::new(vec![
                    CurvePoint::new(Color::WHITE, 0.0),
                    CurvePoint::new(Color::rgba(1., 1., 1., 0.5), 0.2),
                    CurvePoint::new(Color::rgba(1.0, 1.0, 1.0, 0.1), 1.0),
                ])),
                emitter_shape: CircleSegment {
                    radius: 0.0.into(),
                    opening_angle: std::f32::consts::PI / 16.,
                    direction_angle: neg_perpendicular,
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
            ..ParticleSystemBundle::default()
        })
        .insert(Playing);
}

fn emit_ship_collision_particles(
    commands: &mut Commands,
    transform: &Transform,
    particle_pixel_texture: &ParticlePixelTexture,
) {
    commands
        .spawn(ParticleSystemBundle {
            particle_system: ParticleSystem {
                max_particles: 25,
                texture: particle_pixel_texture.0.clone().into(),
                spawn_rate_per_second: 0.0.into(),
                initial_speed: JitteredValue::jittered(175.0, -50.0..0.0),
                lifetime: JitteredValue::jittered(3.0, -0.5..0.0),
                color: ColorOverTime::Gradient(Curve::new(vec![
                    CurvePoint::new(Color::RED, 0.0),
                    CurvePoint::new(Color::BLACK, 0.5),
                    CurvePoint::new(Color::BLACK, 1.0),
                ])),
                looping: false,
                system_duration_seconds: 1.0,
                max_distance: Some(500.0),
                scale: 2.0.into(),
                // scale: ValueOverTime::Lerp(Lerp::new(2.0, 20.)),
                bursts: vec![ParticleBurst::new(0.0, 25)],
                ..ParticleSystem::default()
            },
            transform: Transform::from_xyz(transform.translation.x, transform.translation.y, 0.0),
            ..ParticleSystemBundle::default()
        })
        .insert(Playing);
}

fn emit_ship_destruction_particles(
    commands: &mut Commands,
    transform: &Transform,
    particle_pixel_texture: &ParticlePixelTexture,
) {
    //TODO scale with ship's contained/accumulated energy, aka spirit energy aka prana aka chi
    let n_particles = 1000;
    let n_burst = 200;
    let rate = 150.0;
    let duration = 6.0;
    commands
        .spawn(ParticleSystemBundle {
            particle_system: ParticleSystem {
                max_particles: n_particles,
                texture: particle_pixel_texture.0.clone().into(),
                spawn_rate_per_second: ValueOverTime::Lerp(Lerp::new(rate, 0.)),
                initial_speed: JitteredValue::jittered(200.0, -100.0..0.0),
                lifetime: JitteredValue::jittered(duration, (-duration * 0.3)..0.0),
                color: ColorOverTime::Gradient(Curve::new(vec![
                    CurvePoint::new(Color::RED, 0.0),
                    CurvePoint::new(Color::BLACK, 1.0),
                ])),
                emitter_shape: EmitterShape::CircleSegment(CircleSegment {
                    radius: 10.0.into(),
                    ..default()
                }),
                looping: false,
                system_duration_seconds: duration,
                max_distance: Some(1000.0),
                scale: 3.0.into(),
                bursts: vec![ParticleBurst::new(0.0, n_burst)],
                ..ParticleSystem::default()
            },
            transform: Transform::from_xyz(transform.translation.x, transform.translation.y, 0.0),
            ..ParticleSystemBundle::default()
        })
        .insert(Playing);
}
