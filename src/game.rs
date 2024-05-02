use lazy_static::lazy_static;

use bevy::prelude::*;
use bevy::time::Stopwatch;
use bevy_rapier2d::prelude::*;

use bevy_particle_systems::{
    ColorOverTime, Curve, CurvePoint, EmitterShape, JitteredValue, ParticleSystem,
    ParticleSystemBundle, Playing,
};

use crate::archetypes::AsteroidSizes;
use crate::audio::{
    AsteroidClashSound, AsteroidDestroyedSound, ProjectileEmitSound, ProjectileImpactSound,
    ShipDamagedSound, ShipDestroyedSound, ShipThrustSound, ShipThrustSoundStopwatch,
};
use crate::avatars::{gen_asteroid, gen_playership_from_materialmesh};
use crate::components::{Score, ScoreboardUi};
use crate::physics::handle_collisions;
use crate::play::{despawn_delay, draw_boundary, play_plugin, update_scoreboard, wraparound};
use crate::utils::Heading;

// NEWTYPES
pub type Speed = f32;
pub type TurnSpeed = f32;

// CONSTANTS
pub const AMBIENT_LINEAR_FRICTION_COEFFICIENT: f32 = 0.6;
pub const AMBIENT_ANGULAR_FRICTION_COEFFICIENT: f32 = 1.0;

// play area dims assuming 1920x1080 window with 20% saved for debug and UI
pub const LEFT_WALL: f32 = -768.;
pub const RIGHT_WALL: f32 = 768.;
pub const BOTTOM_WALL: f32 = -432.;
pub const TOP_WALL: f32 = 432.;

// General
pub const DEFAULT_MOVESPEED: Speed = 100.;
pub const DEFAULT_HEALTH: i32 = 1;
pub const DEFAULT_PROJECTILE_EMISSION_COOLDOWN: i32 = 100;
lazy_static! {
    pub static ref DEFAULT_HEADING: Heading = Heading(90.);
    pub static ref DEFAULT_ROTATION: Quat = Quat::from_rotation_z(90.);
}
pub const DEFAULT_TURNRATE: f32 = 10.;
pub const DEFAULT_DAMAGE: i32 = 1;
pub const DEFAULT_DURATION_SECS: u64 = 5;
pub const DEFAULT_RESTITUTION: f32 = 0.5;
pub const DEFAULT_THRUST_FORCE_MAGNITUDE: f32 = 10000.;

// UI
pub const SCOREBOARD_FONT_SIZE: f32 = 20.0;
pub const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
pub const LABEL_COLOR: Color = Color::LIME_GREEN;
pub const SCORE_COLOR: Color = Color::LIME_GREEN;

// Ship
pub const INIT_SHIP_MOVE_SPEED: Speed = 300.;
pub const INIT_SHIP_TURN_RATE: TurnSpeed = 5.;
pub const INIT_SHIP_HEALTH: i32 = 3;
pub const INIT_SHIP_PROJECTILE_SPEED: f32 = 500.;
pub const INIT_SHIP_RESTITUTION: f32 = 1.7;
pub const SHIP_LENGTH_FORE: f32 = 18.;
pub const SHIP_LENGTH_AFT: f32 = 18.;
pub const SHIP_HALF_WIDTH: f32 = 10.;
// prod
// pub const SHIP_THRUST_FORCE_MAGNITUDE: f32 = 10000.;
// dev
pub const SHIP_THRUST_FORCE_MAGNITUDE: f32 = 50000.;

// Asteroid
pub const INIT_ASTEROID_MOVESPEED: Speed = 300.;
pub const INIT_ASTEROID_DAMAGE: i32 = 1;
pub const INIT_ASTEROID_RESTITUTION: f32 = 0.3;

pub const SMALL_ASTEROID_R: f32 = 15.;
pub const SMALL_ASTEROID_HEALTH: i32 = 1;

