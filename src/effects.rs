use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_particle_systems::{
    CircleSegment, ColorOverTime, Curve, CurvePoint, JitteredValue, ParticleBurst, ParticleSystem,
    ParticleSystemBundle, Playing,
};
use bevy_rapier2d::dynamics::Velocity;

use crate::{
    audio::ProjectileImpactSound, avatars::Thrust, components::PlayerShipTag, events::Avatars,
    game::ParticlePixelTexture,
};

// Contains utility functions for producing effects: collision/death sounds and particles

#[derive(Event)]
pub struct CollisionEffectEvent {
    pub id: Entity,
    pub avatar: Avatars,
    pub transform: Option<Transform>,
    pub velocity: Option<Velocity>,
}

#[derive(Event)]
pub struct DestructionEffectEvent {
    pub translation: Vec2,
    pub avatar: Avatars,
}

pub fn handle_collision_effects(
    mut commands: Commands,
    mut evr_coll_effects: EventReader<CollisionEffectEvent>,
    particle_pixel_texture: Res<ParticlePixelTexture>,
    proj_coll_sound: Res<ProjectileImpactSound>,
) {
    for event in evr_coll_effects.read() {
        match event.avatar {
            Avatars::Projectile => {
                // play sound, emit particles
                emit_projectile_collision_particles(
                    &mut commands,
                    &particle_pixel_texture,
                    &event.transform.unwrap_or_default(),
                    &event.velocity.unwrap_or_default(),
                );
                commands.spawn(AudioBundle {
                    source: proj_coll_sound.0.clone(),
                    settings: PlaybackSettings::DESPAWN,
                });
            }
            _ => {}
        }
    }
}

fn emit_projectile_collision_particles(
    commands: &mut Commands,
    particle_pixel_texture: &Res<ParticlePixelTexture>,
    transform: &Transform,
    velocity: &Velocity,
) {
    let direction_angle = Vec2::from_angle(PI).rotate(velocity.linvel).to_angle();
    commands
        .spawn(ParticleSystemBundle {
            particle_system: ParticleSystem {
                max_particles: 6,
                texture: particle_pixel_texture.0.clone().into(),
                spawn_rate_per_second: 0.0.into(),
                // TODO scale with proj velocity
                initial_speed: JitteredValue::jittered(30.0, -10.0..0.0),
                lifetime: JitteredValue::jittered(0.5, -0.25..0.0),
                // color: ColorOverTime::Constant((Color::WHITE)),
                color: ColorOverTime::Gradient(Curve::new(vec![
                    CurvePoint::new(Color::WHITE, 0.0),
                    CurvePoint::new(Color::rgba(1., 1., 1., 0.5), 0.2),
                    CurvePoint::new(Color::rgba(1.0, 1.0, 1.0, 0.1), 1.0),
                ])),
                emitter_shape: CircleSegment {
                    radius: 0.0.into(),
                    opening_angle: std::f32::consts::PI / 2.0,
                    direction_angle,
                }
                .into(),
                looping: false,
                rotate_to_movement_direction: true,
                initial_rotation: (0_f32).to_radians().into(),
                system_duration_seconds: 0.25,
                max_distance: Some(100.0),
                scale: 1.0.into(),
                bursts: vec![ParticleBurst::new(0.0, 6)],
                ..ParticleSystem::default()
            },
            transform: Transform::from_xyz(transform.translation.x, transform.translation.y, 0.0),
            ..ParticleSystemBundle::default()
        })
        .insert(Playing);
}

pub fn handle_destruction_effects(
    mut commands: Commands,
    mut ev_w: EventReader<DestructionEffectEvent>,
) {
    for event in ev_w.read() {
        match event.avatar {
            Avatars::PlayerShip => {
                // death sound
                // death particles
            }
            Avatars::Asteroid => {}
            _ => {}
        }
    }
}

// TODO this is an effect
pub fn emit_thruster_particles(
    commands: &mut Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    q_ship_exists: Query<(), With<PlayerShipTag>>,
    mut q_ship_children: Query<&Children, With<PlayerShipTag>>,
    mut q_particle_system: Query<Entity, (With<Thrust>, With<ParticleSystem>)>,
) {
    if q_ship_exists.get_single().is_ok() {
        if keyboard_input.pressed(KeyCode::KeyS) {
            for children in q_ship_children.iter_mut() {
                for child in children {
                    if let Ok(ent_id) = q_particle_system.get_mut(*child) {
                        commands.entity(ent_id).insert(Playing);
                    }
                }
            }
        }
        if keyboard_input.just_released(KeyCode::KeyS) {
            for children in q_ship_children.iter_mut() {
                for child in children {
                    if let Ok(ent_id) = q_particle_system.get_mut(*child) {
                        commands.entity(ent_id).remove::<Playing>();
                    }
                }
            }
        }
    }
}