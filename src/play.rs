use bevy::prelude::*;
use bevy_vector_shapes::{painter::ShapePainter, shapes::LinePainter};

use crate::{BOTTOM_WALL, LEFT_WALL, PADDLE_SPEED, RIGHT_WALL, TOP_WALL};

pub fn play_plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (
            apply_velocity,
            move_paddle,
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
    );
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

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct PlaySet;

#[derive(Component)]
pub enum Player {
    A,
    B,
}

#[derive(Component)]
pub struct Paddle;

#[derive(Component, Deref, DerefMut, Debug)]
pub struct Velocity(pub Vec2);

pub fn setup_play(
    // mut scores: ResMut<Scores>,
    // mut match_: ResMut<MatchInfo>,
    // mut next_state: ResMut<NextState<RoundState>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    info!("IN setup_match");
    // scores.a = 0;
    // scores.b = 0;
    // match_.round_count = 0;

    // Paddle A
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(
                    LEFT_WALL + (RIGHT_WALL - LEFT_WALL) / 2.,
                    BOTTOM_WALL + (TOP_WALL - BOTTOM_WALL) / 2.,
                    0.,
                ),
                scale: Vec3::new(50., 50., 0.0),
                ..default()
            },
            sprite: Sprite {
                color: Color::rgb(1., 1., 1.),
                ..default()
            },
            ..default()
        },
        Paddle,
        Player::A,
        // Collider,
        // OnMatchView,
    ));

    // Ball
    // commands.spawn((
    //     MaterialMesh2dBundle {
    //         mesh: meshes.add(Circle::default()).into(),
    //         material: materials.add(BALL_COLOR),
    //         transform: Transform::from_translation(BALL_START_POSITION)
    //             .with_scale(Vec2::splat(BALL_RADIUS * 2.).extend(1.)),
    //         ..default()
    //     },
    //     Ball,
    //     Velocity(BALL_START_VELOCITY),
    //     OnMatchView,
    // ));

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

pub fn move_paddle(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &Player), With<Paddle>>,
    time: Res<Time>,
) {
    for (mut transform, player) in query.iter_mut() {
        let top_bound = TOP_WALL;
        let bottom_bound = BOTTOM_WALL;
        let left_bound = LEFT_WALL;
        let right_bound = RIGHT_WALL;

        match player {
            Player::A => {
                let mut y_direction = 0.;
                let mut x_direction = 0.;

                if keyboard_input.pressed(KeyCode::KeyW) {
                    y_direction += 1.;
                }
                if keyboard_input.pressed(KeyCode::KeyS) {
                    y_direction -= 1.;
                }
                if keyboard_input.pressed(KeyCode::KeyD) {
                    x_direction += 1.;
                }
                if keyboard_input.pressed(KeyCode::KeyA) {
                    x_direction -= 1.;
                }

                let new_paddle_y_position =
                    transform.translation.y + y_direction * PADDLE_SPEED * time.delta_seconds();
                transform.translation.y = new_paddle_y_position.clamp(bottom_bound, top_bound);

                let new_paddle_x_position =
                    transform.translation.x + x_direction * PADDLE_SPEED * time.delta_seconds();
                transform.translation.x = new_paddle_x_position.clamp(left_bound, right_bound);
            }
            _ => {} // Player::B => {
                    //     let mut direction = 0.;

                    //     if keyboard_input.pressed(KeyCode::ArrowUp) {
                    //         direction += 1.;
                    //     }
                    //     if keyboard_input.pressed(KeyCode::ArrowDown) {
                    //         direction -= 1.;
                    //     }

                    //     let new_paddle_position =
                    //         transform.translation.y + direction * PADDLE_SPEED * time.delta_seconds();

                    //     transform.translation.y = new_paddle_position.clamp(bottom_bound, top_bound);
                    // }
        }
    }
}
