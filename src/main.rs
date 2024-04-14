#![allow(unused)]

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_vector_shapes::Shape2dPlugin;
use fps::{fps_counter_showhide, fps_text_update_system, setup_fps_counter};
use systems::setup;

mod avatars;
mod archetypes;
mod fps;
mod play;
mod systems;

type Speed = f32;
type TurnRate = f32;

const BACKGROUND_COLOR: Color = Color::rgb(0., 0., 0.);
const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;
const INIT_SHIP_MOVE_SPEED: Speed = 300.;
const INIT_SHIP_TURN_RATE: TurnRate = 5.;
const INIT_LIVES: usize = 2;
const INIT_SHIP_ROTATION: Quat = Quat::from_xyzw(0., 0., 1., 0.);
const INIT_HEALTH: usize = 10;
const INIT_SHIP_PROJECTILE_MOVE_SPEED: f32 = 500.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Aion".to_string(),
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
