#![allow(unused)]

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, window::WindowMode};
use bevy_particle_systems::ParticleSystemPlugin;
use bevy_vector_shapes::Shape2dPlugin;
use fps::{fps_counter_showhide, fps_text_update_system, setup_fps_counter};
use game::{LOGICAL_HEIGHT, LOGICAL_WIDTH};

mod archetypes;
mod audio;
mod avatars;
mod components;
mod controls;
mod effects;
mod events;
mod fps;
mod game;
mod physics;
mod play;
mod utils;

const BACKGROUND_COLOR: Color = Color::rgb(0., 0., 0.);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Aion v0.1.0".to_string(),
                position: WindowPosition::Centered(MonitorSelection::Index(2)),
                resolution: (LOGICAL_WIDTH, LOGICAL_HEIGHT).into(),
                // mode: WindowMode::Fullscreen,
                // resolution: (LOGICAL_WIDTH * 0.75, LOGICAL_HEIGHT * 0.75).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(Shape2dPlugin::default())
        .add_plugins(ParticleSystemPlugin)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, (setup_fps_counter).chain())
        .add_systems(
            Update,
            (
                fps_text_update_system,
                fps_counter_showhide,
                bevy::window::close_on_esc,
            ),
        )
        .add_plugins(game::game_plugin)
        .run();
}
