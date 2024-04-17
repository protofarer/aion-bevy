use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn physics_plugin(app: &mut App) {
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(2.))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_physics)
        .add_systems(Update, print_ball_altitude);
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
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(-300., 200., 0.)))
        .insert(GravityScale(1.));
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
}
