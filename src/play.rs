use std::time::Instant;

use bevy::prelude::*;
use bevy_particle_systems::{
    ColorOverTime, Curve, CurvePoint, EmitterShape, JitteredValue, ParticleSystem,
    ParticleSystemBundle, Playing,
};
use bevy_rapier2d::{dynamics::Velocity, geometry::Collider};
use bevy_vector_shapes::{painter::ShapePainter, shapes::LinePainter};

use crate::{
    archetypes::{AsteroidSizes, ProjectileBundle},
    audio::ProjectileEmitSound,
    avatars::{gen_asteroid, gen_playership},
    components::{
        DespawnDelay, FireType, PlayerShipTag, ProjectileEmission, ProjectileTag, Score,
        ScoreboardUi, TurnRate,
    },
    game::{
        despawn_screen, AsteroidMaterialHandles, AsteroidMeshHandles, GameState, OnPlayScreen,
        PlayerShipMaterialHandle, PlayerShipMeshHandle, ThrustParticleTexture, BOTTOM_WALL,
        LABEL_COLOR, LEFT_WALL, RIGHT_WALL, SCOREBOARD_FONT_SIZE, SCOREBOARD_TEXT_PADDING,
        SCORE_COLOR, TOP_WALL,
    },
    physics::{
        apply_forces_ship, emit_collision_particles, emit_thruster_particles, resolve_collisions,
    },
    utils::Heading,
};

pub fn play_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Play), setup_play)
        .add_systems(
            FixedUpdate,
            (
                ship_turn,
                wraparound,
                ship_fire,
                despawn_delay,
                apply_forces_ship,
                resolve_collisions, // check_for_collisions,
                                    // play_collision_sound,
                                    // process_score,
            )
                .chain()
                .run_if(in_state(GameState::Play)),
        )
        .add_systems(
            Update,
            (
                (
                    draw_boundary,
                    draw_line,
                    update_scoreboard,
                    emit_thruster_particles,
                    emit_collision_particles,
                )
                    .run_if(in_state(GameState::Play)),
                (despawn_screen::<OnPlayScreen>, setup_play)
                    .chain()
                    .run_if(press_r_restart_play),
            ),
        )
        .add_systems(OnExit(GameState::Play), despawn_screen::<OnPlayScreen>);
}

