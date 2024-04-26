use lazy_static::lazy_static;

use bevy::prelude::*;
use bevy::time::Stopwatch;
use bevy_rapier2d::prelude::*;

use bevy_particle_systems::{
    ColorOverTime, Curve, CurvePoint, EmitterShape, JitteredValue, ParticleSystem,
    ParticleSystemBundle, Playing,
};

use crate::archetypes::AsteroidSizes;
use crate::audio::{
    AsteroidClashSound, AsteroidDestroyedSound, ProjectileEmitSound, ProjectileImpactSound,
    ShipDamagedSound, ShipDestroyedSound, ShipThrustSound, ShipThrustSoundStopwatch,
};
use crate::avatars::{gen_asteroid, gen_playership};
use crate::components::{Score, ScoreboardUi};
use crate::physics::{
    apply_forces_ship, emit_collision_particles, emit_thruster_particles,
    handle_projectile_collision_events,
};
use crate::play::{
    despawn_delay, draw_boundary, draw_line, ship_fire, ship_turn, update_scoreboard, wraparound,
};
use crate::utils::Heading;

// NEWTYPES
// deleteme
pub type Speed = f32;
pub type TurnSpeed = f32;

// CONSTANTS
pub const AMBIENT_LINEAR_FRICTION_COEFFICIENT: f32 = 0.6;
pub const AMBIENT_ANGULAR_FRICTION_COEFFICIENT: f32 = 1.0;

// play area dims assuming 1920x1080 window with 20% saved for debug and UI
pub const LEFT_WALL: f32 = -768.;
pub const RIGHT_WALL: f32 = 768.;
pub const BOTTOM_WALL: f32 = -432.;
pub const TOP_WALL: f32 = 432.;

// General
pub const DEFAULT_MOVESPEED: Speed = 100.;
pub const DEFAULT_HEALTH: i32 = 1;
pub const DEFAULT_PROJECTILE_EMISSION_COOLDOWN: i32 = 100;
lazy_static! {
    pub static ref DEFAULT_HEADING: Heading = Heading(0.);
    pub static ref DEFAULT_ROTATION: Quat = Quat::from_rotation_z(0.);
}
pub const DEFAULT_TURNRATE: f32 = 10.;
pub const DEFAULT_DAMAGE: i32 = 1;
pub const DEFAULT_DURATION_SECS: u64 = 5;
pub const DEFAULT_RESTITUTION: f32 = 0.5;
pub const DEFAULT_THRUST_FORCE_MAGNITUDE: f32 = 50000.;

// UI
pub const SCOREBOARD_FONT_SIZE: f32 = 20.0;
pub const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
pub const LABEL_COLOR: Color = Color::LIME_GREEN;
pub const SCORE_COLOR: Color = Color::LIME_GREEN;

// Ship
pub const INIT_SHIP_MOVE_SPEED: Speed = 300.;
pub const INIT_SHIP_TURN_RATE: TurnSpeed = 5.;
pub const INIT_SHIP_HEALTH: i32 = 3;
pub const INIT_SHIP_PROJECTILE_SPEED: f32 = 500.;
pub const INIT_SHIP_RESTITUTION: f32 = 0.9;
pub const SHIP_LENGTH: f32 = 22.;
pub const SHIP_HALF_WIDTH: f32 = 15.;

// Asteroid
pub const INIT_ASTEROID_MOVESPEED: Speed = 300.;
pub const INIT_ASTEROID_DAMAGE: i32 = 1;
pub const INIT_ASTEROID_RESTITUTION: f32 = 0.3;

pub const SMALL_ASTEROID_R: f32 = 15.;
pub const SMALL_ASTEROID_HEALTH: i32 = 1;

pub const MEDIUM_ASTEROID_R: f32 = 30.;
pub const MEDIUM_ASTEROID_HEALTH: i32 = 3;

pub const LARGE_ASTEROID_R: f32 = 50.;
pub const LARGE_ASTEROID_HEALTH: i32 = 5;

