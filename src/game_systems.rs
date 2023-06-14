use crate::game_components::*;
use crate::input::*;
use crate::spawn_utils::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::collections::{HashMap, HashSet};

pub fn setup_input(mut commands: Commands) {
    let horizontal_axis = InputAxis {
        axis_id: crate::input::Axis::Horizontal,
        positive_key_codes: HashSet::from([KeyCode::Right, KeyCode::D]),
        negative_key_codes: HashSet::from([KeyCode::Left, KeyCode::A]),
        val: 0.0,
    };

    let axes = InputAxes {
        val: HashMap::from([(horizontal_axis.axis_id, horizontal_axis)]),
    };

    commands.spawn(axes);
}

pub fn setup_physics(mut physics: ResMut<RapierConfiguration>) {
    physics.gravity = Vec2::ZERO;
}

pub fn setup_cameras(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn setup_match(mut commands: Commands) {
    commands.spawn(MatchData {
        lives: 3,
        ..default()
    });
}

pub fn spawn_paddles(mut commands: Commands, asset_server: Res<AssetServer>) {
    let pos = Vec2::new(0.0, -100.0);
    spawn_paddle(&mut commands, pos, &asset_server);
}

pub fn can_spawn_paddles(
    paddle_query: Query<&Paddle>,
    match_data_query: Query<&MatchData>,
) -> bool {
    let paddle_count = paddle_query.iter().count() as i32;
    let match_data = match_data_query.single();
    return paddle_count <= match_data.max_paddle_count;
}

pub fn spawn_balls(
    mut commands: Commands,
    paddle_query: Query<(&Transform, &Sprite), With<Paddle>>,
    asset_server: Res<AssetServer>,
) {
    let (player_transform, player_sprite) = paddle_query.single();
    let player_pos = player_transform.translation.truncate();
    let sprite_size = player_sprite.custom_size.unwrap_or(Vec2::ZERO);
    let pos = Vec2::new(player_pos.x, player_pos.y + sprite_size.y);
    spawn_ball(&mut commands, pos, &asset_server);
}

pub fn can_spawn_balls(ball_query: Query<&Ball>, match_data_query: Query<&MatchData>) -> bool {
    let ball_count = ball_query.iter().count() as i32;
    let match_data = match_data_query.single();
    return ball_count <= match_data.max_ball_count;
}

pub fn spawn_walls(mut commands: Commands, asset_server: Res<AssetServer>) {
    let file = std::fs::read_to_string("assets/settings/walls.txt").expect("Map not found!");
    let line_count = file.lines().count();
    let column_count = file.lines().next().unwrap().len();
    let map_size = Vec2::new(column_count as f32, line_count as f32);
    let wall_size = Wall::get_size();
    let y_offset = 100.0;
    let initial_pos = Vec2::new(-(map_size.x * wall_size.x) / 2.0, y_offset);
    let mut pos = Vec2::new(initial_pos.x, initial_pos.y);
    for line in file.lines() {
        for c in line.chars() {
            if let Some(w) = WallColor::from_char(c) {
                spawn_wall(&mut commands, w, pos, &asset_server);
            }
            pos = Vec2::new(pos.x + wall_size.x, pos.y);
        }
        pos = Vec2::new(initial_pos.x, pos.y + wall_size.y);
    }
}

pub fn move_paddles() {}

pub fn move_balls() {}
