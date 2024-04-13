use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // mut next_state: ResMut<NextState<GameState>>,
) {
    commands.spawn(Camera2dBundle::default());

    // let wall_collision_sound = asset_server.load("sounds/breakout_collision.ogg");
    // let paddle_collision_sound = asset_server.load("sounds/med_shoot.wav");
    // let goal_collision_sound = asset_server.load("sounds/jump.wav");
    // commands.insert_resource(CollisionSound {
    //     wall: wall_collision_sound,
    //     paddle: paddle_collision_sound,
    //     goal: goal_collision_sound,
    // });
    // commands.insert_resource(Scores { a: 0, b: 0 });
    // commands.insert_resource(MatchInfo {
    //     round_count: 0,
    //     rounds_total: ROUNDS_TOTAL,
    // });
    // commands.insert_resource(RoundData {
    //     paddle_hit_count: 0,
    // });

    // next_state.set(GameState::Menu);
}
