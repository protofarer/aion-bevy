use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    avatars::{Heading, Thruster},
    components::Player,
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
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut q_ship: Query<(&Children, &mut ExternalForce, &Transform), With<Player>>,
    mut q_thruster: Query<&Thruster>,
) {
    let (children, mut ext_force, transform) = q_ship.single_mut();

    // clear all external forces and torques on ship
    *ext_force = ExternalForce::default();

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
    }
}

fn handle_projectile_collision_events(mut collision_events: EventReader<CollisionEvent>) {
    for event in collision_events.read() {
        info!("collision event {:?}", event);
    }

    // commands.spawn(AudioBundle {
    //     source: bg_music.0.clone(),
    //     settings: PlaybackSettings::DESPAWN,
    // });
}
