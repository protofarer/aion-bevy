use bevy::prelude::*;
use bevy_particle_systems::{
    ColorOverTime, Curve, CurvePoint, EmitterShape, JitteredValue, ParticleSystem,
    ParticleSystemBundle, Playing,
};
use bevy_rapier2d::geometry::Collider;
use bevy_vector_shapes::{painter::ShapePainter, shapes::LinePainter};
use noise::{NoiseFn, Perlin};

use crate::{
    archetypes::AsteroidSizes,
    avatars::{Asteroid, PlayerShip},
    components::{DespawnDelay, ProjectileTag, Score, ScoreboardUi},
    controls::{ship_fire, ship_turn, thrust_ship},
    effects::{
        handle_collision_effects, handle_destruction_effects, handle_thrust_effects,
        CollisionEffectEvent, DestructionEffectEvent, ThrustEffectEvent,
    },
    events::{CollisionAsteroidAsteroidEvent, CollisionProjectileEvent},
    game::{
        despawn_screen, AsteroidMaterialHandles, AsteroidMeshHandles, GameState, OnPlayScreen,
        ParticlePixelTexture, PlayerShipTexture, StarComplexTexture, StarCoreTexture,
        StarSimpleTexture, BOTTOM_WALL, LABEL_COLOR, LEFT_WALL, RIGHT_WALL, SCOREBOARD_FONT_SIZE,
        SCOREBOARD_TEXT_PADDING, SCORE_COLOR, TOP_WALL,
    },
    physics::handle_collisions,
    utils::Heading,
};

