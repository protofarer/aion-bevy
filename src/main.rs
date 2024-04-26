// #![allow(unused)]

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_particle_systems::ParticleSystemPlugin;
use bevy_vector_shapes::Shape2dPlugin;
use fps::{fps_counter_showhide, fps_text_update_system, setup_fps_counter};

mod play;
mod archetypes;
mod audio;
mod avatars;
mod components;
mod fps;
mod game;
mod physics;
mod utils;

const BACKGROUND_COLOR: Color = Color::rgb(0., 0., 0.);

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
        .add_plugins(ParticleSystemPlugin)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, (setup_fps_counter).chain()) // setup_play here while no scenes impl'd
        .add_systems(Update, (fps_text_update_system, fps_counter_showhide))
        .add_plugins(game::game_plugin)
        .run();
}

// #[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
// enum GameState {
//     // Menu,
//     #[default]
//     Match,
//     End,
// }