pub const MEDIUM_ASTEROID_R: f32 = 30.;
pub const MEDIUM_ASTEROID_HEALTH: i32 = 3;

pub const LARGE_ASTEROID_R: f32 = 50.;
pub const LARGE_ASTEROID_HEALTH: i32 = 5;

pub fn game_plugin(app: &mut App) {
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(2.))
        // .add_plugins(RapierDebugRenderPlugin::default())
        .insert_resource(Score(0))
        .init_state::<GameState>()
        .add_systems(Startup, (load_assets, setup_menu).chain())
        .add_plugins(play_plugin);
}

pub fn setup_menu(
    mut commands: Commands,
    mut game_state: ResMut<NextState<GameState>>, // mut meshes: ResMut<Assets<Mesh>>,
                                                  // mut materials: ResMut<Assets<ColorMaterial>>,
) {
    game_state.set(GameState::Play);
}

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.insert_resource(ShipThrustSoundStopwatch(Stopwatch::new()));

    // let background_music = asset_server.load("sounds/Windless Slopes.ogg");
    // commands.insert_resource(BackgroundMusic(background_music));

    let light_shot_sound = asset_server.load("sounds/light_shot.wav");
    commands.insert_resource(ProjectileEmitSound(light_shot_sound));

    let ship_thrust_sound = asset_server.load("sounds/thrust.wav");
    commands.insert_resource(ShipThrustSound(ship_thrust_sound));

    let projectile_impact_sound = asset_server.load("sounds/scratch.wav");
    commands.insert_resource(ProjectileImpactSound(projectile_impact_sound));

    let destroy_asteroid_sound = asset_server.load("sounds/destroy_asteroid.wav");
    commands.insert_resource(AsteroidDestroyedSound(destroy_asteroid_sound));

    // let moderate_thud_sound = asset_server.load("sounds/moderate_thud.wav");
    // commands.insert_resource(SomeThudSound(moderate_thud_sound));

    let damage_ship_sound = asset_server.load("sounds/damage_ship.wav");
    commands.insert_resource(ShipDamagedSound(damage_ship_sound));

    let destroy_ship_sound = asset_server.load("sounds/human_physical_death.wav");
    commands.insert_resource(ShipDestroyedSound(destroy_ship_sound));

    let asteroid_clash_sound = asset_server.load("sounds/asteroid_clash.wav");
    commands.insert_resource(AsteroidClashSound(asteroid_clash_sound));

    // let handle_playership_mesh = meshes.add(Triangle2d::new(
    //     Vec2::new(-SHIP_HALF_WIDTH, -SHIP_HALF_WIDTH),
    //     Vec2::Y * SHIP_LENGTH,
    //     Vec2::new(SHIP_HALF_WIDTH, -SHIP_HALF_WIDTH),
    // ));
    // commands.insert_resource(PlayerShipMeshHandle(handle_playership_mesh));

    // let handle_playership_colormaterial = materials.add(Color::LIME_GREEN);
    // commands.insert_resource(PlayerShipMaterialHandle(handle_playership_colormaterial));

    let handle_white_colormaterial = materials.add(Color::WHITE);
    commands.insert_resource(WhiteMaterialHandle(handle_white_colormaterial));

    let asteroid_material = materials.add(Color::GRAY);
    commands.insert_resource(AsteroidMaterialHandles(vec![asteroid_material]));

    let mut asteroid_mesh_handles = vec![];
    for n_sides in [5, 6, 8] {
        for r in [SMALL_ASTEROID_R, MEDIUM_ASTEROID_R, LARGE_ASTEROID_R] {
            let handle_mesh = meshes.add(RegularPolygon::new(r, n_sides));
            asteroid_mesh_handles.push(handle_mesh);
            // ???
        }
    }
    commands.insert_resource(AsteroidMeshHandles(asteroid_mesh_handles));

    let playership_texture = asset_server.load("ship_K.png").into();
    commands.insert_resource(PlayerShipTexture(playership_texture));

    let particle_pixel_texture = asset_server.load("px.png").into();
    commands.insert_resource(ParticlePixelTexture(particle_pixel_texture));

    let powerup_essential_texture = asset_server.load("enemy_A.png").into();
    commands.insert_resource(PowerupEssentialTexture(powerup_essential_texture));
    let powerup_simple_texture = asset_server.load("enemy_C.png").into();
    commands.insert_resource(PowerupSimpleTexture(powerup_simple_texture));
    let powerup_complex_texture = asset_server.load("enemy_E.png").into();
    commands.insert_resource(PowerupComplexTexture(powerup_complex_texture));

    let star_essential_texture = asset_server.load("star_06.png").into();
    commands.insert_resource(StarEssentialTexture(star_essential_texture));
    let star_simple_texture = asset_server.load("star_04.png").into();
    commands.insert_resource(StarSimpleTexture(star_simple_texture));
    let star_complex_texture = asset_server.load("star_08.png").into();
    commands.insert_resource(StarComplexTexture(star_complex_texture));

    let green_planet_texture = asset_server.load("planet00.png").into();
    commands.insert_resource(PlanetGreenTexture(green_planet_texture));
    let grey_planet_texture = asset_server.load("planet04.png").into();
    commands.insert_resource(PlanetGreyTexture(grey_planet_texture));
    let purple_planet_texture = asset_server.load("planet09.png").into();
    commands.insert_resource(PlanetPurpleTexture(purple_planet_texture));
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Menu,
    #[default]
    Play,
    End,
}

