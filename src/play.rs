use std::time::Instant;

use bevy::prelude::*;
use bevy_particle_systems::{
    ColorOverTime, Curve, CurvePoint, EmitterShape, JitteredValue, ParticleSystem,
    ParticleSystemBundle, Playing,
};
use bevy_rapier2d::{
    dynamics::Velocity,
    geometry::{ActiveEvents, Collider, Sensor},
    parry::shape::SharedShape,
};
use bevy_vector_shapes::{painter::ShapePainter, shapes::LinePainter};
use noise::{
    core::perlin::perlin_2d, permutationtable::PermutationTable, utils::PlaneMapBuilder, NoiseFn,
    Perlin,
};

use crate::{
    archetypes::{AsteroidSizes, ProjectileBundle},
    audio::ProjectileEmitSound,
    avatars::{gen_asteroid, gen_playership, gen_playership_from_materialmesh},
    components::{
        DespawnDelay, FireType, PickupTag, PlayerShipTag, ProjectileEmission, ProjectileTag, Score,
        ScoreboardUi, TurnRate,
    },
    effects::{
        handle_collision_effects, handle_destruction_effects, CollisionEffectEvent,
        DestructionEffectEvent,
    },
    events::{update_collide_ship, CollisionAsteroidAsteroidEvent, CollisionProjectileEvent},
    game::{
        despawn_screen, AsteroidMaterialHandles, AsteroidMeshHandles, GameState, OnPlayScreen,
        ParticlePixelTexture, PlanetGreenTexture, PlanetGreyTexture, PlanetPurpleTexture,
        PlayerShipMaterialHandle, PlayerShipMeshHandle, PlayerShipTexture, PowerupComplexTexture,
        PowerupSimpleTexture, StarComplexTexture, StarEssentialTexture, StarSimpleTexture,
        WhiteMaterialHandle, BOTTOM_WALL, LABEL_COLOR, LEFT_WALL, RIGHT_WALL, SCOREBOARD_FONT_SIZE,
        SCOREBOARD_TEXT_PADDING, SCORE_COLOR, TOP_WALL,
    },
    physics::{apply_forces_ship, handle_collisions, handle_destructions},
    utils::Heading,
};

pub fn play_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Play), setup_play)
        .add_systems(
            FixedUpdate,
            (
                ship_turn,
                apply_forces_ship,
                wraparound,
                ship_fire,
                handle_collisions,
                handle_destructions,
                despawn_delay,
            )
                .chain()
                .run_if(in_state(GameState::Play)),
        )
        .add_systems(
            Update,
            (
                (
                    draw_boundary,
                    handle_collision_effects,
                    // handle_destruction_effects,
                    // update_scoreboard,
                ),
                // .run_if(in_state(GameState::Play)),
                (despawn_screen::<OnPlayScreen>, setup_play)
                    .chain()
                    .run_if(press_r_restart_play),
            ),
        )
        .add_systems(OnExit(GameState::Play), despawn_screen::<OnPlayScreen>)
        .add_event::<CollisionAsteroidAsteroidEvent>()
        .add_event::<CollisionProjectileEvent>()
        .add_event::<DestructionEffectEvent>()
        .add_event::<CollisionEffectEvent>();
}

