#![allow(unused)]

use std::fmt::Debug;

use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
    window::{WindowMode, WindowResized},
};
use bevy_particle_systems::ParticleSystemPlugin;
use bevy_vector_shapes::{painter::ShapePainter, shapes::LinePainter, Shape2dPlugin};
use fps::{fps_counter_showhide, fps_text_update_system, setup_fps_counter, FpsRoot};
use game::{BOTTOM_WALL, LEFT_WALL, LOGICAL_HEIGHT, LOGICAL_WIDTH, RIGHT_WALL, TOP_WALL};

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
        .insert_resource(ShowDebugDisplay(false))
        .insert_resource(ResolutionSettings {
            large: Vec2::new(1920., 1080.),
            medium: Vec2::new(800., 600.),
            small: Vec2::new(640., 360.),
        })
        .add_systems(Startup, (setup_fps_counter, setup_resolution_display))
        .add_systems(
            Update,
            (
                (
                    bevy::window::close_on_esc,
                    toggle_debug_display,
                    update_debug_display_visibility,
                    on_resize_system,
                    toggle_resolution,
                ),
                (fps_text_update_system, fps_counter_showhide, draw_grid)
                    .run_if(is_debug_display_on),
            ),
        )
        .add_plugins(game::game_plugin)
        .run();
}

#[derive(Resource, Deref, DerefMut)]
struct ShowDebugDisplay(bool);

#[derive(Component)]
struct OnDebugDisplay;

#[derive(Component)]
struct ResolutionText;

#[derive(Resource)]
struct ResolutionSettings {
    large: Vec2,
    medium: Vec2,
    small: Vec2,
}

fn is_debug_display_on(show_debug_display: Res<ShowDebugDisplay>) -> bool {
    **show_debug_display
}

fn toggle_debug_display(
    keys: Res<ButtonInput<KeyCode>>,
    mut show_debug_display: ResMut<ShowDebugDisplay>,
) {
    if keys.just_pressed(KeyCode::Backquote) {
        **show_debug_display = !**show_debug_display;
    }
}

// TODO all use OnDebugDisplay,
fn update_debug_display_visibility(
    show_debug_display: Res<ShowDebugDisplay>,
    mut q_debug_visibility: Query<&mut Visibility, With<OnDebugDisplay>>,
    // mut q_fps: Query<&mut Visibility, (Without<Node>, Without<OnDebugDisplay>, With<FpsRoot>)>,
) {
    if show_debug_display.is_changed() {
        for mut vis in &mut q_debug_visibility {
            *vis = match **show_debug_display {
                true => Visibility::Visible,
                false => Visibility::Hidden,
            };
        }
        // let mut fps_vis = q_fps.single_mut();
        // *fps_vis = match **show_debug_display {
        //     true => Visibility::Visible,
        //     _ => Visibility::Hidden,
        // };
    }
}

fn setup_resolution_display(mut cmd: Commands) {
    cmd.spawn((
        NodeBundle {
            background_color: BackgroundColor(Color::BLACK.with_a(0.5)),
            z_index: ZIndex::Global(i32::MAX),
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Percent(1.),
                top: Val::Percent(3.),
                bottom: Val::Auto,
                left: Val::Auto,
                padding: UiRect::all(Val::Px(4.0)),
                ..default()
            },
            ..default()
        },
        OnDebugDisplay,
    ))
    .with_children(|root| {
        root.spawn((
            TextBundle::from_section(
                "Resolution",
                TextStyle {
                    font_size: 20.0,
                    color: Color::LIME_GREEN,
                    ..default()
                },
            ),
            ResolutionText,
        ));
    });
}

/// This system shows how to request the window to a new resolution
fn toggle_resolution(
    keys: Res<ButtonInput<KeyCode>>,
    mut windows: Query<&mut Window>,
    resolution: Res<ResolutionSettings>,
) {
    let mut window = windows.single_mut();

    if keys.just_pressed(KeyCode::Digit1) {
        let res = resolution.small;
        window.resolution.set(res.x, res.y);
    }
    if keys.just_pressed(KeyCode::Digit2) {
        let res = resolution.medium;
        window.resolution.set(res.x, res.y);
    }
    if keys.just_pressed(KeyCode::Digit3) {
        let res = resolution.large;
        window.resolution.set(res.x, res.y);
    }
}

/// This system shows how to respond to a window being resized.
/// Whenever the window is resized, the text will update with the new resolution.
fn on_resize_system(
    mut q: Query<&mut Text, With<ResolutionText>>,
    mut resize_reader: EventReader<WindowResized>,
) {
    let mut text = q.single_mut();
    for e in resize_reader.read() {
        // When resolution is being changed
        text.sections[0].value = format!("{:.1} x {:.1}", e.width, e.height);
    }
}

fn draw_grid(mut painter: ShapePainter) {
    let height = TOP_WALL - BOTTOM_WALL;
    let width = RIGHT_WALL - LEFT_WALL;
    let line_color = Color::rgba(1., 1., 1., 0.025);

    painter.thickness = 1.;
    painter.color = line_color;

    let s = 100;

    for x in (0..(width / 2.0) as usize).step_by(s) {
        painter.line(
            Vec3::new(x as f32, -height / 2., 0.),
            Vec3::new(x as f32, height / 2., 0.),
        );
        painter.line(
            Vec3::new(-(x as f32), -height / 2., 0.),
            Vec3::new(-(x as f32), height / 2., 0.),
        );
    }
    for y in (0..(height / 2.0) as usize).step_by(s) {
        painter.line(
            Vec3::new(-width / 2., y as f32, 0.),
            Vec3::new(width / 2., y as f32, 0.),
        );
        painter.line(
            Vec3::new(-width / 2., -(y as f32), 0.),
            Vec3::new(width / 2., -(y as f32), 0.),
        );
    }
}
