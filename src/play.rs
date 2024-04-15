use core::time;

use bevy::utils::{Duration, Instant};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_vector_shapes::{painter::ShapePainter, shapes::LinePainter};
use rand::Rng;

use crate::components::{BodyRotationRate, MoveSpeed, Player, TurnRate, Velocity};
use crate::DEFAULT_MOVESPEED;
use crate::{
    avatars::{Asteroid, Boxoid, Heading, PlayerShip, Projectile},
    GameState, Speed, BOTTOM_WALL, INIT_ASTEROID_MOVE_SPEED, INIT_SHIP_HEALTH,
    INIT_SHIP_MOVE_SPEED, INIT_SHIP_PROJECTILE_MOVE_SPEED, INIT_SHIP_ROTATION, INIT_SHIP_TURN_RATE,
    LARGE_ASTEROID_R, LEFT_WALL, MEDIUM_ASTEROID_R, RIGHT_WALL, SMALL_ASTEROID_R, TOP_WALL,
};

pub fn play_plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (
            apply_velocity,
            move_ship,
            apply_body_rotation,
            // check_for_collisions,
            // play_collision_sound,
            // process_score,
        )
            .chain()
            .in_set(PlaySet),
    )
    .add_systems(Startup, setup_play)
    .add_systems(
        Update,
        (
            bevy::window::close_on_esc,
            draw_boundary, // run_end.run_if(in_state(GameState::End)),
                           // (update_score_ui, bevy::window::close_on_esc, run_match).in_set(MatchSet),
        ),
    )
    .configure_sets(Update, (PlaySet.run_if(in_state(GameState::Match))))
    .configure_sets(FixedUpdate, (PlaySet.run_if(in_state(GameState::Match))));
    // .add_systems(OnEnter(GameState::Match), setup_match)
    // .add_systems(OnEnter(GameState::End), setup_end)
    // .add_systems(OnExit(GameState::Match), despawn_screen::<OnMatchView>)
    // .add_systems(OnExit(GameState::End), despawn_screen::<OnEndScreen>)
    // .configure_sets(
    //     Update,
    //     (
    //         PlaySet.run_if(in_state(RoundState::In)),
    //         MatchSet.run_if(in_state(GameState::Match)),
    //     ),
    // )
    // .configure_sets(FixedUpdate, (PlaySet.run_if(in_state(RoundState::In)),))
    // .add_event::<CollisionEvent>()
    // .add_event::<ScoreEvent>();
}