#[derive(Component)]
pub struct OnPlayScreen;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlaySet;

// Meshes

#[derive(Resource)]
pub struct AsteroidMeshHandles(pub Vec<Handle<Mesh>>);

#[derive(Resource)]
pub struct AsteroidMaterialHandles(pub Vec<Handle<ColorMaterial>>);

#[derive(Resource)]
pub struct PlayerShipMeshHandle(pub Handle<Mesh>);

#[derive(Resource)]
pub struct ParticleMeshHandle(pub Handle<Mesh>);

// Color materials
#[derive(Resource)]
pub struct PlayerShipMaterialHandle(pub Handle<ColorMaterial>);

#[derive(Resource)]
pub struct WhiteMaterialHandle(pub Handle<ColorMaterial>);

// Textures
#[derive(Resource)]
pub struct ParticlePixelTexture(pub Handle<Image>);

#[derive(Resource)]
pub struct PowerupEssentialTexture(pub Handle<Image>);

#[derive(Resource)]
pub struct PowerupSimpleTexture(pub Handle<Image>);

#[derive(Resource)]
pub struct PowerupComplexTexture(pub Handle<Image>);

#[derive(Resource)]
pub struct StarEssentialTexture(pub Handle<Image>);

#[derive(Resource)]
pub struct StarSimpleTexture(pub Handle<Image>);

#[derive(Resource)]
pub struct StarComplexTexture(pub Handle<Image>);

#[derive(Resource)]
pub struct PlayerShipTexture(pub Handle<Image>);

#[derive(Resource)]
pub struct PlanetGreenTexture(pub Handle<Image>);
#[derive(Resource)]
pub struct PlanetGreyTexture(pub Handle<Image>);
#[derive(Resource)]
pub struct PlanetPurpleTexture(pub Handle<Image>);

#[derive(Resource)]
pub struct Textures {
    pub particle_pixel: Handle<Image>,
    pub powerup_simple: Handle<Image>,
    pub powerup_basic: Handle<Image>,
    pub powerup_complex: Handle<Image>,
    pub star_simple: Handle<Image>,
    pub star_basic: Handle<Image>,
    pub star_complex: Handle<Image>,
}

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in to_despawn.iter() {
        // println!("despawning entity {:?}", entity);
        commands.entity(entity).despawn_recursive();
    }
}
