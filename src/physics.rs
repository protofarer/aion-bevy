use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    audio::{
        AsteroidDestroyedSound, ProjectileEmitSound, ProjectileImpactSound, ShipDamagedSound,
        ShipThrustSound, ShipThrustSoundStopwatch,
    },
    avatars::Thruster,
    components::{AsteroidTag, Health, Player, PlayerShipTag, ProjectileTag},
    utils::Heading,
};

pub fn physics_plugin(app: &mut App) {
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(2.))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_physics)
        .add_systems(
            FixedUpdate,
            (apply_forces_ship, handle_projectile_collision_events).chain(),
        );
    // .add_systems(Update, print_ball_altitude);
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

pub fn apply_forces_ship(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut q_ship: Query<(&Children, &mut ExternalForce, &Transform), With<Player>>,
    mut q_thruster: Query<&Thruster>,
    thrust_sound: Res<ShipThrustSound>,
    mut thrust_sound_stopwatch: ResMut<ShipThrustSoundStopwatch>,
    time: Res<Time>,
) {
    let (children, mut ext_force, transform) = q_ship.single_mut();

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
        ext_force.force.x += heading.0.cos() * sum_forces;
        ext_force.force.y += heading.0.sin() * sum_forces;

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

fn handle_projectile_collision_events(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    collision_sound: Res<ProjectileImpactSound>,
    damage_ship_sound: Res<ShipDamagedSound>,
    destroy_asteroid_sound: Res<AsteroidDestroyedSound>,
    projectile_query: Query<&ProjectileTag>,
    ship_query: Query<&PlayerShipTag>,
    asteroid_query: Query<&mut Health, With<AsteroidTag>>,
) {
    for event in collision_events.read() {
        match event {
            CollisionEvent::Started(ent_a, ent_b, _flags) => {
                if projectile_query.get(*ent_a).is_ok() || projectile_query.get(*ent_b).is_ok() {
                    commands.spawn(AudioBundle {
                        source: collision_sound.0.clone(),
                        settings: PlaybackSettings::DESPAWN,
                    });

                    let asteroid_id = if asteroid_query.get(*ent_a).is_ok() {
                        Some(ent_a)
                    } else if asteroid_query.get(*ent_b).is_ok() {
                        Some(ent_b)
                    } else {
                        None
                    };

                    if let Some(asteroid_id) = asteroid_id {
                        // TODO if asteroid hits <= 0
                        // reduce asteroid hp
                        // if <= 0, destroy
                        // play destroy sound

                        commands.spawn(AudioBundle {
                            source: destroy_asteroid_sound.0.clone(),
                            settings: PlaybackSettings::DESPAWN,
                        });
                    }
                    if (ship_query.get(*ent_a).is_ok() || ship_query.get(*ent_b).is_ok())
                        || (asteroid_query.get(*ent_a).is_ok()
                            || asteroid_query.get(*ent_b).is_ok())
                    {
                        // TODO ship hp
                        commands.spawn(AudioBundle {
                            source: damage_ship_sound.0.clone(),
                            settings: PlaybackSettings::DESPAWN,
                        });
                    }
                }
            }
            _ => {}
        }
    }
}