pub fn setup_play(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
    asteroid_mesh_handles: Res<AsteroidMeshHandles>,
    asteroid_material_handles: Res<AsteroidMaterialHandles>,
    playership_mesh_handle: Res<PlayerShipMeshHandle>,
    playership_material_handle: Res<PlayerShipMaterialHandle>, // bg_music: Res<BackgroundMusic>,
    thrust_particle_texture: Res<ThrustParticleTexture>,
    asset_server: Res<AssetServer>,
) {
    let (ship, children) = gen_playership(
        playership_mesh_handle.0.clone(),
        playership_material_handle.0.clone(),
        0.,
        0.,
        None,
        thrust_particle_texture.0.clone().into(),
    );
    commands
        .spawn(ship)
        .with_children(|parent| {
            parent.spawn(children.0);
            parent.spawn(children.1);
        })
        .insert(OnPlayScreen);

    let ast1 = gen_asteroid(
        AsteroidSizes::Medium,
        5,
        asteroid_mesh_handles.0.clone(),
        asteroid_material_handles.0.clone(),
        200.,
        0.,
        Velocity {
            linvel: Heading(0.).linvel(0.),
            angvel: 0.5,
        },
    );
    commands.spawn(ast1).insert(OnPlayScreen);

    // clashing asteroids
    let start_x = LEFT_WALL + 50.;
    let dx = 250.;
    let y = TOP_WALL - 50.;
    let separation_y = 150.;
    let pairs = [
        (AsteroidSizes::Small, AsteroidSizes::Small),
        (AsteroidSizes::Small, AsteroidSizes::Medium),
        (AsteroidSizes::Small, AsteroidSizes::Large),
        (AsteroidSizes::Medium, AsteroidSizes::Medium),
        (AsteroidSizes::Medium, AsteroidSizes::Large),
        (AsteroidSizes::Large, AsteroidSizes::Large),
    ];
    for (i, (size_a, size_b)) in pairs.iter().enumerate() {
        commands
            .spawn(gen_asteroid(
                *size_a,
                5,
                asteroid_mesh_handles.0.clone(),
                asteroid_material_handles.0.clone(),
                start_x + (dx * i as f32),
                y,
                Velocity {
                    linvel: Heading(-90.).linvel(20.),
                    angvel: 0.,
                },
            ))
            .insert(OnPlayScreen);
        commands
            .spawn(gen_asteroid(
                *size_b,
                5,
                asteroid_mesh_handles.0.clone(),
                asteroid_material_handles.0.clone(),
                start_x + (dx * i as f32),
                y - separation_y,
                Velocity {
                    linvel: Heading(90.).linvel(20.),
                    angvel: 0.,
                },
            ))
            .insert(OnPlayScreen);
    }

    // Diagonal collision, see collision particles
    commands
        .spawn(gen_asteroid(
            AsteroidSizes::Medium,
            5,
            asteroid_mesh_handles.0.clone(),
            asteroid_material_handles.0.clone(),
            LEFT_WALL + 50.,
            BOTTOM_WALL + 300.,
            Velocity {
                linvel: Heading(-45.).linvel(20.),
                angvel: 0.,
            },
        ))
        .insert(OnPlayScreen);
    commands
        .spawn(gen_asteroid(
            AsteroidSizes::Medium,
            5,
            asteroid_mesh_handles.0.clone(),
            asteroid_material_handles.0.clone(),
            LEFT_WALL + 130.,
            BOTTOM_WALL + 230.,
            Velocity {
                linvel: Heading(135.).linvel(20.),
                angvel: 0.,
            },
        ))
        .insert(OnPlayScreen);
    // spawn for test
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

    // let handle_mesh_asteroid_med_5 = meshes.add(RegularPolygon::new(MEDIUM_ASTEROID_R, 5));
    commands
        .spawn((
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
        ))
        .insert(OnPlayScreen);

    // commands
    //     .spawn(ParticleSystemBundle {
    //         particle_system: ParticleSystem {
    //             max_particles: 10_000,
    //             texture: ParticleTexture::Sprite(asset_server.load("my_particle.png")),
    //             spawn_rate_per_second: 25.0.into(),
    //             initial_speed: JitteredValue::jittered(3.0, -1.0..1.0),
    //             lifetime: JitteredValue::jittered(8.0, -2.0..2.0),
    //             color: ColorOverTime::Gradient(Gradient::new(vec![
    //                 ColorPoint::new(Color::WHITE, 0.0),
    //                 ColorPoint::new(Color::rgba(0.0, 0.0, 1.0, 0.0), 1.0),
    //             ])),
    //             looping: true,
    //             system_duration_seconds: 10.0,
    //             ..ParticleSystem::default()
    //         },
    //         ..ParticleSystemBundle::default()
    //     })
    //     .insert(Playing);

    // CIRCULAR OUTWARD
    // commands
    //     .spawn(ParticleSystemBundle {
    //         particle_system: ParticleSystem {
    //             max_particles: 1_000,
    //             texture: asset_server.load("px.png").into(),
    //             spawn_rate_per_second: 10.0.into(),
    //             initial_speed: JitteredValue::jittered(200.0, -50.0..50.0),
    //             velocity_modifiers: vec![Drag(0.01.into())],
    //             lifetime: JitteredValue::jittered(8.0, -2.0..2.0),
    //             color: ColorOverTime::Gradient(Curve::new(vec![
    //                 CurvePoint::new(Color::PURPLE, 0.0),
    //                 CurvePoint::new(Color::RED, 0.5),
    //                 CurvePoint::new(Color::rgba(0.0, 0.0, 1.0, 0.0), 1.0),
    //             ])),
    //             looping: true,
    //             system_duration_seconds: 10.0,
    //             max_distance: Some(100.0),
    //             scale: 1.0.into(),
    //             bursts: vec![
    //                 ParticleBurst::new(0.0, 100),
    //                 ParticleBurst::new(2.0, 100),
    //                 ParticleBurst::new(4.0, 100),
    //                 ParticleBurst::new(6.0, 100),
    //                 ParticleBurst::new(8.0, 100),
    //             ],
    //             ..ParticleSystem::default()
    //         },
    //         transform: Transform::from_xyz(LEFT_WALL + 25., BOTTOM_WALL + 25., 0.0),
    //         ..ParticleSystemBundle::default()
    //     })
    //     .insert(Playing);

    // STRAIGHT EMIT,eg: tracers, laser residue
    commands
        .spawn(ParticleSystemBundle {
            particle_system: ParticleSystem {
                max_particles: 50_000,
                texture: asset_server.load("px.png").into(),
                spawn_rate_per_second: 10.0.into(),
                initial_speed: JitteredValue::jittered(70.0, -3.0..3.0),
                lifetime: JitteredValue::jittered(3.0, -2.0..2.0),
                color: ColorOverTime::Gradient(Curve::new(vec![
                    CurvePoint::new(Color::PURPLE, 0.0),
                    CurvePoint::new(Color::RED, 0.5),
                    CurvePoint::new(Color::rgba(0.0, 0.0, 1.0, 0.0), 1.0),
                ])),
                emitter_shape: EmitterShape::line(200.0, std::f32::consts::FRAC_PI_4),
                looping: true,
                rotate_to_movement_direction: true,
                initial_rotation: (-90.0_f32).to_radians().into(),
                system_duration_seconds: 10.0,
                max_distance: Some(300.0),
                scale: 1.0.into(),
                ..ParticleSystem::default()
            },
            transform: Transform::from_xyz(LEFT_WALL + 100., BOTTOM_WALL + 100., 0.0),
            ..ParticleSystemBundle::default()
        })
        .insert(Playing)
        .insert(OnPlayScreen);

    // CONE, eg: ship exhaust, weapon muzzle flash
    // commands
    //     .spawn(ParticleSystemBundle {
    //         particle_system: ParticleSystem {
    //             max_particles: 1000,
    //             texture: thrust_particle_texture.0.clone().into(),
    //             spawn_rate_per_second: 50.0.into(),
    //             initial_speed: JitteredValue::jittered(200.0, -25.0..25.0),
    //             lifetime: JitteredValue::jittered(2.0, -1.0..1.0),
    //             color: ColorOverTime::Gradient(Curve::new(vec![
    //                 CurvePoint::new(Color::PURPLE, 0.0),
    //                 CurvePoint::new(Color::RED, 0.5),
    //                 CurvePoint::new(Color::rgba(0.0, 0.0, 1.0, 0.0), 1.0),
    //             ])),
    //             emitter_shape: CircleSegment {
    //                 radius: 30.0.into(),
    //                 opening_angle: std::f32::consts::PI / 12.,
    //                 direction_angle: PI,
    //             }
    //             .into(),
    //             looping: true,
    //             rotate_to_movement_direction: true,
    //             initial_rotation: (0.0_f32).to_radians().into(),
    //             system_duration_seconds: 10.0,
    //             max_distance: Some(200.0),
    //             scale: 1.0.into(),
    //             ..ParticleSystem::default()
    //         },
    //         transform: Transform::from_xyz(30., 0., 0.0),
    //         ..ParticleSystemBundle::default()
    //     })
    //     .insert(Playing);
}

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

