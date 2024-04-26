use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;
use bevy_particle_systems::{
    CircleSegment, ColorOverTime, Curve, CurvePoint, JitteredValue, ParticleBurst, ParticleSystem,
    ParticleSystemBundle, Playing,
};
use bevy_rapier2d::prelude::*;

use crate::{
    audio::{
        AsteroidClashSound, AsteroidDestroyedSound, ProjectileImpactSound, ShipDamagedSound,
        ShipDestroyedSound, ShipThrustSound, ShipThrustSoundStopwatch,
    },
    avatars::Thrust,
    components::{
        AsteroidTag, CollisionRadius, Damage, DespawnDelay, Health, PlayerShipTag, ProjectileTag,
        Score,
    },
    game::ThrustParticleTexture,
    utils::Heading,
};

pub fn emit_thruster_particles(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut q_ship: Query<&Children, With<PlayerShipTag>>,
    mut q_particle_system_child: Query<Entity, (With<Thrust>, With<ParticleSystem>)>,
) {
    if keyboard_input.pressed(KeyCode::KeyS) {
        for children in q_ship.iter_mut() {
            for child in children {
                if let Ok(ent_id) = q_particle_system_child.get_mut(*child) {
                    commands.entity(ent_id).insert(Playing);
                }
            }
        }
    }
    if keyboard_input.just_released(KeyCode::KeyS) {
        for children in q_ship.iter_mut() {
            for child in children {
                if let Ok(ent_id) = q_particle_system_child.get_mut(*child) {
                    commands.entity(ent_id).remove::<Playing>();
                }
            }
        }
    }
}

pub fn emit_collision_particles(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    // mut contact_force_events: EventReader<ContactForceEvent>,
    q_aster: Query<
        (&Transform, &CollisionRadius),
        (
            With<AsteroidTag>,
            Without<PlayerShipTag>,
            Without<ProjectileTag>,
        ),
    >,
    q_proj: Query<
        (&Transform, &Velocity),
        (
            With<ProjectileTag>,
            Without<AsteroidTag>,
            Without<PlayerShipTag>,
        ),
    >,
    // mut q_ship: Query<
    //     &Transform,
    //     (
    //         With<PlayerShipTag>,
    //         Without<AsteroidTag>,
    //         Without<ProjectileTag>,
    //     ),
    // >,
    thrust_particle_texture: Res<ThrustParticleTexture>,
) {
    for event in collision_events.read() {
        match event {
            CollisionEvent::Started(ent_a, ent_b, _flags) => {
                // TODO refactor into a function: f(q_aster, ent_a, ent_b) -> (bool, result_a.unwrapped, result_b.unwrapped)
                let aster_a_result = q_aster.get(*ent_a);
                let aster_b_result = q_aster.get(*ent_b);
                if [aster_a_result, aster_b_result].iter().all(|x| x.is_ok()) {
                    let (transform_a, r_a) = aster_a_result.unwrap();
                    let (transform_b, _r_b) = aster_b_result.unwrap();

                    let normalized =
                        (transform_b.translation - transform_a.translation).normalize();
                    let pos_perpendicular =
                        Vec2::new(normalized.x, normalized.y).to_angle() + PI / 2.0;
                    let neg_perpendicular =
                        Vec2::new(normalized.x, normalized.y).to_angle() - PI / 2.0;
                    let collision_pt = transform_a.translation + normalized * r_a.0;

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
                            ..ParticleSystemBundle::default()
                        })
                        .insert(Playing);
                    commands
                        .spawn(ParticleSystemBundle {
                            particle_system: ParticleSystem {
                                max_particles: 200,
                                texture: thrust_particle_texture.0.clone().into(),
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

                if let Ok((transform, velocity)) = q_proj.get(*ent_a) {
                    // f(commands, transf, vel) {}
                    let direction_angle = Vec2::from_angle(PI).rotate(velocity.linvel).to_angle();

                    // TODO gen_projectile_impact_particle_bundle(texture, translation, )
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
                            transform: Transform::from_xyz(
                                transform.translation.x,
                                transform.translation.y,
                                0.0,
                            ),
                            ..ParticleSystemBundle::default()
                        })
                        .insert(Playing);
                }

                if let Ok((transform, velocity)) = q_proj.get(*ent_b) {
                    // f(commands, transf, vel) {}
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
                            transform: Transform::from_xyz(
                                transform.translation.x,
                                transform.translation.y,
                                0.0,
                            ),
                            ..ParticleSystemBundle::default()
                        })
                        .insert(Playing);
                }
            }
            _ => {}
        }
    }
}

