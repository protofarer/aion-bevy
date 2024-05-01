use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;
use bevy_particle_systems::{
    CircleSegment, ColorOverTime, Curve, CurvePoint, JitteredValue, Lerp, ParticleBurst,
    ParticleSystem, ParticleSystemBundle, Playing, ValueOverTime,
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
    effects::{CollisionEffectEvent, DestructionEffectEvent},
    events::{Avatars, CollisionAsteroidAsteroidEvent, CollisionProjectileEvent},
    game::ParticlePixelTexture,
    utils::Heading,
};



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

// TODO
// - SOLUTION: fixedupdate will emit effect events, data flows to Update systems, which handles perceivable effects
//   - thus is more like option A
// collision_sound: Res<ProjectileImpactSound>,
// destroy_ship_sound: Res<ShipDestroyedSound>,
// destroy_asteroid_sound: Res<AsteroidDestroyedSound>,
// asteroid_clash_sound: Res<AsteroidClashSound>,
pub fn handle_collisions(
    mut commands: Commands,
    mut evr_collisions: EventReader<CollisionEvent>,
    mut evw_collisions: EventWriter<CollisionEffectEvent>,
    mut score: ResMut<Score>,
    q_proj: Query<
        (Entity, &Damage, &Transform, &Velocity),
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
    for event in evr_collisions.read() {
        match event {
            CollisionEvent::Started(ent_a, ent_b, _flags) => {
                let proj_a_result = q_proj.get(*ent_a);
                let proj_b_result = q_proj.get(*ent_b);

                if let Ok((id, damage, transform, velocity)) = proj_a_result {
                    // emit projectile impact effect
                    evw_collisions.send(CollisionEffectEvent {
                        id,
                        avatar: Avatars::Projectile,
                        transform: Some(*transform),
                        velocity: Some(*velocity),
                    });
                    commands.entity(id).insert(DespawnDelay(Timer::new(
                        Duration::from_secs_f32(2.0),
                        TimerMode::Once,
                    )));
                }

                if let Ok((id, damage, transform, velocity)) = proj_b_result {
                    // emit projectile impact effect
                    evw_collisions.send(CollisionEffectEvent {
                        id,
                        avatar: Avatars::Projectile,
                        transform: Some(*transform),
                        velocity: Some(*velocity),
                    });
                    commands.entity(id).insert(DespawnDelay(Timer::new(
                        Duration::from_secs_f32(2.0),
                        TimerMode::Once,
                    )));
                }

                // // proj-aster
                // {
                //     let aster_a_result = q_aster.get(*ent_a);
                //     let aster_b_result = q_aster.get(*ent_b);

                //     if [aster_a_result, aster_b_result].iter().any(|x| x.is_ok())
                //         && [proj_a_result, proj_b_result].iter().any(|x| x.is_ok())
                //     {
                //         let proj_id = if proj_a_result.is_ok() {
                //             *ent_a
                //         } else {
                //             *ent_b
                //         };

                //         let aster_id = if aster_a_result.is_ok() {
                //             *ent_a
                //         } else {
                //             *ent_b
                //         };

                //         if let Ok((mut aster_health, _)) = q_aster.get_mut(aster_id) {
                //             // TODO remove collider, add DespawnDelay component
                //             // TODO system_despawn_delay
                //             // ticks down delay timer then destroys ent
                //             if aster_health.0 <= 1 {
                //                 commands.spawn(AudioBundle {
                //                     source: destroy_asteroid_sound.0.clone(),
                //                     settings: PlaybackSettings::DESPAWN,
                //                 });
                //                 commands.entity(aster_id).despawn();
                //                 score.0 += 1;
                //             } else {
                //                 if let Ok(proj_dmg) = q_proj.get(proj_id) {
                //                     aster_health.0 -= proj_dmg.0;
                //                 }
                //             }
                //         }
                //     }
                // }

                // // aster-ship
                // {
                //     let ship_a_result = q_ship.get(*ent_a);
                //     let ship_b_result = q_ship.get(*ent_b);
                //     let aster_a_result = q_aster.get(*ent_a);
                //     let aster_b_result = q_aster.get(*ent_b);

                //     if [aster_a_result, aster_b_result].iter().any(|x| x.is_ok())
                //         && [ship_a_result, ship_b_result].iter().any(|x| x.is_ok())
                //     {
                //         let aster_id = if aster_a_result.is_ok() {
                //             *ent_a
                //         } else {
                //             *ent_b
                //         };

                //         let ship_id = if ship_a_result.is_ok() {
                //             *ent_a
                //         } else {
                //             *ent_b
                //         };

                //         if let Ok(mut ship_health) = q_ship.get_mut(ship_id) {
                //             if let Ok((_, aster_dmg)) = q_aster.get(aster_id) {
                //                 ship_health.0 -= aster_dmg.0;

                //                 if ship_health.0 <= 1 {
                //                     commands.spawn(AudioBundle {
                //                         source: destroy_ship_sound.0.clone(),
                //                         settings: PlaybackSettings::DESPAWN,
                //                     });
                //                     commands.entity(ship_id).despawn_recursive();
                //                 }
                //             }
                //         }
                //     }
                // }

                // // proj-ship
                // {
                //     let ship_a_result = q_ship.get(*ent_a);
                //     let ship_b_result = q_ship.get(*ent_b);
                //     let proj_a_result = q_proj.get(*ent_a);
                //     let proj_b_result = q_proj.get(*ent_b);

                //     if [proj_a_result, proj_b_result].iter().any(|x| x.is_ok())
                //         && [ship_a_result, ship_b_result].iter().any(|x| x.is_ok())
                //     {
                //         let proj_id = if proj_a_result.is_ok() {
                //             *ent_a
                //         } else {
                //             *ent_b
                //         };

                //         let ship_id = if ship_a_result.is_ok() {
                //             *ent_a
                //         } else {
                //             *ent_b
                //         };

                //         if let Ok(mut ship_health) = q_ship.get_mut(ship_id) {
                //             if ship_health.0 <= 1 {
                //                 commands.spawn(AudioBundle {
                //                     source: destroy_ship_sound.0.clone(),
                //                     settings: PlaybackSettings::DESPAWN,
                //                 });
                //                 commands.entity(ship_id).despawn();
                //             } else {
                //                 if let Ok(proj_dmg) = q_proj.get(proj_id) {
                //                     ship_health.0 -= proj_dmg.0;
                //                 }
                //             }
                //         }
                //     }
                // }

                // // aster-aster
                // {
                //     let aster_a_result = q_aster.get(*ent_a);
                //     let aster_b_result = q_aster.get(*ent_b);
                //     if [aster_a_result, aster_b_result].iter().all(|x| x.is_ok()) {
                //         commands.spawn(AudioBundle {
                //             source: asteroid_clash_sound.0.clone(),
                //             settings: PlaybackSettings::DESPAWN,
                //         });
                //     }
                // }
            }
            _ => {}
        }
    }
}

//
// pub fn emit_collision_effects(
//     mut commands: Commands,
//     mut ev_w: EventWriter<EffectsCollisionEvent>,
// )

// TODO move to effects.rs
// read health, emit destroy effects
// chain after handle_collisions
pub fn handle_destructions(
    mut commands: Commands,
    mut ev_w: EventWriter<DestructionEffectEvent>,
    mut q_health: Query<(Entity, &Health)>,
    q_aster: Query<(), With<AsteroidTag>>,
    q_ship: Query<(), With<PlayerShipTag>>,
) {
    for (ent_id, health) in q_health.iter() {
        if let Ok(()) = q_aster.get(ent_id) {
            if health.0 <= 0 {
                ev_w.send(DestructionEffectEvent {
                    translation: Vec2::new(0., 0.),
                    avatar: Avatars::Asteroid,
                });
            }
        }
        if let Ok(()) = q_ship.get(ent_id) {
            if health.0 <= 0 {
                ev_w.send(DestructionEffectEvent {
                    translation: Vec2::new(0., 0.),
                    avatar: Avatars::PlayerShip,
                });
            }
        }
    }
}