pub fn setup_play(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
    asteroid_mesh_handles: Res<AsteroidMeshHandles>,
    asteroid_material_handles: Res<AsteroidMaterialHandles>,
    playership_mesh_handle: Res<PlayerShipMeshHandle>,
    playership_material_handle: Res<PlayerShipMaterialHandle>, // bg_music: Res<BackgroundMusic>,
    playership_texture: Res<PlayerShipTexture>,
    white_material_handle: Res<WhiteMaterialHandle>,
    particle_pixel_texture: Res<ParticlePixelTexture>,
    powerup_essential_texture: Res<PowerupSimpleTexture>,
    powerup_simple_texture: Res<PowerupSimpleTexture>,
    powerup_complex_texture: Res<PowerupComplexTexture>,
    star_essential_texture: Res<StarEssentialTexture>,
    star_simple_texture: Res<StarSimpleTexture>,
    star_complex_texture: Res<StarComplexTexture>,
    green_planet_texture: Res<PlanetGreenTexture>,
    // grey_planet_texture: Res<PlanetGreyTexture>,
    // purple_planet_texture: Res<PlanetPurpleTexture>,
) {
    spawn_playership(
        0.,
        -150.,
        &mut commands,
        &playership_texture,
        None,
        &particle_pixel_texture,
    );

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
    // let start_x = LEFT_WALL + 50.;
    // let dx = 250.;
    // let y = TOP_WALL - 50.;
    // let separation_y = 150.;
    // let pairs = [
    //     (AsteroidSizes::Small, AsteroidSizes::Small),
    //     (AsteroidSizes::Small, AsteroidSizes::Medium),
    //     (AsteroidSizes::Small, AsteroidSizes::Large),
    //     (AsteroidSizes::Medium, AsteroidSizes::Medium),
    //     (AsteroidSizes::Medium, AsteroidSizes::Large),
    //     (AsteroidSizes::Large, AsteroidSizes::Large),
    // ];
    // for (i, (size_a, size_b)) in pairs.iter().enumerate() {
    //     commands
    //         .spawn(gen_asteroid(
    //             *size_a,
    //             5,
    //             asteroid_mesh_handles.0.clone(),
    //             asteroid_material_handles.0.clone(),
    //             start_x + (dx * i as f32),
    //             y,
    //             Velocity {
    //                 linvel: Heading(-90.).linvel(20.),
    //                 angvel: 0.,
    //             },
    //         ))
    //         .insert(OnPlayScreen);
    //     commands
    //         .spawn(gen_asteroid(
    //             *size_b,
    //             5,
    //             asteroid_mesh_handles.0.clone(),
    //             asteroid_material_handles.0.clone(),
    //             start_x + (dx * i as f32),
    //             y - separation_y,
    //             Velocity {
    //                 linvel: Heading(90.).linvel(20.),
    //                 angvel: 0.,
    //             },
    //         ))
    //         .insert(OnPlayScreen);
    // }

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
    //             max_particles: 50_000,
    //             texture: asset_server.load("px.png").into(),
    //             spawn_rate_per_second: 10.0.into(),
    //             initial_speed: JitteredValue::jittered(70.0, -3.0..3.0),
    //             lifetime: JitteredValue::jittered(3.0, -2.0..2.0),
    //             color: ColorOverTime::Gradient(Curve::new(vec![
    //                 CurvePoint::new(Color::PURPLE, 0.0),
    //                 CurvePoint::new(Color::RED, 0.5),
    //                 CurvePoint::new(Color::rgba(0.0, 0.0, 1.0, 0.0), 1.0),
    //             ])),
    //             emitter_shape: EmitterShape::line(200.0, std::f32::consts::FRAC_PI_4),
    //             looping: true,
    //             rotate_to_movement_direction: true,
    //             initial_rotation: (-90.0_f32).to_radians().into(),
    //             system_duration_seconds: 10.0,
    //             max_distance: Some(300.0),
    //             scale: 1.0.into(),
    //             ..ParticleSystem::default()
    //         },
    //         transform: Transform::from_xyz(LEFT_WALL + 100., BOTTOM_WALL + 100., 0.0),
    //         ..ParticleSystemBundle::default()
    //     })
    //     .insert(Playing)
    //     .insert(OnPlayScreen);

    // let noise = Perlin::new(0);
    // let width = (RIGHT_WALL - LEFT_WALL);
    // let height = (TOP_WALL - BOTTOM_WALL);
    // for y in (0..height as i32).step_by(20) {
    //     for x in (0..width as i32).step_by(20) {
    //         let bright = noise.get([
    //             x as f64 / (0.1 * width as f64),  // / (width as f64 * 10000.),
    //             y as f64 / (0.1 * height as f64), // / (height as f64 * 10000.),
    //         ]);
    //         let bright = ((bright + 1.0) / 2.0) as f32;

    //         let dx = (noise.get([
    //             x as f64 / (0.2 * width as f64),
    //             y as f64 / (0.2 * width as f64),
    //             0.0,
    //         ]) * 50.) as f32;
    //         let dy = (noise.get([
    //             y as f64 / (0.2 * height as f64),
    //             x as f64 / (0.2 * width as f64),
    //             1.0,
    //         ]) * 50.) as f32;
    //         let dz = ((noise.get([
    //             y as f64 / (0.2 * height as f64),
    //             x as f64 / (0.2 * width as f64),
    //             2.0,
    //         ]) as f32)
    //             + 1.0 / 2.0)
    //             * 3.0;

    //         commands.spawn(SpriteBundle {
    //             sprite: Sprite {
    //                 // color: Color::rgba(1. - bright, 0., bright as f32, bright as f32),
    //                 color: Color::rgba(1., 1., 1., bright),
    //                 ..default()
    //             },
    //             transform: Transform::from_xyz(
    //                 // x as f32 - width / 2.,
    //                 // y as f32 - height / 2.,
    //                 x as f32 - (width / 2.0) + dx as f32,
    //                 y as f32 - (height / 2.0) + dy as f32,
    //                 0.0,
    //             )
    //             // .with_scale(Vec3::splat(dz)),
    //             .with_scale(Vec3::splat(1.0)),
    //             ..default()
    //         });
    //     }
    // }

    // Simple powerup, large and easy to get
    spawn_essential_powerup(-200., 0., &mut commands, &powerup_essential_texture);
    spawn_simple_powerup(-250., 0., &mut commands, &powerup_simple_texture);
    spawn_complex_powerup(-300., 0., &mut commands, &powerup_complex_texture);

    spawn_essential_star(&mut commands, &star_essential_texture);
    spawn_simple_star(&mut commands, &star_simple_texture);
    spawn_complex_star(&mut commands, &star_complex_texture);

    // spawn_complex_star(&mut commands, &star_complex_texture);
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            // GOLD
            // color: Color::rgba(1.0, 0.84, 0.0, 0.5),
            // color: Color::rgba(1., 1., 1., 0.7),
            ..default()
        },
        texture: green_planet_texture.0.clone(),
        transform: Transform::from_xyz(200., -100., 0.).with_scale(Vec3::splat(0.10)),
        ..default()
    },));
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
                                        Some(2.0),
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

