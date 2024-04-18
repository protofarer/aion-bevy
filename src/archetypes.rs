use bevy::prelude::*;
use bevy_rapier2d::{dynamics::Velocity, parry::simba::scalar::SupersetOf};

use crate::{
    avatars::Heading, components::MoveSpeed, Speed, BOTTOM_WALL, DEFAULT_MOVESPEED,
    INIT_SHIP_MOVE_SPEED, LEFT_WALL, RIGHT_WALL, TOP_WALL,
};

pub type ArchParticle = (SpriteBundle, Velocity);

pub fn gen_particle(
    x: f32,
    y: f32,
    heading: Option<Heading>,
    move_speed: Option<Speed>,
    color: Option<Color>,
) -> ArchParticle {
    let move_speed = match move_speed {
        Some(x) => x,
        None => DEFAULT_MOVESPEED,
    };
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
        Velocity {
            linvel: Vec2::new(heading.0.x, heading.0.y) * move_speed,
            angvel: 0.,
        },
    )
}
