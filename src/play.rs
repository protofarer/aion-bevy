use bevy::prelude::*;
use bevy_vector_shapes::{painter::ShapePainter, shapes::LinePainter};

use crate::{
    avatars::{Boxoid, PlayerShip, Projectile},
    GameState, BOTTOM_WALL, INIT_HEALTH, INIT_LIVES, INIT_SHIP_MOVE_SPEED,
    INIT_SHIP_PROJECTILE_MOVE_SPEED, INIT_SHIP_ROTATION, INIT_SHIP_TURN_RATE, LEFT_WALL,
    RIGHT_WALL, TOP_WALL,
};

pub fn play_plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (
            apply_velocity,
            move_ship,
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

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct PlaySet;

#[derive(Component)]
pub enum Player {
    A,
    B,
}

#[derive(Component)]
pub struct Health(pub usize);

impl Default for Health {
    fn default() -> Self {
        Self(INIT_HEALTH)
    }
}

#[derive(Component)]
pub struct ShipStats {
    move_speed: f32,
    turn_speed: f32,
}

impl Default for ShipStats {
    fn default() -> Self {
        Self {
            move_speed: INIT_SHIP_MOVE_SPEED,
            turn_speed: INIT_SHIP_TURN_RATE,
        }
    }
}

#[derive(Component)]
pub struct ProjectileStats {
    pub move_speed: f32,
}

impl Default for ProjectileStats {
    fn default() -> Self {
        Self {
            move_speed: INIT_SHIP_PROJECTILE_MOVE_SPEED,
        }
    }
}

#[derive(Component, Deref, DerefMut, Debug)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Collider;

#[derive(Component)]
pub struct ScoreboardUi(Player);

#[derive(Component)]
pub struct OnMatchView;

#[derive(Component, Clone)]
pub struct OnEndScreen;

pub fn setup_play(
    // mut scores: ResMut<Scores>,
    // mut match_: ResMut<MatchInfo>,
    // mut next_state: ResMut<NextState<RoundState>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    info!("IN setup_match");
    commands.spawn(PlayerShip::default());
    commands.spawn(Boxoid::new(400., 200.));
    commands.spawn(Projectile::new(500., 200., None, None, Some(Color::RED)));

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
        // TODO apply ship speed in directio of its rotation vector
        // transform.rotation
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

pub fn move_ship(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &ShipStats)>,
    time: Res<Time>,
) {
    for (mut transform, ship) in query.iter_mut() {
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

        transform.rotate_z(rotation_sign * ship.turn_speed * time.delta_seconds());

        // get fwd vector by applying current rot to ships init facing vec
        let movement_direction = (transform.rotation * INIT_SHIP_ROTATION) * Vec3::X;
        let movement_distance = thrust * ship.move_speed * time.delta_seconds();
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