pub fn play_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Play), setup_play)
        .add_systems(
            FixedUpdate,
            (
                ship_turn,
                thrust_ship,
                wraparound,
                ship_fire,
                handle_collisions,
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
                    handle_destruction_effects,
                    handle_thrust_effects,
                    update_scoreboard,
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
        .add_event::<CollisionEffectEvent>()
        .add_event::<ThrustEffectEvent>();
}

// mut meshes: ResMut<Assets<Mesh>>,
// mut materials: ResMut<Assets<ColorMaterial>>,
// playership_mesh_handle: Res<PlayerShipMeshHandle>,
// playership_material_handle: Res<PlayerShipMaterialHandle>, // bg_music: Res<BackgroundMusic>,
pub fn setup_play(
    mut cmd: Commands,
    asteroid_mesh_handles: Res<AsteroidMeshHandles>,
    asteroid_material_handles: Res<AsteroidMaterialHandles>,
    playership_texture: Res<PlayerShipTexture>,
    // white_material_handle: Res<WhiteMaterialHandle>,
    particle_pixel_texture: Res<ParticlePixelTexture>,
    // powerup_core_texture: Res<PowerupCoreTexture>,
    // powerup_simple_texture: Res<PowerupSimpleTexture>,
    // powerup_complex_texture: Res<PowerupComplexTexture>,
    star_core_texture: Res<StarCoreTexture>,
    star_simple_texture: Res<StarSimpleTexture>,
    star_complex_texture: Res<StarComplexTexture>,
    // green_planet_texture: Res<PlanetGreenTexture>,
    // grey_planet_texture: Res<PlanetGreyTexture>,
    // purple_planet_texture: Res<PlanetPurpleTexture>,
) {
    PlayerShip::spawn(
        0.,
        -150.,
        None,
        &playership_texture,
        &particle_pixel_texture,
        &mut cmd,
    );

    // highly accessibly asteroid
    Asteroid::spawn(
        AsteroidSizes::Medium,
        5,
        0.,
        100.,
        None,
        Some(0.),
        asteroid_mesh_handles.0.clone(),
        asteroid_material_handles.0.clone(),
        &mut cmd,
    );

    dev_row_of_clashing_asteroids(
        &mut cmd,
        &asteroid_mesh_handles,
        &asteroid_material_handles,
    );

    // Diagonal collision, see collision particles
    Asteroid::spawn(
        AsteroidSizes::Medium,
        5,
        LEFT_WALL + 50.,
        BOTTOM_WALL + 300.,
        None,
        Some(0.),
        asteroid_mesh_handles.0.clone(),
        asteroid_material_handles.0.clone(),
        &mut cmd,
    );
    Asteroid::spawn(
        AsteroidSizes::Medium,
        5,
        LEFT_WALL + 130.,
        BOTTOM_WALL + 230.,
        None,
        Some(0.),
        asteroid_mesh_handles.0.clone(),
        asteroid_material_handles.0.clone(),
        &mut cmd,
    );

    cmd
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

    spawn_cosmic_background(
        &mut cmd,
        &star_core_texture,
        &star_simple_texture,
        &star_complex_texture,
    );
    spawn_cosmic_wind(300., -400., None, &mut cmd, &particle_pixel_texture);

    // Simple powerup, large and easy to get
    // spawn_core_powerup(-200., 0., &mut commands, &powerup_core_texture);
    // spawn_simple_powerup(-250., 0., &mut commands, &powerup_simple_texture);
    // spawn_complex_powerup(-300., 0., &mut commands, &powerup_complex_texture);
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

pub fn update_scoreboard(scoreboard: Res<Score>, mut query: Query<&mut Text, With<ScoreboardUi>>) {
    if let Ok(mut text) = query.get_single_mut() {
        text.sections[1].value = scoreboard.to_string();
    }
}

pub fn despawn_delay(
    mut cmd: Commands,
    mut query: Query<(Entity, &mut DespawnDelay), With<ProjectileTag>>,
    time: Res<Time>,
) {
    for (entity, mut despawn_delay) in &mut query {
        if despawn_delay.tick(time.delta()).just_finished() {
            cmd.entity(entity).despawn();
        }
    }
}

pub fn press_r_restart_play(keyboard_input: Res<ButtonInput<KeyCode>>) -> bool {
    keyboard_input.just_pressed(KeyCode::KeyR)
}

fn spawn_core_star(
    x: f32,
    y: f32,
    energy: Option<f32>,
    color: Option<Color>,
    cmd: &mut Commands,
    star_simple_texture: &StarCoreTexture,
) {
    let energy = energy.unwrap_or(1.0);
    let color = match color {
        Some(x) => Color::hsl(x.h(), 0.7 + energy * 0.3, 0.2 + 0.7 * energy),
        None => Color::hsl(0.0, 0.0, 0.2 + 0.6 * energy),
    };
    cmd.spawn((SpriteBundle {
        sprite: Sprite { color, ..default() },
        texture: star_simple_texture.0.clone(),
        transform: Transform::from_xyz(x, y, 0.).with_scale(Vec3::splat(0.05 * energy)),
        ..default()
    },));
}

fn spawn_simple_star(
    x: f32,
    y: f32,
    energy: Option<f32>,
    color: Option<Color>,
    cmd: &mut Commands,
    star_basic_texture: &StarSimpleTexture,
) {
    let energy = energy.unwrap_or(1.0);
    let color = match color {
        Some(x) => Color::hsl(x.h(), 0.7 + energy * 0.3, 0.2 + 0.7 * energy),
        None => Color::WHITE,
    };
    cmd.spawn((SpriteBundle {
        sprite: Sprite { color, ..default() },
        texture: star_basic_texture.0.clone(),
        transform: Transform::from_xyz(x, y, 0.).with_scale(Vec3::splat(0.05 + (energy * 0.05))),
        ..default()
    },));
}

fn spawn_complex_star(
    x: f32,
    y: f32,
    energy: Option<f32>,
    color: Option<Color>,
    cmd: &mut Commands,
    star_complex_texture: &StarComplexTexture,
) {
    let energy = energy.unwrap_or(1.0);
    let color = match color {
        Some(x) => Color::hsl(x.h(), 0.4 + energy * 0.3, 0.2 + 0.7 * energy),
        None => Color::WHITE,
    };
    cmd.spawn((SpriteBundle {
        sprite: Sprite { color, ..default() },
        texture: star_complex_texture.0.clone(),
        transform: Transform::from_xyz(x, y, 0.).with_scale(Vec3::splat(0.04 + (energy * 0.09))),
        ..default()
    },));
}

// fn spawn_core_powerup(
//     x: f32,
//     y: f32,
//     commands: &mut Commands,
//     powerup_core_texture: &PowerupCoreTexture,
// ) {
//     commands.spawn((
//         SpriteBundle {
//             sprite: Sprite {
//                 color: Color::WHITE,
//                 ..default()
//             },
//             texture: powerup_core_texture.0.clone(),
//             transform: Transform::from_xyz(x, y, 0.).with_scale(Vec3::splat(0.8)),
//             ..default()
//         },
//         Collider::from(SharedShape::ball(20.)),
//         Sensor,
//         PickupTag,
//         ActiveEvents::COLLISION_EVENTS,
//     ));
// }

// fn spawn_simple_powerup(
//     x: f32,
//     y: f32,
//     commands: &mut Commands,
//     powerup_simple_texture: &PowerupSimpleTexture,
// ) {
//     commands.spawn((
//         SpriteBundle {
//             sprite: Sprite {
//                 color: Color::WHITE,
//                 ..default()
//             },
//             texture: powerup_simple_texture.0.clone(),
//             transform: Transform::from_xyz(x, y, 0.).with_scale(Vec3::splat(0.8)),
//             ..default()
//         },
//         Collider::from(SharedShape::ball(20.)),
//         Sensor,
//         PickupTag,
//         ActiveEvents::COLLISION_EVENTS,
//     ));
// }

// fn spawn_complex_powerup(
//     x: f32,
//     y: f32,
//     commands: &mut Commands,
//     powerup_complex_texture: &PowerupComplexTexture,
// ) {
//     commands.spawn((
//         SpriteBundle {
//             sprite: Sprite {
//                 color: Color::WHITE,
//                 ..default()
//             },
//             texture: powerup_complex_texture.0.clone(),
//             transform: Transform::from_xyz(x, y, 0.).with_scale(Vec3::splat(0.8)),
//             ..default()
//         },
//         Collider::from(SharedShape::ball(20.)),
//         Sensor,
//         PickupTag,
//         ActiveEvents::COLLISION_EVENTS,
//     ));
// }

fn spawn_cosmic_wind(
    x: f32,
    y: f32,
    heading: Option<Heading>,
    commands: &mut Commands,
    particle_pixel_texture: &ParticlePixelTexture,
) {
    commands
        .spawn(ParticleSystemBundle {
            particle_system: ParticleSystem {
                max_particles: 1000,
                texture: particle_pixel_texture.0.clone().into(),
                spawn_rate_per_second: 100.0.into(),
                initial_speed: JitteredValue::jittered(200.0, -50.0..0.0),
                lifetime: JitteredValue::jittered(4.0, -3.0..0.0),
                // color: ColorOverTime::Gradient(Curve::new(vec![
                //     CurvePoint::new(Color::PURPLE, 0.0),
                //     CurvePoint::new(Color::RED, 0.5),
                //     CurvePoint::new(Color::rgba(0.0, 0.0, 1.0, 0.0), 1.0),
                // ])),
                color: ColorOverTime::Gradient(Curve::new(vec![
                    CurvePoint::new(Color::rgba(1., 1., 1., 0.0), 0.0),
                    CurvePoint::new(Color::rgba(1., 1., 1., 1.), 0.5),
                    CurvePoint::new(Color::rgba(1., 1., 1., 0.0), 1.0),
                ])),
                // color: ColorOverTime::Constant(Color::WHITE),
                emitter_shape: EmitterShape::line(300.0, heading.unwrap_or_default().to_radians()),
                looping: true,
                despawn_on_finish: true, // true for one-shot, false for reuse like weapons fire...?
                // rotate_to_movement_direction: true,
                // initial_rotation: (0.0_f32).to_radians().into(),
                system_duration_seconds: 5.0,
                max_distance: Some(1000.0),
                scale: 2.0.into(),
                ..ParticleSystem::default()
            },
            transform: Transform::from_xyz(x, y, 0.0),
            ..ParticleSystemBundle::default()
        })
        .insert(Playing)
        .insert(OnPlayScreen);
}

fn dev_row_of_clashing_asteroids(
    cmd: &mut Commands,
    asteroid_mesh_handles: &AsteroidMeshHandles,
    asteroid_material_handles: &AsteroidMaterialHandles,
) {
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
        Asteroid::spawn(
            *size_a,
            5,
            start_x + (dx * i as f32),
            y,
            Some(Heading(-90.)),
            Some(20.),
            asteroid_mesh_handles.0.clone(),
            asteroid_material_handles.0.clone(),
            cmd,
        );
        Asteroid::spawn(
            *size_b,
            5,
            start_x + (dx * i as f32),
            y - separation_y,
            Some(Heading(90.)),
            Some(20.),
            asteroid_mesh_handles.0.clone(),
            asteroid_material_handles.0.clone(),
            cmd,
        );
    }
}

fn spawn_cosmic_background(
    mut cmd: &mut Commands,
    star_core_texture: &StarCoreTexture,
    star_simple_texture: &StarSimpleTexture,
    star_complex_texture: &StarComplexTexture,
) {
    let noise = Perlin::new(0);
    let width = RIGHT_WALL - LEFT_WALL;
    let height = TOP_WALL - BOTTOM_WALL;
    let step = 50; // simple density
    let density_core = 0.2;
    let density_simple = 0.06;
    let density_complex = 0.01;
    for y in (0..height as i32).step_by(step) {
        for x in (0..width as i32).step_by(step) {
            let energy = noise.get([
                x as f64 / (0.1 * width as f64),  // / (width as f64 * 10000.),
                y as f64 / (0.1 * height as f64), // / (height as f64 * 10000.),
            ]);
            let energy = ((energy + 1.0) / 2.0) as f32;

            let dx = (noise.get([
                x as f64 / (0.2 * width as f64),
                y as f64 / (0.2 * width as f64),
                0.0,
            ]) * 100.) as f32;
            let dy = (noise.get([
                y as f64 / (0.2 * height as f64),
                x as f64 / (0.2 * width as f64),
                1.0,
            ]) * 100.) as f32;
            // let dz = ((noise.get([
            //     y as f64 / (0.2 * height as f64),
            //     x as f64 / (0.2 * width as f64),
            //     2.0,
            // ]) as f32)
            //     + 1.0 / 2.0);

            // stars only show when a noised value is above an arbitrary threshold
            if rand::random::<f32>() > (1. - density_core) {
                spawn_core_star(
                    x as f32 - (width / 2.) + dx,
                    y as f32 - (height / 2.) + dy,
                    Some(energy),
                    None,
                    &mut cmd,
                    &star_core_texture,
                );
            } else if rand::random::<f32>() > (1. - density_simple) {
                let rng = rand::random::<f32>();
                let color = if rng < 0.4 {
                    Some(Color::hsl(250., 0.0, 0.0))
                } else if rng < 0.8 {
                    Some(Color::hsl(280., 0.0, 0.0))
                } else {
                    None
                };
                spawn_simple_star(
                    x as f32 - (width / 2.) + dx,
                    y as f32 - (height / 2.) + dy,
                    Some(energy),
                    color,
                    &mut cmd,
                    &star_simple_texture,
                );
            } else if rand::random::<f32>() > (1. - density_complex) {
                let rng = rand::random::<f32>();
                let color = if rng < 0.2 {
                    Color::hsl(55., 0., 0.)
                } else if rng < 0.4 {
                    Color::hsl(220., 0., 0.)
                } else if rng < 0.6 {
                    Color::hsl(320., 0., 0.3)
                } else if rng < 0.8 {
                    Color::hsl(0., 0., 0.)
                } else {
                    Color::WHITE
                };
                spawn_complex_star(
                    x as f32 - (width / 2.) + dx,
                    y as f32 - (height / 2.) + dy,
                    Some(energy),
                    Some(color),
                    &mut cmd,
                    &star_complex_texture,
                );
            }
        }
    }
}

// pub fn draw_line(mut painter: ShapePainter) {
//     let line_color = Color::ORANGE;

//     painter.thickness = 1.;
//     painter.color = line_color;

//     painter.line(Vec3::new(0., -100., 0.), Vec3::new(30., -100., 0.));
// }
