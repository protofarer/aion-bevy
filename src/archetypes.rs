use bevy::prelude::*;

use crate::{
    avatars::Heading,
    components::{MoveSpeed, Velocity},
    Speed, BOTTOM_WALL, INIT_SHIP_MOVE_SPEED, LEFT_WALL, RIGHT_WALL, TOP_WALL,
};

pub type ArchParticle = (SpriteBundle, MoveSpeed, Velocity);

pub fn gen_particle(
    x: f32,
    y: f32,
    heading: Option<Heading>,
    move_speed: Option<Speed>,
    color: Option<Color>,
) -> ArchParticle {
    let move_speed = match move_speed {
        Some(x) => MoveSpeed(x),
        None => MoveSpeed::default(),
    };
    let ms = move_speed.0;
    let heading = heading.unwrap_or_default();
    (
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(x, y, 0.),
                rotation: heading.into(),
                ..default()
            },
            sprite: Sprite {
                color: color.unwrap_or_default(),
                ..default()
            },
            ..default()
        },
        move_speed,
        Velocity(Vec2::new(ms * heading.0.x, ms * heading.0.y)),
    )
}