pub fn ship_turn(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &TurnRate), With<PlayerShipTag>>,
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

pub fn ship_fire(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut q_ship: Query<&Children, With<PlayerShipTag>>,
    mut q_emitter: Query<(&GlobalTransform, &mut ProjectileEmission, &FireType)>,
    fire_sound: Res<ProjectileEmitSound>,
) {
    // when fire key pressed
    if keyboard_input.pressed(KeyCode::Space) {
        // find ship, get children projectile emitters
        for children in &mut q_ship {
            for child in children {
                if let Ok((global_transform, mut emitter, firetype)) = q_emitter.get_mut(*child) {
                    // spawn primary fire projectile
                    match firetype {
                        FireType::Primary => {
                            let last_emit = emitter.last_emission_time;

                            if last_emit.elapsed().as_millis() as i32 >= emitter.cooldown_ms {
                                emitter.last_emission_time = Instant::now();

                                let (_scale, rotation, translation) =
                                    global_transform.to_scale_rotation_translation();

                                commands
                                    .spawn(ProjectileBundle::new(
                                        translation.x,
                                        translation.y,
                                        Some(rotation.into()),
                                        Some(emitter.projectile_speed),
                                        None,
                                        Some(emitter.damage),
                                        None,
                                        None,
                                    ))
                                    .insert(OnPlayScreen);
                                commands.spawn(AudioBundle {
                                    source: fire_sound.0.clone(),
                                    ..default()
                                });
                            }
                        }
                        _ => (),
                    };
                }
            }
        }
    }
}

pub fn update_scoreboard(scoreboard: Res<Score>, mut query: Query<&mut Text, With<ScoreboardUi>>) {
    if let Ok(mut text) = query.get_single_mut() {
        text.sections[1].value = scoreboard.0.to_string();
    }
}

pub fn despawn_delay(
    mut commands: Commands,
    mut query: Query<(Entity, &mut DespawnDelay), With<ProjectileTag>>,
    time: Res<Time>,
) {
    for (entity, mut despawn_delay) in &mut query {
        if despawn_delay.0.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn press_r_restart_play(keyboard_input: Res<ButtonInput<KeyCode>>) -> bool {
    keyboard_input.just_pressed(KeyCode::KeyR)
}
