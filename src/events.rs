use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;
use bevy_particle_systems::*;
use bevy_rapier2d::{dynamics::Velocity, pipeline::CollisionEvent};

use crate::{
    audio::{ProjectileImpactSound, ShipDamagedSound},
    components::{AsteroidTag, CollisionRadius, DespawnDelay, PlayerShipTag, ProjectileTag},
    game::ParticlePixelTexture,
};

#[derive(Event)]
pub struct CollisionAsteroidAsteroidEvent(pub Entity, pub Entity);

#[derive(Event)]
pub struct CollisionProjectileEvent {
    pub projectile_ent: Entity,
    pub other_ent: Entity,
}

#[derive(Event)]
pub struct CollisionPlayerShipEvent;

#[derive(Event)]
pub struct CollisionProjectileAsteroidEvent;

#[derive(Default)]
pub enum Avatars {
    PlayerShip,
    Asteroid,
    Projectile,
    #[default]
    Other,
}

pub fn update_collide_ship(
    mut commands: Commands,
    mut evr_collisions: EventReader<CollisionEvent>,
    q_ship: Query<&Transform, With<PlayerShipTag>>,
    particle_pixel_texture: Res<ParticlePixelTexture>,
    damage_ship_sound: Res<ShipDamagedSound>,
) {
    for event in evr_collisions.read() {
        match event {
            CollisionEvent::Started(ent_a, ent_b, _flags) => {
                if let Ok((transform)) = q_ship.get(*ent_a) {
                    emit_ship_collision_particles(
                        &mut commands,
                        &transform,
                        &particle_pixel_texture,
                    );
                    commands.spawn(AudioBundle {
                        source: damage_ship_sound.0.clone(),
                        settings: PlaybackSettings::DESPAWN,
                    });
                }

                if let Ok((transform)) = q_ship.get(*ent_b) {
                    emit_ship_collision_particles(
                        &mut commands,
                        &transform,
                        &particle_pixel_texture,
                    );
                    commands.spawn(AudioBundle {
                        source: damage_ship_sound.0.clone(),
                        settings: PlaybackSettings::DESPAWN,
                    });
                }
            }
            _ => {}
        }
    }
}

fn emit_ship_collision_particles(
    commands: &mut Commands,
    transform: &Transform,
    particle_pixel_texture: &ParticlePixelTexture,
) {
    commands
        .spawn(ParticleSystemBundle {
            particle_system: ParticleSystem {
                max_particles: 25,
                texture: particle_pixel_texture.0.clone().into(),
                spawn_rate_per_second: 0.0.into(),
                initial_speed: JitteredValue::jittered(175.0, -50.0..0.0),
                lifetime: JitteredValue::jittered(3.0, -0.5..0.0),
                color: ColorOverTime::Gradient(Curve::new(vec![
                    CurvePoint::new(Color::RED, 0.0),
                    CurvePoint::new(Color::BLACK, 0.5),
                    CurvePoint::new(Color::BLACK, 1.0),
                ])),
                looping: false,
                system_duration_seconds: 1.0,
                max_distance: Some(500.0),
                scale: 3.0.into(),
                // scale: ValueOverTime::Lerp(Lerp::new(2.0, 20.)),
                bursts: vec![ParticleBurst::new(0.0, 25)],
                ..ParticleSystem::default()
            },
            transform: Transform::from_xyz(transform.translation.x, transform.translation.y, 0.0),
            ..ParticleSystemBundle::default()
        })
        .insert(Playing);
}
