#![allow(unused)]

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_vector_shapes::Shape2dPlugin;
use fps::{fps_counter_showhide, fps_text_update_system, setup_fps_counter};
use systems::setup;

mod fps;
mod play;
mod systems;

const BACKGROUND_COLOR: Color = Color::rgb(0., 0., 0.);
const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;
const PADDLE_SPEED: f32 = 500.;
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
        // .init_state::<GameState>()
        .add_systems(Startup, (setup, setup_fps_counter))
        .add_plugins((play::play_plugin))
        .run();
}

// #[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
// enum GameState {
//     #[default]
//     Menu,
//     Match,
//     End,
// }
