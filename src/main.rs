#![allow(unused)]

use std::f32::consts::PI;

use audio::{
    AsteroidClashSound, AsteroidDestroyedSound, ProjectileEmitSound, ProjectileImpactSound,
    ShipDamagedSound, ShipDestroyedSound, ShipThrustSound, ShipThrustSoundStopwatch,
};
use lazy_static::lazy_static;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, time::Stopwatch};
use bevy_vector_shapes::Shape2dPlugin;
use fps::{fps_counter_showhide, fps_text_update_system, setup_fps_counter};
use play::setup_play;

use crate::utils::Heading;

mod archetypes;
mod audio;
mod avatars;
mod components;
mod fps;
mod physics;
mod play;
mod utils;

pub type Speed = f32;
pub type TurnSpeed = f32;

// CONSTANTS
const AMBIENT_LINEAR_FRICTION_COEFFICIENT: f32 = 0.6;
const AMBIENT_ANGULAR_FRICTION_COEFFICIENT: f32 = 1.0;
const BACKGROUND_COLOR: Color = Color::rgb(0., 0., 0.);

// play area dims assuming 1920x1080 window with 20% saved for debug and UI
const LEFT_WALL: f32 = -768.;
const RIGHT_WALL: f32 = 768.;
const BOTTOM_WALL: f32 = -432.;
const TOP_WALL: f32 = 432.;

// General
const DEFAULT_MOVESPEED: Speed = 100.;
const DEFAULT_HEALTH: i32 = 1;
const DEFAULT_PROJECTILE_EMISSION_COOLDOWN: i32 = 100;
lazy_static! {
    static ref DEFAULT_HEADING: Heading = Heading(0.);
    static ref DEFAULT_ROTATION: Quat = Quat::from_rotation_z(0.);
}
const DEFAULT_TURNRATE: f32 = 10.;
const DEFAULT_DAMAGE: i32 = 1;
const DEFAULT_DURATION_SECS: u64 = 5;
const DEFAULT_RESTITUTION: f32 = 0.5;
const DEFAULT_THRUST_FORCE_MAGNITUDE: f32 = 50000.;

// UI
const SCOREBOARD_FONT_SIZE: f32 = 20.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
const LABEL_COLOR: Color = Color::LIME_GREEN;
const SCORE_COLOR: Color = Color::LIME_GREEN;

// Ship
const INIT_SHIP_MOVE_SPEED: Speed = 300.;
const INIT_SHIP_TURN_RATE: TurnSpeed = 5.;
const INIT_SHIP_HEALTH: i32 = 3;
const INIT_SHIP_PROJECTILE_SPEED: f32 = 500.;
const INIT_SHIP_RESTITUTION: f32 = 0.9;
const SHIP_LENGTH: f32 = 22.;
const SHIP_HALF_WIDTH: f32 = 15.;

// Asteroid
const INIT_ASTEROID_MOVESPEED: Speed = 300.;
const INIT_ASTEROID_DAMAGE: i32 = 1;
const INIT_ASTEROID_RESTITUTION: f32 = 0.3;

const SMALL_ASTEROID_R: f32 = 15.;
const SMALL_ASTEROID_HEALTH: i32 = 1;

const MEDIUM_ASTEROID_R: f32 = 30.;
const MEDIUM_ASTEROID_HEALTH: i32 = 3;

const LARGE_ASTEROID_R: f32 = 50.;
const LARGE_ASTEROID_HEALTH: i32 = 5;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Aion".to_string(),
                resolution: (1920., 1080.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(Shape2dPlugin::default())
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .init_state::<GameState>()
        .add_systems(Startup, (setup, setup_play, setup_fps_counter).chain()) // setup_play here while no scenes impl'd
        .add_systems(Update, (fps_text_update_system, fps_counter_showhide))
        .add_plugins(play::play_plugin)
        .add_plugins(physics::physics_plugin)
        .run();
}

pub fn setup(
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

    let asteroid_material_handle = materials.add(Color::GRAY);
    commands.insert_resource(AsteroidMaterialHandles(vec![asteroid_material_handle]));

    let mut asteroid_mesh_handles = vec![];
    for n_sides in [5, 6, 8] {
        for r in [SMALL_ASTEROID_R, MEDIUM_ASTEROID_R, LARGE_ASTEROID_R] {
            let handle_mesh = meshes.add(RegularPolygon::new(r, n_sides));
            asteroid_mesh_handles.push(handle_mesh);
            // ???
        }
    }
    commands.insert_resource(AsteroidMeshHandles(asteroid_mesh_handles));

    // let wall_collision_sound = asset_server.load("sounds/breakout_collision.ogg");
    // let paddle_collision_sound = asset_server.load("sounds/med_shoot.wav");
    // let goal_collision_sound = asset_server.load("sounds/jump.wav");
    // commands.insert_resource(CollisionSound {
    //     wall: wall_collision_sound,
    //     paddle: paddle_collision_sound,
    //     goal: goal_collision_sound,
    // });
    // commands.insert_resource(Scores { a: 0, b: 0 });
    // commands.insert_resource(MatchInfo {
    //     round_count: 0,
    //     rounds_total: ROUNDS_TOTAL,
    // });
    // commands.insert_resource(RoundData {
    //     paddle_hit_count: 0,
    // });
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    // Menu,
    #[default]
    Match,
    End,
}

#[derive(Resource)]
pub struct AsteroidMeshHandles(Vec<Handle<Mesh>>);

#[derive(Resource)]
pub struct AsteroidMaterialHandles(Vec<Handle<ColorMaterial>>);

#[derive(Resource)]
pub struct PlayerShipMeshHandle(Handle<Mesh>);

#[derive(Resource)]
pub struct PlayerShipMaterialHandle(Handle<ColorMaterial>);
