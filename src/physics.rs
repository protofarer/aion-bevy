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
    effects::{CollisionEffectEvent, DestructionEffectEvent, ThrustEffectEvent},
    events::{Avatars, CollisionAsteroidAsteroidEvent, CollisionProjectileEvent},
    game::ParticlePixelTexture,
    utils::Heading,
};

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
    mut evw_effects_collisions: EventWriter<CollisionEffectEvent>,
    mut evw_effects_destruction: EventWriter<DestructionEffectEvent>,
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
        (Entity, &mut Health, &Transform),
        (
            With<PlayerShipTag>,
            Without<ProjectileTag>,
            Without<AsteroidTag>,
        ),
    >,
    mut q_aster: Query<
        (Entity, &mut Health, &Damage, &Transform, &CollisionRadius),
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
                let proj_a = q_proj.get(*ent_a).ok();
                let proj_b = q_proj.get(*ent_b).ok();
                let any_proj = proj_a.or(proj_b);

                let aster_a = q_aster.get(*ent_a).is_ok();
                let aster_b = q_aster.get(*ent_b).is_ok();
                let is_any_aster = aster_a || aster_b;
                let is_all_aster = aster_a && aster_b;

                let ship_a = q_ship.get(*ent_a).is_ok();
                let ship_b = q_ship.get(*ent_b).is_ok();
                let is_any_ship = ship_a || ship_b;

                // Projectile Collision Effect ONLY (not incl damage)
                if let Some((id, damage, transform, velocity)) = any_proj {
                    evw_effects_collisions.send(CollisionEffectEvent {
                        avatar_a: Avatars::Projectile,
                        ent_a: Some(id),
                        transform_a: Some(*transform),
                        velocity_a: Some(*velocity),
                        collision_radius_a: None,
                        ..default()
                    });
                    commands.entity(id).insert(DespawnDelay(Timer::new(
                        Duration::from_secs_f32(2.0),
                        TimerMode::Once,
                    )));
                }

                // proj-aster
                // No asteroid sound, simply projectile collision effects as above

                if is_any_aster && any_proj.is_some() {
                    let (aster_id, mut aster_health, _, aster_transform, _) = if aster_a {
                        q_aster.get_mut(*ent_a).unwrap()
                    } else {
                        q_aster.get_mut(*ent_b).unwrap()
                    };
                    let (_proj_id, proj_dmg, _proj_transform, _proj_velocity) = any_proj.unwrap();

                    aster_health.0 -= proj_dmg.0;

                    if aster_health.0 <= 0 {
                        score.0 += 1;
                        evw_effects_destruction.send(DestructionEffectEvent {
                            avatar: Avatars::Asteroid,
                            transform: *aster_transform,
                        });
                        commands.entity(aster_id).despawn_recursive();
                    }
                }

                // proj-ship
                if is_any_ship && any_proj.is_some() {
                    // (Entity, &mut Health, &Transform),
                    let (ship_id, mut ship_health, ship_transform) = if ship_a {
                        q_ship.get_mut(*ent_a).unwrap()
                    } else {
                        q_ship.get_mut(*ent_b).unwrap()
                    };
                    let (_proj_id, proj_dmg, _proj_transform, _proj_velocity) = any_proj.unwrap();

                    ship_health.0 -= proj_dmg.0;

                    if ship_health.0 <= 0 {
                        evw_effects_destruction.send(DestructionEffectEvent {
                            avatar: Avatars::PlayerShip,
                            transform: *ship_transform,
                        });
                        commands.entity(ship_id).despawn_recursive();
                    } else {
                        evw_effects_collisions.send(CollisionEffectEvent {
                            avatar_a: Avatars::PlayerShip,
                            transform_a: Some(*ship_transform),
                            ..default()
                        });
                    }
                }

                // // aster-aster
                if is_all_aster {
                    let (_, _, _, aster_a_transform, coll_r_a) = q_aster.get(*ent_a).unwrap();
                    let (_, _, _, aster_b_transform, _) = q_aster.get(*ent_b).unwrap();
                    evw_effects_collisions.send(CollisionEffectEvent {
                        avatar_a: Avatars::Asteroid,
                        transform_a: Some(*aster_a_transform),
                        collision_radius_a: Some(*coll_r_a),
                        avatar_b: Some(Avatars::Asteroid),
                        transform_b: Some(*aster_b_transform),
                        ..default()
                    });
                }

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
            }
            _ => {}
        }
    }
}
