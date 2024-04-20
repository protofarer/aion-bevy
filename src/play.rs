use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::utils::Instant;
use bevy_rapier2d::prelude::*;
use bevy_vector_shapes::{painter::ShapePainter, shapes::LinePainter};

use crate::components::{
    BackgroundMusic, FireType, FireTypes, Player, ProjectileEmission, ProjectileTag, Score,
    ScoreboardUi, TurnRate,
};
use crate::{
    avatars::{Asteroid, PlayerShip, Projectile},
    utils::Heading,
    GameState, BOTTOM_WALL, LEFT_WALL, MEDIUM_ASTEROID_R, RIGHT_WALL, TOP_WALL,
};
use crate::{
    DEFAULT_ROTATION, LABEL_COLOR, SCOREBOARD_FONT_SIZE, SCOREBOARD_TEXT_PADDING, SCORE_COLOR,
};

pub fn play_plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (
            move_ship, wraparound,
            ship_fire,
            // check_for_collisions,
            // play_collision_sound,
            // process_score,
        )
            .chain()
            .in_set(PlaySet),
    )
    .add_systems(
        Update,
        (
            bevy::window::close_on_esc,
            draw_boundary,
            draw_line,
            update_scoreboard,
        ),
    )
    .configure_sets(Update, PlaySet.run_if(in_state(GameState::Match)))
    .configure_sets(FixedUpdate, PlaySet.run_if(in_state(GameState::Match)))
    .insert_resource(Score(0));
}

pub fn setup_play(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    // bg_music: Res<BackgroundMusic>,
) {
    let handle_playership_mesh = meshes.add(Triangle2d::new(
        Vec2::new(-15., -15.),
        Vec2::X * 22.,
        Vec2::new(-15., 15.),
    ));
    let handle_playership_colormaterial = materials.add(Color::LIME_GREEN);

    let (ship, children) = PlayerShip::new(
        0.,
        0.,
        None,
        handle_playership_mesh,
        handle_playership_colormaterial,
    );
    commands.spawn(ship).with_children(|parent| {
        parent.spawn(children);
    });

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
        200.,
        0.,
        MEDIUM_ASTEROID_R,
        handle_polygon.clone(),
        handle_asteroid_colormaterial.clone(),
        None,
        None,
        None,
    ));

    // commands.spawn(Asteroid::new(
    //     200.,
    //     -50.,
    //     MEDIUM_ASTEROID_R,
    //     handle_polygon.clone(),
    //     handle_asteroid_colormaterial.clone(),
    //     Some(Heading::from_angle(-PI / 2.)),
    //     Some(500.),
    //     None,
    // ));

    commands.spawn((
        ScoreboardUi,
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: LABEL_COLOR,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: SCOREBOARD_FONT_SIZE,
                color: SCORE_COLOR,
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: SCOREBOARD_TEXT_PADDING * 10.,
            right: SCOREBOARD_TEXT_PADDING,
            ..default()
        }),
    ));
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct PlaySet;

pub fn draw_line(mut painter: ShapePainter) {
    let line_color = Color::ORANGE;

    painter.thickness = 1.;
    painter.color = line_color;

    painter.line(Vec3::new(0., -100., 0.), Vec3::new(30., -100., 0.));
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

pub fn move_ship(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &TurnRate), With<Player>>,
    time: Res<Time>,
) {
    for (mut transform, turnrate) in query.iter_mut() {
        // let mut thrust = 0.;
        // if keyboard_input.pressed(KeyCode::KeyS) {
        //     thrust += 1.;
        // }
        // get fwd vector by applying current rot to ships init facing vec
        // let movement_direction = (transform.rotation * *DEFAULT_HEADING) * Vec3::X;
        // let movement_distance = thrust * movespeed.0 * time.delta_seconds();
        // let translation_delta = movement_direction * movement_distance;
        // transform.translation += translation_delta;

        let mut rotation_sign = 0.;
        if keyboard_input.pressed(KeyCode::KeyA) {
            rotation_sign += 1.;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            rotation_sign -= 1.;
        }
        transform.rotate_z(rotation_sign * turnrate.0 * time.delta_seconds());
    }
}

pub fn wraparound(mut query: Query<&mut Transform, With<Collider>>) {
    for mut transform in query.iter_mut() {
        if transform.translation.y >= TOP_WALL {
            transform.translation.y = BOTTOM_WALL + (transform.translation.y - TOP_WALL);
        }
        if transform.translation.y <= BOTTOM_WALL {
            transform.translation.y = TOP_WALL - (BOTTOM_WALL - transform.translation.y);
        }
        if transform.translation.x >= RIGHT_WALL {
            transform.translation.x = LEFT_WALL + (transform.translation.x - RIGHT_WALL);
        }
        if transform.translation.x <= LEFT_WALL {
            transform.translation.x = RIGHT_WALL - (LEFT_WALL - transform.translation.x);
        }
    }
}

pub fn ship_fire(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut q_ship: Query<&Children, With<Player>>,
    mut q_emitter: Query<(&GlobalTransform, &mut ProjectileEmission, &FireType)>,
) {
    // when fire key pressed
    if keyboard_input.pressed(KeyCode::Space) {
        // find ship, get children projectile emitters
        for children in &mut q_ship {
            for child in children {
                if let Ok((global_transform, mut emitter, firetype)) = q_emitter.get_mut(*child) {
                    // spawn primary fire projectile
                    match firetype.fire_type {
                        FireTypes::Primary => {
                            let last_emit = emitter.last_emission_time;

                            if last_emit.elapsed().as_millis() as i32 >= emitter.cooldown_ms {
                                emitter.last_emission_time = Instant::now();

                                let (_scale, rotation, translation) =
                                    global_transform.to_scale_rotation_translation();

                                commands.spawn(Projectile::new(
                                    translation.x,
                                    translation.y,
                                    Some(rotation.into()),
                                    Some(emitter.projectile_speed),
                                    None,
                                    Some(emitter.damage),
                                    Some(emitter.projectile_duration),
                                    None,
                                    None,
                                    ProjectileTag,
                                ));
                            }
                        }
                        _ => (),
                    };
                }
            }
        }
    }
}

// pub fn system_projectile_emission(
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     mut query: Query<(&mut Transform, &TurnRate, &MoveSpeed), With<Player>>,
//     time: Res<Time>,
// ) {

// }

fn update_scoreboard(scoreboard: Res<Score>, mut query: Query<&mut Text, With<ScoreboardUi>>) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.0.to_string();
}