fn spawn_essential_star(commands: &mut Commands, star_simple_texture: &StarEssentialTexture) {
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,
            ..default()
        },
        texture: star_simple_texture.0.clone(),
        transform: Transform::from_xyz(-200., -50., 0.).with_scale(Vec3::splat(0.05)),
        ..default()
    },));
}

fn spawn_simple_star(commands: &mut Commands, star_basic_texture: &StarSimpleTexture) {
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::rgba(1., 1., 1., 0.8),
            ..default()
        },
        texture: star_basic_texture.0.clone(),
        transform: Transform::from_xyz(-200., -100., 0.).with_scale(Vec3::splat(0.10)),
        ..default()
    },));
}

fn spawn_complex_star(commands: &mut Commands, star_complex_texture: &StarComplexTexture) {
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            // GOLD
            // color: Color::rgba(1.0, 0.84, 0.0, 0.5),
            color: Color::rgba(1., 1., 1., 0.7),
            ..default()
        },
        texture: star_complex_texture.0.clone(),
        transform: Transform::from_xyz(-200., -150., 0.).with_scale(Vec3::splat(0.10)),
        ..default()
    },));
}

fn spawn_essential_powerup(
    x: f32,
    y: f32,
    commands: &mut Commands,
    powerup_essential_texture: &PowerupSimpleTexture,
) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                ..default()
            },
            texture: powerup_essential_texture.0.clone(),
            transform: Transform::from_xyz(x, y, 0.).with_scale(Vec3::splat(0.8)),
            ..default()
        },
        Collider::from(SharedShape::ball(20.)),
        Sensor,
        PickupTag,
        ActiveEvents::COLLISION_EVENTS,
    ));
}

fn spawn_simple_powerup(
    x: f32,
    y: f32,
    commands: &mut Commands,
    powerup_simple_texture: &PowerupSimpleTexture,
) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                ..default()
            },
            texture: powerup_simple_texture.0.clone(),
            transform: Transform::from_xyz(x, y, 0.).with_scale(Vec3::splat(0.8)),
            ..default()
        },
        Collider::from(SharedShape::ball(20.)),
        Sensor,
        PickupTag,
        ActiveEvents::COLLISION_EVENTS,
    ));
}

fn spawn_complex_powerup(
    x: f32,
    y: f32,
    commands: &mut Commands,
    powerup_complex_texture: &PowerupComplexTexture,
) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                ..default()
            },
            texture: powerup_complex_texture.0.clone(),
            transform: Transform::from_xyz(x, y, 0.).with_scale(Vec3::splat(0.8)),
            ..default()
        },
        Collider::from(SharedShape::ball(20.)),
        Sensor,
        PickupTag,
        ActiveEvents::COLLISION_EVENTS,
    ));
}

fn spawn_playership(
    x: f32,
    y: f32,
    commands: &mut Commands,
    playership_texture: &PlayerShipTexture,
    heading: Option<Heading>,
    particle_pixel_texture: &ParticlePixelTexture,
) {
    let (ship, children) = gen_playership(playership_texture, x, y, None, particle_pixel_texture);
    commands
        .spawn(ship)
        .with_children(|parent| {
            parent.spawn(children.0);
            parent.spawn(children.1);
        })
        .insert(OnPlayScreen);
}

fn print_ship_heading(mut query: Query<&Transform, With<PlayerShipTag>>) {
    for transform in query.iter() {
        let heading = Heading::from(transform.rotation);
    }
}

// pub fn draw_line(mut painter: ShapePainter) {
//     let line_color = Color::ORANGE;

//     painter.thickness = 1.;
//     painter.color = line_color;

//     painter.line(Vec3::new(0., -100., 0.), Vec3::new(30., -100., 0.));
// }