pub fn apply_forces_ship(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut q_ship: Query<(&Children, &mut ExternalForce, &Transform), With<PlayerShipTag>>,
    mut q_thruster: Query<&Thrust>,
    thrust_sound: Res<ShipThrustSound>,
    mut thrust_sound_stopwatch: ResMut<ShipThrustSoundStopwatch>,
    time: Res<Time>,
) {
    for (children, mut ext_force, transform) in q_ship.iter_mut() {
        // clear all external forces and torques on ship
        *ext_force = ExternalForce::default();

        thrust_sound_stopwatch.0.tick(time.delta());

        if keyboard_input.pressed(KeyCode::KeyS) {
            let mut sum_forces: f32 = 0.;
            for child in children {
                if let Ok(thruster) = q_thruster.get_mut(*child) {
                    sum_forces += thruster.0;
                }
            }

            let heading: Heading = transform.rotation.into();
            ext_force.force.x += heading.x() * sum_forces;
            ext_force.force.y += heading.y() * sum_forces;

            if thrust_sound_stopwatch.0.elapsed() >= Duration::from_secs_f32(0.3) {
                thrust_sound_stopwatch.0.reset();
                commands.spawn(AudioBundle {
                    source: thrust_sound.0.clone(),
                    ..default()
                });
            }
        }
        if keyboard_input.just_pressed(KeyCode::KeyS) {
            commands.spawn(AudioBundle {
                source: thrust_sound.0.clone(),
                ..default()
            });
        }
    }
}

