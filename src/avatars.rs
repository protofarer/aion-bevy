use bevy::prelude::*;

use crate::{
    play::{Collider, Health, Player, ProjectileStats, ShipStats},
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

impl PlayerShip {
    pub fn new(x: f32, y: f32, heading: Option<Heading>) -> Self {
        Self {
            sprite: SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(LEFT_WALL + x, BOTTOM_WALL + y, 0.),
                    rotation: heading.unwrap_or_default().into(),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::GREEN,
                    ..default()
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

impl Default for PlayerShip {
    fn default() -> Self {
        Self {
            sprite: SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(
                        LEFT_WALL + (RIGHT_WALL - LEFT_WALL) / 2.,
                        BOTTOM_WALL + (TOP_WALL - BOTTOM_WALL) / 2.,
                        0.,
                    ),
                    scale: Vec3::new(20., 50., 0.0),
                    rotation: INIT_SHIP_ROTATION,
                },
                sprite: Sprite {
                    color: Color::GREEN,
                    ..default()
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

#[derive(Bundle)]
pub struct Boxoid {
    sprite: SpriteBundle,
    collider: Collider,
    health: Health,
}

impl Boxoid {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            sprite: SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(LEFT_WALL + x, BOTTOM_WALL + y, 0.),
                    scale: Vec3::new(50., 50., 0.0),
                    rotation: INIT_SHIP_ROTATION,
                },
                sprite: Sprite {
                    color: Color::ORANGE_RED,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
            health: Health(1),
        }
    }
}

impl Default for Boxoid {
    fn default() -> Self {
        Self {
            sprite: SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(
                        LEFT_WALL + (RIGHT_WALL - LEFT_WALL) / 2.,
                        BOTTOM_WALL + (TOP_WALL - BOTTOM_WALL) / 2.,
                        0.,
                    ),
                    scale: Vec3::new(50., 50., 0.0),
                    rotation: INIT_SHIP_ROTATION,
                },
                ..default()
            },
            collider: Collider,
            health: Health(1),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Heading(Vec3);

impl Heading {
    pub fn from_angle(angle_degrees: f32) -> Self {
        let angle_radians = angle_degrees.to_radians();
        let x = angle_radians.cos();
        let y = angle_radians.sin();
        Heading(Vec3::new(x, y, 0.))
    }
}

impl Default for Heading {
    fn default() -> Self {
        Heading(INIT_SHIP_ROTATION.xyz())
    }
}

impl Into<Quat> for Heading {
    fn into(self) -> Quat {
        let angle_radians = self.0.y.atan2(self.0.x);
        Quat::from_rotation_z(angle_radians)
    }
}

#[derive(Bundle)]
pub struct Projectile {
    sprite: SpriteBundle,
    stats: ProjectileStats,
    collider: Collider,
}

impl Projectile {
    pub fn new(x: f32, y: f32, heading: Option<Heading>, color: Option<Color>) -> Self {
        let rotation = heading.unwrap_or_default().into();
        Self {
            sprite: SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(LEFT_WALL + x, BOTTOM_WALL + y, 0.),
                    rotation,
                    ..default()
                },
                sprite: Sprite {
                    color: color.unwrap_or_default(),
                    ..default()
                },
                ..default()
            },
            stats: ProjectileStats::default(),
            collider: Collider,
        }
    }
}

impl Default for Projectile {
    fn default() -> Self {
        Self {
            sprite: SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(
                        LEFT_WALL + (RIGHT_WALL - LEFT_WALL) / 2.,
                        BOTTOM_WALL + (TOP_WALL - BOTTOM_WALL) / 2.,
                        0.,
                    ),
                    rotation: INIT_SHIP_ROTATION,
                    ..default()
                },
                sprite: Sprite {
                    color: Color::ORANGE_RED,
                    ..default()
                },
                ..default()
            },
            stats: ProjectileStats::default(),
            collider: Collider,
        }
    }
}
