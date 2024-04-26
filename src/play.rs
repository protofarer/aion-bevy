use std::time::Instant;

use bevy::prelude::*;
use bevy_rapier2d::geometry::Collider;
use bevy_vector_shapes::{painter::ShapePainter, shapes::LinePainter};

use crate::{
    archetypes::ProjectileBundle,
    audio::ProjectileEmitSound,
    components::{
        DespawnDelay, FireType, PlayerShipTag, ProjectileEmission, ProjectileTag, Score, ScoreboardUi, TurnRate
    },
    game::{BOTTOM_WALL, LEFT_WALL, RIGHT_WALL, TOP_WALL},
};

pub fn draw_line(mut painter: ShapePainter) {
    let line_color = Color::ORANGE;

    painter.thickness = 1.;
    painter.color = line_color;

    painter.line(Vec3::new(0., -100., 0.), Vec3::new(30., -100., 0.));
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

pub fn ship_turn(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &TurnRate), With<PlayerShipTag>>,
    time: Res<Time>,
) {
    for (mut transform, turnrate) in query.iter_mut() {
        // let mut thrust = 0.;
        // if keyboard_input.pressed(KeyCode::KeyS) {
        //     thrust += 1.;
        // }
        // get fwd vector by applying current rot to ships init facing vec
        // let movement_direction = (transform.rotation * *DEFAULT_HEADING) * Vec3::X;
        // let movement_distance = thrust * movespeed.0 * time.delta_seconds();
        // let translation_delta = movement_direction * movement_distance;
        // transform.translation += translation_delta;

        let mut rotation_sign = 0.;
        if keyboard_input.pressed(KeyCode::KeyA) {
            rotation_sign += 1.;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            rotation_sign -= 1.;
        }
        transform.rotate_z(rotation_sign * turnrate.0 * time.delta_seconds());
    }
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

pub fn ship_fire(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut q_ship: Query<&Children, With<PlayerShipTag>>,
    mut q_emitter: Query<(&GlobalTransform, &mut ProjectileEmission, &FireType)>,
    fire_sound: Res<ProjectileEmitSound>,
) {
    // when fire key pressed
    if keyboard_input.pressed(KeyCode::Space) {
        // find ship, get children projectile emitters
        for children in &mut q_ship {
            for child in children {
                if let Ok((global_transform, mut emitter, firetype)) = q_emitter.get_mut(*child) {
                    // spawn primary fire projectile
                    match firetype {
                        FireType::Primary => {
                            let last_emit = emitter.last_emission_time;

                            if last_emit.elapsed().as_millis() as i32 >= emitter.cooldown_ms {
                                emitter.last_emission_time = Instant::now();

                                let (_scale, rotation, translation) =
                                    global_transform.to_scale_rotation_translation();

                                commands.spawn(ProjectileBundle::new(
                                    translation.x,
                                    translation.y,
                                    Some(rotation.into()),
                                    Some(emitter.projectile_speed),
                                    None,
                                    Some(emitter.damage),
                                    None,
                                    None,
                                ));
                                commands.spawn(AudioBundle {
                                    source: fire_sound.0.clone(),
                                    ..default()
                                });
                            }
                        }
                        _ => (),
                    };
                }
            }
        }
    }
}

pub fn update_scoreboard(scoreboard: Res<Score>, mut query: Query<&mut Text, With<ScoreboardUi>>) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.0.to_string();
}

pub fn despawn_delay(
    mut commands: Commands,
    mut query: Query<(Entity, &mut DespawnDelay), With<ProjectileTag>>,
    time: Res<Time>,
) {
    for (entity, mut despawn_delay) in &mut query {
        if despawn_delay.0.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
