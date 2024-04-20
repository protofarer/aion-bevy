#![allow(unused)]

use avatars::Heading;
use lazy_static::lazy_static;
use std::f32::consts::PI;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_rapier2d::prelude::*;
use bevy_vector_shapes::Shape2dPlugin;
use fps::{fps_counter_showhide, fps_text_update_system, setup_fps_counter};
use systems::setup;

mod archetypes;
mod avatars;
mod components;
mod fps;
mod physics;
mod play;
mod systems;

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
lazy_static! {
    static ref DEFAULT_HEADING: Heading = Heading::from_angle(0.);
    static ref DEFAULT_ROTATION: Quat = Quat::from_rotation_z(0.);
}
const DEFAULT_BODY_ROTATION_RATE: f32 = 0.;
const DEFAULT_TURNRATE: f32 = 10.;
const DEFAULT_DAMAGE: i32 = 1;
const DEFAULT_DURATION_SECS: u64 = 5;
const DEFAULT_VELOCITY: Vec2 = Vec2::new(100., 0.);
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
const INIT_SHIP_PROJECTILE_MOVE_SPEED: f32 = 500.;

// Asteroid
const INIT_ASTEROID_MOVE_SPEED: Speed = 300.;
const INIT_ASTEROID_HEALTH: i32 = 1;
const INIT_ASTEROID_DAMAGE: i32 = 1;
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
        .add_plugins(Shape2dPlugin::default())
        .add_plugins(play::play_plugin)
        .add_plugins(physics::physics_plugin)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .init_state::<GameState>()
        .add_systems(Startup, (setup, setup_fps_counter))
        .add_systems(Update, (fps_text_update_system, fps_counter_showhide))
        .run();
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    // Menu,
    #[default]
    Match,
    End,
}
