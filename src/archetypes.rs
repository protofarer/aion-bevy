use bevy::prelude::*;

use crate::{
    avatars::Heading, components::MoveSpeed, Speed, BOTTOM_WALL, INIT_SHIP_MOVE_SPEED,
    INIT_SHIP_ROTATION, LEFT_WALL, RIGHT_WALL, TOP_WALL,
};

pub type ArchParticle = (SpriteBundle, MoveSpeed);

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
    (
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(LEFT_WALL + x, BOTTOM_WALL + y, 0.),
                rotation: heading.unwrap_or_default().into(),
                ..default()
            },
            sprite: Sprite {
                color: color.unwrap_or_default(),
                ..default()
            },
            ..default()
        },
        move_speed,
    )
}
