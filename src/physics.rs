use bevy::{prelude::*, sprite::Material2d};
use bevy_rapier2d::{prelude::*, rapier::dynamics::RigidBodyVelocity};

use crate::{
    avatars::{Heading, PlayerShip, Thruster},
    components::{Player, PrimaryThrustMagnitude},
};

pub fn physics_plugin(app: &mut App) {
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(2.))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_physics)
        .add_systems(FixedUpdate, (apply_forces_ship));
    // .add_systems(Update, print_ball_altitude);
}

pub fn setup_physics(mut commands: Commands) {
    commands
        .spawn(Collider::cuboid(350., 25.))
        .insert(TransformBundle::from(Transform::from_xyz(-350., -50., 0.)));
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(10.))
        .insert(Velocity::linear(Vec2::new(0., -300.)))
        .insert(ExternalForce {
            force: Vec2::ZERO,
            torque: 0.,
        })
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(-300., 200., 0.)))
        .insert(GravityScale(1.));
}

pub fn apply_forces_ship(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut q_ship: Query<(&Children, &mut ExternalForce, &Transform, &mut Velocity), With<Player>>,
    mut q_thruster: Query<(&Thruster)>,
) {
    let (children, mut ext_force, transform, mut velocity) = q_ship.single_mut();

    // clear all external forces and torques on ship
    *ext_force = ExternalForce::default();

    if keyboard_input.pressed(KeyCode::KeyS) {
        let mut sum_forces: f32 = 0.;
        for child in children {
            if let Ok((thruster)) = q_thruster.get_mut(*child) {
                sum_forces += thruster.0;
            }
        }
        let heading: Heading = transform.rotation.into();
        ext_force.force.x += heading.0.x * sum_forces;
        ext_force.force.y += heading.0.y * sum_forces;
    }
}
