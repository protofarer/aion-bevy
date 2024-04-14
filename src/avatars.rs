use bevy::prelude::*;

use crate::{
    play::{Collider, Health, Player, ShipStats},
    BOTTOM_WALL, INIT_SHIP_MOVE_SPEED, INIT_SHIP_ROTATION, LEFT_WALL, RIGHT_WALL, TOP_WALL,
};

#[derive(Bundle)]
pub struct PlayerShip {
    sprite: SpriteBundle,
    stats: ShipStats,
    collider: Collider,
    health: Health,
    player: Player,
    // RigidBodyCpt,
    // RotatableBodyCpt,
    // ColorBodyCpt,
    // RotationalInputCpt,
    // ProjectileEmitterCpt,
    // HealthCpt,
}

impl Default for PlayerShip {
    fn default() -> Self {
        Self {
            sprite: SpriteBundle {
                transform: Transform {
                    // once texture used
                    // transform: Transform::from_xyz(
                    //     LEFT_WALL + (RIGHT_WALL - LEFT_WALL) / 2.,
                    //     BOTTOM_WALL + (TOP_WALL - BOTTOM_WALL) / 2.,
                    //     0.,
                    // ),
                    translation: Vec3::new(
                        LEFT_WALL + (RIGHT_WALL - LEFT_WALL) / 2.,
                        BOTTOM_WALL + (TOP_WALL - BOTTOM_WALL) / 2.,
                        0.,
                    ),
                    scale: Vec3::new(20., 50., 0.0),
                    rotation: INIT_SHIP_ROTATION,
                },
                ..default()
            },
            stats: ShipStats::default(),
            collider: Collider,
            health: Health::default(),
            player: Player::A,
        }
    }
}