pub fn setup_play(
    // mut scores: ResMut<Scores>,
    // mut match_: ResMut<MatchInfo>,
    // mut next_state: ResMut<NextState<RoundState>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    info!("IN setup_match");

    // Player Ships
    let handle_playership_mesh = meshes.add(Triangle2d::new(
        Vec2::Y * 22.,
        Vec2::new(-15., -15.),
        Vec2::new(15., -15.),
    ));
    let handle_playership_colormaterial = materials.add(Color::LIME_GREEN);
    commands.spawn(PlayerShip::new(
        0.,
        0.,
        None,
        handle_playership_mesh.clone(),
        handle_playership_colormaterial,
    ));

    // Asteroids
    let handle_asteroid_colormaterial = materials.add(Color::GRAY);
    // let n = 15.;
    // let dx = (RIGHT_WALL - LEFT_WALL) / n;
    // let mut i = 0;
    // for sides in [5, 6, 8] {
    //     for r in [SMALL_ASTEROID_R, MEDIUM_ASTEROID_R, LARGE_ASTEROID_R] {
    //         let handle_polygon = meshes.add(RegularPolygon::new(r, sides));
    //         commands.spawn(Asteroid::new(
    //             LEFT_WALL + 50. + i as f32 * dx,
    //             250.,
    //             None,
    //             handle_polygon.clone(),
    //             handle_asteroid_colormaterial.clone(),
    //             r,
    //         ));
    //         i += 1;
    //     }
    // }
    let handle_polygon = meshes.add(RegularPolygon::new(MEDIUM_ASTEROID_R, 5));
    commands.spawn(Asteroid::new(
        0.,
        250.,
        MEDIUM_ASTEROID_R,
        handle_polygon.clone(),
        handle_asteroid_colormaterial.clone(),
        None,
        None,
        None,
    ));

    // // Scores
    // // A
    // commands.spawn((
    //     ScoreboardUi(Player::A),
    //     TextBundle::from_sections([TextSection::from_style(TextStyle {
    //         font_size: SCORE_FONT_SIZE,
    //         color: TEXT_COLOR,
    //         ..default()
    //     })])
    //     .with_style(Style {
    //         // position_type: PositionType::Relative,
    //         // top: Val::Px(100.),
    //         // left: Val::Percent(25.),
    //         top: SCORE_A_POSITION.top,
    //         left: SCORE_A_POSITION.left,
    //         ..default()
    //     }),
    //     OnMatchView,
    // ));
    // // B
    // commands.spawn((
    //     ScoreboardUi(Player::B),
    //     TextBundle::from_sections([TextSection::from_style(TextStyle {
    //         font_size: SCORE_FONT_SIZE,
    //         color: TEXT_COLOR,
    //         ..default()
    //     })])
    //     .with_style(Style {
    //         // position_type: PositionType::Relative,
    //         top: SCORE_B_POSITION.top,
    //         left: SCORE_B_POSITION.left,
    //         ..default()
    //     }),
    //     OnMatchView,
    // ));

    // commands.spawn((WallBundle::new(WallLocation::Bottom), OnMatchView));
    // commands.spawn((WallBundle::new(WallLocation::Top), OnMatchView));
    // commands.spawn((GoalBundle::new(GoalLocation::Left), OnMatchView));
    // commands.spawn((GoalBundle::new(GoalLocation::Right), OnMatchView));

    // next_state.set(RoundState::Countdown);
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct PlaySet;

pub fn draw_boundary(mut painter: ShapePainter) {
    let height = TOP_WALL - BOTTOM_WALL;
    let width = RIGHT_WALL - LEFT_WALL;
    let line_color = Color::WHITE;

    painter.thickness = 1.;
    painter.color = line_color;

    painter.line(
        Vec3::new(-width / 2., -height / 2., 0.),
        Vec3::new(-width / 2., height / 2., 0.),
    );
    painter.line(
        Vec3::new(-width / 2., height / 2., 0.),
        Vec3::new(width / 2., height / 2., 0.),
    );
    painter.line(
        Vec3::new(-width / 2., -height / 2., 0.),
        Vec3::new(width / 2., -height / 2., 0.),
    );
    painter.line(
        Vec3::new(width / 2., -height / 2., 0.),
        Vec3::new(width / 2., height / 2., 0.),
    );
}

pub fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

pub fn apply_body_rotation(mut query: Query<(&mut Transform, &BodyRotationRate)>, time: Res<Time>) {
    for (mut transform, brr) in &mut query {
        transform.rotate_z(brr.0);
    }
}
pub fn move_ship(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &TurnRate, &MoveSpeed), With<Player>>,
    time: Res<Time>,
) {
    for (mut transform, turnrate, movespeed) in query.iter_mut() {
        let mut thrust = 0.;
        let mut rotation_sign = 0.;

        if keyboard_input.pressed(KeyCode::KeyS) {
            thrust += 1.;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            rotation_sign += 1.;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            rotation_sign -= 1.;
        }

        transform.rotate_z(rotation_sign * turnrate.0 * time.delta_seconds());

        // get fwd vector by applying current rot to ships init facing vec
        let movement_direction = (transform.rotation * INIT_SHIP_ROTATION) * Vec3::X;
        let movement_distance = thrust * movespeed.0 * time.delta_seconds();
        let translation_delta = movement_direction * movement_distance;
        transform.translation += translation_delta;

        if (transform.translation.y >= TOP_WALL) {
            transform.translation.y = BOTTOM_WALL + (transform.translation.y - TOP_WALL);
        }
        if (transform.translation.y <= BOTTOM_WALL) {
            transform.translation.y = TOP_WALL - (BOTTOM_WALL - transform.translation.y);
        }
        if (transform.translation.x >= RIGHT_WALL) {
            transform.translation.x = LEFT_WALL + (transform.translation.x - RIGHT_WALL);
        }
        if (transform.translation.x <= LEFT_WALL) {
            transform.translation.x = RIGHT_WALL - (LEFT_WALL - transform.translation.x);
        }
    }
}