pub fn game_plugin(app: &mut App) {
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(2.))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, (load_assets, setup_game).chain())
        .add_systems(
            FixedUpdate,
            (
                ship_turn,
                wraparound,
                ship_fire,
                despawn_delay,
                apply_forces_ship,
                handle_projectile_collision_events, // check_for_collisions,
                                                    // play_collision_sound,
                                                    // process_score,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                bevy::window::close_on_esc,
                draw_boundary,
                draw_line,
                update_scoreboard,
                emit_thruster_particles,
                emit_collision_particles,
            ),
        )
        // .configure_sets(Update, PlaySet.run_if(in_state(GameState::Match)))
        // .configure_sets(FixedUpdate, PlaySet.run_if(in_state(GameState::Match)))
        .insert_resource(Score(0));
}

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.insert_resource(ShipThrustSoundStopwatch(Stopwatch::new()));

    // let background_music = asset_server.load("sounds/Windless Slopes.ogg");
    // commands.insert_resource(BackgroundMusic(background_music));

    let light_shot_sound = asset_server.load("sounds/light_shot.wav");
    commands.insert_resource(ProjectileEmitSound(light_shot_sound));

    let ship_thrust_sound = asset_server.load("sounds/thrust.wav");
    commands.insert_resource(ShipThrustSound(ship_thrust_sound));

    let projectile_impact_sound = asset_server.load("sounds/scratch.wav");
    commands.insert_resource(ProjectileImpactSound(projectile_impact_sound));

    let destroy_asteroid_sound = asset_server.load("sounds/destroy_asteroid.wav");
    commands.insert_resource(AsteroidDestroyedSound(destroy_asteroid_sound));

    let damage_ship_sound = asset_server.load("sounds/ship_damage.wav");
    commands.insert_resource(ShipDamagedSound(damage_ship_sound));

    let destroy_ship_sound = asset_server.load("sounds/human_physical_death.wav");
    commands.insert_resource(ShipDestroyedSound(destroy_ship_sound));

    let asteroid_clash_sound = asset_server.load("sounds/asteroid_clash.wav");
    commands.insert_resource(AsteroidClashSound(asteroid_clash_sound));

    let handle_playership_mesh = meshes.add(Triangle2d::new(
        Vec2::new(-SHIP_HALF_WIDTH, -SHIP_HALF_WIDTH),
        Vec2::X * SHIP_LENGTH,
        Vec2::new(-SHIP_HALF_WIDTH, SHIP_HALF_WIDTH),
    ));
    commands.insert_resource(PlayerShipMeshHandle(handle_playership_mesh));

    let handle_playership_colormaterial = materials.add(Color::LIME_GREEN);
    commands.insert_resource(PlayerShipMaterialHandle(handle_playership_colormaterial));

    let asteroid_material = materials.add(Color::GRAY);
    commands.insert_resource(AsteroidMaterialHandles(vec![asteroid_material]));

    let mut asteroid_mesh_handles = vec![];
    for n_sides in [5, 6, 8] {
        for r in [SMALL_ASTEROID_R, MEDIUM_ASTEROID_R, LARGE_ASTEROID_R] {
            let handle_mesh = meshes.add(RegularPolygon::new(r, n_sides));
            asteroid_mesh_handles.push(handle_mesh);
            // ???
        }
    }
    commands.insert_resource(AsteroidMeshHandles(asteroid_mesh_handles));

    let thruster_particle_texture = asset_server.load("px.png").into();
    commands.insert_resource(ThrustParticleTexture(thruster_particle_texture));
}

pub fn setup_game(
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
    commands.spawn(ship).with_children(|parent| {
        parent.spawn(children.0);
        parent.spawn(children.1);
    });

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
    commands.spawn(ast1);

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
        commands.spawn(gen_asteroid(
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
        ));
        commands.spawn(gen_asteroid(
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
        ));
    }

    // Diagonal collision, see collision particles
    commands.spawn(gen_asteroid(
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
    ));
    commands.spawn(gen_asteroid(
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
    ));
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
        .insert(Playing);

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

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct PlaySet;

#[derive(Resource)]
pub struct AsteroidMeshHandles(Vec<Handle<Mesh>>);

#[derive(Resource)]
pub struct AsteroidMaterialHandles(Vec<Handle<ColorMaterial>>);

#[derive(Resource)]
pub struct PlayerShipMeshHandle(Handle<Mesh>);

#[derive(Resource)]
pub struct PlayerShipMaterialHandle(Handle<ColorMaterial>);

#[derive(Resource)]
pub struct ParticleMeshHandle(Handle<Mesh>);

#[derive(Resource)]
pub struct ThrustParticleTexture(pub Handle<Image>);
