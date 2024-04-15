#![allow(unused)]

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_vector_shapes::Shape2dPlugin;
use fps::{fps_counter_showhide, fps_text_update_system, setup_fps_counter};
use systems::setup;

mod archetypes;
mod avatars;
mod components;
mod fps;
mod play;
mod systems;

pub type Speed = f32;
pub type TurnSpeed = f32;

const BACKGROUND_COLOR: Color = Color::rgb(0., 0., 0.);

// play area dims assuming 1920x1080 window with 20% saved for debug and UI
const LEFT_WALL: f32 = -768.;
const RIGHT_WALL: f32 = 768.;
const BOTTOM_WALL: f32 = -432.;
const TOP_WALL: f32 = 432.;

// General initialization constants
const DEFAULT_MOVESPEED: Speed = 100.;
const DEFAULT_HEALTH: i32 = 1;
const DEFAULT_HEADING: Quat = Quat::from_xyzw(0., 0., 0.71, 0.71);
const DEFAULT_BODY_ROTATION_RATE: f32 = 0.;
const DEFAULT_TURNRATE: f32 = 10.;
const DEFAULT_DAMAGE: i32 = 1;
const DEFAULT_DURATION_SECS: u64 = 5;
const DEFAULT_VELOCITY: Vec2 = Vec2::new(0., 0.);

// Ship initialization constants
const INIT_SHIP_MOVE_SPEED: Speed = 300.;
const INIT_SHIP_TURN_RATE: TurnSpeed = 5.;
const INIT_SHIP_ROTATION: Quat = DEFAULT_HEADING;
const INIT_SHIP_HEALTH: i32 = 3;
const INIT_SHIP_PROJECTILE_MOVE_SPEED: f32 = 500.;

// Asteroid initialization constants
const INIT_ASTEROID_MOVE_SPEED: Speed = 300.;
const INIT_ASTEROID_TURN_RATE: TurnSpeed = 5.;
const INIT_ASTEROID_ROTATION: Quat = DEFAULT_HEADING;
const INIT_ASTEROID_HEALTH: i32 = 1;
const INIT_ASTEROID_PROJECTILE_MOVE_SPEED: f32 = 500.;
const SMALL_ASTEROID_R: f32 = 15.;
const MEDIUM_ASTEROID_R: f32 = 30.;
const LARGE_ASTEROID_R: f32 = 50.;

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
        .add_systems(Update, (fps_text_update_system, fps_counter_showhide))
        .add_plugins(Shape2dPlugin::default())
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .init_state::<GameState>()
        .add_systems(Startup, (setup, setup_fps_counter))
        .add_plugins((play::play_plugin))
        .run();
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    // Menu,
    #[default]
    Match,
    End,
}
