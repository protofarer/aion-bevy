use std::{borrow::BorrowMut, time::Duration};

use bevy::prelude::*;
use bevy_particle_systems::{ParticleSystem, Playing};
use bevy_rapier2d::prelude::*;

use crate::{
    audio::{
        AsteroidClashSound, AsteroidDestroyedSound, ProjectileEmitSound, ProjectileImpactSound,
        ShipDamagedSound, ShipDestroyedSound, ShipThrustSound, ShipThrustSoundStopwatch,
    },
    avatars::Thrust,
    components::{AsteroidTag, Damage, Health, Player, PlayerShipTag, ProjectileTag, Score},
    utils::Heading,
};

pub fn physics_plugin(app: &mut App) {
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(2.))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_physics)
        .add_systems(
            FixedUpdate,
            (apply_forces_ship, handle_projectile_collision_events).chain(),
        )
        .add_systems(Update, emit_thruster_particles);
}

pub fn setup_physics(mut commands: Commands) {
    // commands
    //     .spawn(Collider::cuboid(350., 25.))
    //     .insert(TransformBundle::from(Transform::from_xyz(-350., -50., 0.)));
    // commands
    //     .spawn(RigidBody::Dynamic)
    //     .insert(Collider::ball(10.))
    //     .insert(Velocity::linear(Vec2::new(0., -300.)))
    //     .insert(ExternalForce {
    //         force: Vec2::ZERO,
    //         torque: 0.,
    //     })
    //     .insert(Restitution::coefficient(0.7))
    //     .insert(TransformBundle::from(Transform::from_xyz(-300., 200., 0.)))
    //     .insert(GravityScale(1.));
}

pub fn emit_thruster_particles(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut q_ship: Query<(&Children), With<PlayerShipTag>>,
    mut q_particle_system_child: Query<Entity, (With<Thrust>, With<ParticleSystem>)>,
) {
    if keyboard_input.pressed(KeyCode::KeyS) {
        for (children) in q_ship.iter_mut() {
            for child in children {
                if let Ok(ent_id) = q_particle_system_child.get_mut(*child) {
                    commands.entity(ent_id).insert(Playing);
                }
            }
        }
    }
    if keyboard_input.just_released(KeyCode::KeyS) {
        for (children) in q_ship.iter_mut() {
            for child in children {
                if let Ok(ent_id) = q_particle_system_child.get_mut(*child) {
                    commands.entity(ent_id).remove::<Playing>();
                }
            }
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

fn handle_projectile_collision_events(
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
                            commands.entity(proj_id).despawn();
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
