use std::time::{Duration, Instant};

use bevy::prelude::*;
use bevy_rapier2d::dynamics::ExternalForce;

use crate::{archetypes::ProjectileBundle, audio::{ProjectileEmitSound, ShipThrustSound, ShipThrustSoundStopwatch}, avatars::Thrust, components::{FireType, PlayerShipTag, ProjectileEmission, TurnRate}, effects::ThrustEffectEvent, game::OnPlayScreen, utils::Heading};

pub fn ship_turn(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &TurnRate), With<PlayerShipTag>>,
    time: Res<Time>,
) {
    for (mut transform, turnrate) in query.iter_mut() {
        // let mut thrust = 0.;
        // if keyboard_input.pressed(KeyCode::KeyS) {
        //     thrust += 1.;
        // }
        // get fwd vector by applying current rot to ships init facing vec
        // let movement_direction = (transform.rotation * *DEFAULT_HEADING) * Vec3::X;
        // let movement_distance = thrust * movespeed.0 * time.delta_seconds();
        // let translation_delta = movement_direction * movement_distance;
        // transform.translation += translation_delta;

        let mut rotation_sign = 0.;
        if keyboard_input.pressed(KeyCode::KeyA) {
            rotation_sign += 1.;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            rotation_sign -= 1.;
        }
        transform.rotate_z(rotation_sign * turnrate.0 * time.delta_seconds());
    }
}

pub fn ship_fire(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut q_ship: Query<&Children, With<PlayerShipTag>>,
    mut q_emitter: Query<(&GlobalTransform, &mut ProjectileEmission, &FireType)>,
    fire_sound: Res<ProjectileEmitSound>,
) {
    // when fire key pressed
    if keyboard_input.pressed(KeyCode::Space) {
        // find ship, get children projectile emitters
        for children in &mut q_ship {
            for child in children {
                if let Ok((global_transform, mut emitter, firetype)) = q_emitter.get_mut(*child) {
                    // spawn primary fire projectile
                    match firetype {
                        FireType::Primary => {
                            let last_emit = emitter.last_emission_time;

                            if last_emit.elapsed().as_millis() as i32 >= emitter.cooldown_ms {
                                emitter.last_emission_time = Instant::now();

                                let (_scale, rotation, translation) =
                                    global_transform.to_scale_rotation_translation();

                                commands
                                    .spawn(ProjectileBundle::new(
                                        translation.x,
                                        translation.y,
                                        Some(rotation.into()),
                                        Some(emitter.projectile_speed),
                                        None,
                                        Some(emitter.damage),
                                        None,
                                        None,
                                        Some(2.0),
                                    ))
                                    .insert(OnPlayScreen);
                                commands.spawn(AudioBundle {
                                    source: fire_sound.0.clone(),
                                    ..default()
                                });
                            }
                        }
                        _ => (),
                    };
                }
            }
        }
    }
}

pub fn thrust_ship(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut evw_thrust_effect: EventWriter<ThrustEffectEvent>,
    mut q_ship: Query<(Entity, &Children, &mut ExternalForce, &Transform), With<PlayerShipTag>>,
    mut q_thruster: Query<&Thrust>,
    thrust_sound: Res<ShipThrustSound>,
    mut thrust_sound_stopwatch: ResMut<ShipThrustSoundStopwatch>,
    time: Res<Time>,
) {
    for (ent_id, children, mut ext_force, transform) in q_ship.iter_mut() {
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
            evw_thrust_effect.send(ThrustEffectEvent {
                id: ent_id,
                is_thrusting: true,
            });
        }
        if keyboard_input.just_released(KeyCode::KeyS) {
            evw_thrust_effect.send(ThrustEffectEvent {
                id: ent_id,
                is_thrusting: false,
            });
        }
    }
}