pub fn handle_projectile_collision_events(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    collision_sound: Res<ProjectileImpactSound>,
    damage_ship_sound: Res<ShipDamagedSound>,
    destroy_ship_sound: Res<ShipDestroyedSound>,
    destroy_asteroid_sound: Res<AsteroidDestroyedSound>,
    asteroid_clash_sound: Res<AsteroidClashSound>,
    mut score: ResMut<Score>,
    q_proj: Query<
        &Damage,
        (
            With<ProjectileTag>,
            Without<AsteroidTag>,
            Without<PlayerShipTag>,
        ),
    >,
    mut q_ship: Query<
        &mut Health,
        (
            With<PlayerShipTag>,
            Without<ProjectileTag>,
            Without<AsteroidTag>,
        ),
    >,
    mut q_aster: Query<
        (&mut Health, &Damage),
        (
            With<AsteroidTag>,
            Without<PlayerShipTag>,
            Without<ProjectileTag>,
        ),
    >,
) {
    for event in collision_events.read() {
        match event {
            CollisionEvent::Started(ent_a, ent_b, _flags) => {
                let proj_a_result = q_proj.get(*ent_a);
                let proj_b_result = q_proj.get(*ent_b);

                if proj_a_result.is_ok() {
                    commands.entity(*ent_a).insert(DespawnDelay(Timer::new(
                        Duration::from_secs_f32(2.0),
                        TimerMode::Once,
                    )));
                }

                if proj_b_result.is_ok() {
                    commands.entity(*ent_b).insert(DespawnDelay(Timer::new(
                        Duration::from_secs_f32(1.0),
                        TimerMode::Once,
                    )));
                }

                if [proj_a_result, proj_b_result].iter().any(|x| x.is_ok()) {
                    commands.spawn(AudioBundle {
                        source: collision_sound.0.clone(),
                        settings: PlaybackSettings::DESPAWN,
                    });
                }

                // proj-aster
                {
                    let aster_a_result = q_aster.get(*ent_a);
                    let aster_b_result = q_aster.get(*ent_b);

                    if [aster_a_result, aster_b_result].iter().any(|x| x.is_ok())
                        && [proj_a_result, proj_b_result].iter().any(|x| x.is_ok())
                    {
                        let proj_id = if proj_a_result.is_ok() {
                            *ent_a
                        } else {
                            *ent_b
                        };

                        let aster_id = if aster_a_result.is_ok() {
                            *ent_a
                        } else {
                            *ent_b
                        };

                        if let Ok((mut aster_health, _)) = q_aster.get_mut(aster_id) {
                            // TODO remove collider, add DespawnDelay component
                            // TODO system_despawn_delay
                            // ticks down delay timer then destroys ent
                            if aster_health.0 <= 1 {
                                commands.spawn(AudioBundle {
                                    source: destroy_asteroid_sound.0.clone(),
                                    settings: PlaybackSettings::DESPAWN,
                                });
                                commands.entity(aster_id).despawn();
                                score.0 += 1;
                            } else {
                                if let Ok(proj_dmg) = q_proj.get(proj_id) {
                                    aster_health.0 -= proj_dmg.0;
                                }
                            }
                        }
                    }
                }

                // aster-ship
                {
                    let ship_a_result = q_ship.get(*ent_a);
                    let ship_b_result = q_ship.get(*ent_b);
                    let aster_a_result = q_aster.get(*ent_a);
                    let aster_b_result = q_aster.get(*ent_b);

                    if [aster_a_result, aster_b_result].iter().any(|x| x.is_ok())
                        && [ship_a_result, ship_b_result].iter().any(|x| x.is_ok())
                    {
                        let aster_id = if aster_a_result.is_ok() {
                            *ent_a
                        } else {
                            *ent_b
                        };

                        let ship_id = if ship_a_result.is_ok() {
                            *ent_a
                        } else {
                            *ent_b
                        };

                        if let Ok(mut ship_health) = q_ship.get_mut(ship_id) {
                            if ship_health.0 <= 1 {
                                commands.spawn(AudioBundle {
                                    source: destroy_ship_sound.0.clone(),
                                    settings: PlaybackSettings::DESPAWN,
                                });
                                commands.entity(ship_id).despawn();
                            } else {
                                if let Ok((_, aster_dmg)) = q_aster.get(aster_id) {
                                    ship_health.0 -= aster_dmg.0;
                                    commands.spawn(AudioBundle {
                                        source: damage_ship_sound.0.clone(),
                                        settings: PlaybackSettings::DESPAWN,
                                    });
                                }
                            }
                        }
                    }
                }

                // proj-ship
                {
                    let ship_a_result = q_ship.get(*ent_a);
                    let ship_b_result = q_ship.get(*ent_b);
                    let proj_a_result = q_proj.get(*ent_a);
                    let proj_b_result = q_proj.get(*ent_b);

                    if [proj_a_result, proj_b_result].iter().any(|x| x.is_ok())
                        && [ship_a_result, ship_b_result].iter().any(|x| x.is_ok())
                    {
                        let proj_id = if proj_a_result.is_ok() {
                            *ent_a
                        } else {
                            *ent_b
                        };

                        let ship_id = if ship_a_result.is_ok() {
                            *ent_a
                        } else {
                            *ent_b
                        };

                        if let Ok(mut ship_health) = q_ship.get_mut(ship_id) {
                            if ship_health.0 <= 1 {
                                commands.spawn(AudioBundle {
                                    source: destroy_ship_sound.0.clone(),
                                    settings: PlaybackSettings::DESPAWN,
                                });
                                commands.entity(ship_id).despawn();
                            } else {
                                if let Ok(proj_dmg) = q_proj.get(proj_id) {
                                    ship_health.0 -= proj_dmg.0;
                                    commands.spawn(AudioBundle {
                                        source: damage_ship_sound.0.clone(),
                                        settings: PlaybackSettings::DESPAWN,
                                    });
                                }
                            }
                        }
                    }
                }

                // aster-aster
                {
                    let aster_a_result = q_aster.get(*ent_a);
                    let aster_b_result = q_aster.get(*ent_b);
                    if [aster_a_result, aster_b_result].iter().all(|x| x.is_ok()) {
                        commands.spawn(AudioBundle {
                            source: asteroid_clash_sound.0.clone(),
                            settings: PlaybackSettings::DESPAWN,
                        });
                    }
                }
            }
            _ => {}
        }
    }
}
