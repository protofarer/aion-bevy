use bevy::{prelude::*, sprite::Material2d};
use bevy_rapier2d::{prelude::*, rapier::dynamics::RigidBodyVelocity};

use crate::{
    avatars::{Heading, PlayerShip},
    components::{Player, PrimaryThrustMagnitude},
};

pub fn physics_plugin(app: &mut App) {
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(2.))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_physics)
        .add_systems(FixedUpdate, apply_thrust);
    // .add_systems(Update, print_ball_altitude);
}

pub fn setup_physics(mut commands: Commands) {
    commands
        .spawn(Collider::cuboid(700., 50.))
        .insert(TransformBundle::from(Transform::from_xyz(0., -100., 0.)));

    // let mut gscale = 1.;
    // let mut r = 5.;
    // let mut x = 200.;
    // for i in (0..5) {
    //     commands
    //         .spawn(RigidBody::Dynamic)
    //         .insert(Collider::ball(r))
    //         .insert(Restitution::coefficient(0.7))
    //         .insert(TransformBundle::from(Transform::from_xyz(x, 400., 0.)))
    //         .insert(GravityScale(gscale));
    //     // gscale *= 2.;
    //     r += 5.;
    //     x += 50.;
    // }
    // commands
    //     .spawn(RigidBody::Dynamic)
    //     .insert(Collider::ball(50.))
    //     .insert(Restitution::coefficient(0.7))
    //     .insert(TransformBundle::from(Transform::from_xyz(200., 51., 0.)))
    //     .insert(GravityScale(1.));
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

pub fn apply_thrust(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut q_ship: Query<
        (
            Entity,
            // &mut Velocity,
            &mut ExternalForce,
            &Transform,
            &PrimaryThrustMagnitude,
        ),
        With<Player>,
    >,
) {
    let (
        id,
        // mut vel,
        mut primary_thrust_force,
        transform,
        primary_thrust_magnitude,
    ) = q_ship.single_mut();

    if keyboard_input.pressed(KeyCode::KeyS) {
        info!("Added thrust force");
        let heading: Heading = transform.rotation.into();
        primary_thrust_force.force =
            Vec2::new(heading.0.x, heading.0.y) * primary_thrust_magnitude.0;
        // force: Vec2::new(100000., 0.),
        //     torque: 0.,
        // };
        // commands.entity(id).insert(ExternalForce {
        //     force: Vec2::new(10000., 0.),
        //     torque: 0.,
        // });
    }
    if keyboard_input.just_released(KeyCode::KeyS) {
        info!("Removed thrust force");
        *primary_thrust_force = ExternalForce {
            force: Vec2::ZERO,
            torque: 0.,
        };
        // *vel = Velocity {
        //     linvel: Vec2::ZERO,
        //     angvel: 0.,
        // };
    }
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
}
