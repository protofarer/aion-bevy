use bevy::{
    pbr::wireframe::Wireframe,
    prelude::*,
    sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle},
    utils::{Duration, Instant},
};
use rand::Rng;

use crate::{
    archetypes::gen_particle,
    components::{BodyRotationRate, Collider, Damage, Health, MoveSpeed, Player, TransientExistence, TurnRate},
    Speed, BOTTOM_WALL, INIT_SHIP_HEALTH, INIT_SHIP_MOVE_SPEED, INIT_SHIP_ROTATION,
    INIT_SHIP_TURN_RATE, LEFT_WALL, RIGHT_WALL, TOP_WALL,
};

#[derive(Bundle)]
pub struct PlayerShip<M: Material2d> {
    // sprite: SpriteBundle,
    mesh_bundle: MaterialMesh2dBundle<M>,
    move_speed: MoveSpeed,
    turn_rate: TurnRate,
    collider: Collider,
    health: Health,
    player: Player,
    // ProjectileEmitterCpt,
}

impl<M: Material2d> PlayerShip<M> {
    pub fn new(
        x: f32,
        y: f32,
        heading: Option<Heading>,
        mesh: Handle<Mesh>,
        material: Handle<M>,
    ) -> Self {
        Self {
            mesh_bundle: MaterialMesh2dBundle {
                mesh: mesh.into(),
                material,
                transform: Transform {
                    translation: Vec3::new(x, y, 1.),
                    rotation: heading.unwrap_or_default().into(),
                    ..default()
                },
                // ::from_translation(Vec3::new(0., 0., 0.))
                //     .with_scale(Vec2::splat(50.).extend(1.)),
                ..default()
            },
            collider: Collider,
            health: Health(INIT_SHIP_HEALTH),
            player: Player::A,
            move_speed: MoveSpeed(INIT_SHIP_MOVE_SPEED),
            turn_rate: TurnRate(INIT_SHIP_TURN_RATE),
        }
    }
}

#[derive(Bundle)]
pub struct Asteroid<M: Material2d> {
    // sprite: SpriteBundle,
    mesh_bundle: MaterialMesh2dBundle<M>,
    collider: Collider,
    health: Health,
    body_rotation_rate: BodyRotationRate,
    move_speed: MoveSpeed,
    damage: Damage,
}

impl<M: Material2d> Asteroid<M> {
    pub fn new(
        x: f32,
        y: f32,
        r: f32,
        mesh: Handle<Mesh>,
        material: Handle<M>,
        heading: Option<Heading>,
        move_speed: Option<Speed>,
        damage: Option<i32>,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let body_rotation_rate = (rng.gen::<f32>() * 0.1) - 0.05;
        let move_speed = match move_speed {
            Some(x) => MoveSpeed(x),
            None => MoveSpeed::default(),
        };
        let damage = match damage {
            Some(x) => Damage(x),
            None => Damage::default(),
        };
        Self {
            mesh_bundle: MaterialMesh2dBundle {
                mesh: mesh.into(),
                material,
                transform: Transform {
                    translation: Vec3::new(x, y, 2.),
                    rotation: heading.unwrap_or_default().into(),
                    scale: Vec2::splat(1.).extend(1.),
                    ..default()
                },
                ..default()
            },
            move_speed,
            damage,
            collider: Collider,
            health: Health::default(),
            body_rotation_rate: BodyRotationRate(body_rotation_rate),
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
pub struct Particle {
    sprite: SpriteBundle,
    move_speed: MoveSpeed,
    transient_existence: TransientExistence,
}

impl Particle {
    pub fn new(
        x: f32,
        y: f32,
        heading: Option<Heading>,
        move_speed: Option<Speed>,
        color: Option<Color>,
        duration: Option<Duration>,
    ) -> Self {
        let move_speed = match move_speed {
            Some(x) => MoveSpeed(x),
            None => MoveSpeed::default(),
        };
        let transient_existence = match duration {
            Some(x) => TransientExistence::new(x),
            None => TransientExistence::default(),
        };

        Self {
            sprite: SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(LEFT_WALL + x, BOTTOM_WALL + y, 0.),
                    rotation: INIT_SHIP_ROTATION,
                    ..default()
                },
                sprite: Sprite {
                    color: color.unwrap_or_default(),
                    ..default()
                },
                ..default()
            },
            move_speed,
            transient_existence,
        }
    }
}

impl Default for Particle {
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
                ..default()
            },
            move_speed: MoveSpeed::default(),
            transient_existence: TransientExistence::default(),
        }
    }
}

#[derive(Bundle)]
pub struct Projectile {
    sprite: SpriteBundle,
    collider: Collider,
    damage: Damage,
    move_speed: MoveSpeed,
    transient_existence: TransientExistence,
}

impl Projectile {
    pub fn new(
        x: f32,
        y: f32,
        heading: Option<Heading>,
        move_speed: Option<Speed>,
        color: Option<Color>,
        damage: Option<i32>,
        duration: Option<Duration>,
    ) -> Self {
        let particle = gen_particle(x, y, heading, move_speed, color);
        let sprite = particle.0;
        let damage = match damage {
            Some(x) => Damage(x),
            None => Damage::default(),
        };
        let transient_existence = match duration {
            Some(x) => TransientExistence::new(x),
            None => TransientExistence::default(),
        };
        let move_speed = match move_speed {
            Some(x) => MoveSpeed(x),
            None => MoveSpeed::default(),
        };
        Self {
            sprite,
            collider: Collider,
            damage,
            transient_existence,
            move_speed,
        }
    }
}

impl Default for Projectile {
    fn default() -> Self {
        let particle = gen_particle(0., 0., None, None, None);
        let sprite = particle.0;
        let stats = particle.1;
        Self {
            sprite,
            collider: Collider,
            damage: Damage::default(),
            transient_existence: TransientExistence::default(),
            move_speed: MoveSpeed::default(),
        }
    }
}
