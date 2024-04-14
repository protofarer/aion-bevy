use bevy::prelude::*;

use crate::{
    avatars::{Heading, ParticleStats},
    play::{Collider, Health, Player, ProjectileStats, ShipStats},
    Speed, BOTTOM_WALL, INIT_SHIP_MOVE_SPEED, INIT_SHIP_ROTATION, LEFT_WALL, RIGHT_WALL, TOP_WALL,
};

pub type ArchParticle = (SpriteBundle, ParticleStats);

pub fn gen_particle(
    x: f32,
    y: f32,
    heading: Option<Heading>,
    speed: Option<Speed>,
    color: Option<Color>,
) -> ArchParticle {
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
        ParticleStats {
            move_speed: speed.unwrap_or_default(),
        },
    )
}
