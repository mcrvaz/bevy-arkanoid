use crate::game_components::*;
use crate::game_events::*;
use crate::input::*;
use crate::spawn_utils::*;
use crate::utils::*;
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
    let bundle = Camera2dBundle::default();
    commands.spawn(bundle);
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
    let pos = Vec2::new(player_pos.x, player_pos.y + sprite_size.y + 10.0);
    let speed = 150.0;
    let vel = Vec2::new(rand_sign(), 1.0) * speed;
    spawn_ball(&mut commands, pos, vel, &asset_server);
}

pub fn can_spawn_balls(ball_query: Query<&Ball>, match_data_query: Query<&MatchData>) -> bool {
    let ball_count = ball_query.iter().count() as i32;
    let match_data = match_data_query.single();
    return ball_count <= match_data.max_ball_count;
}

pub fn setup_map(commands: Commands, asset_server: Res<AssetServer>) {
    let file = std::fs::read_to_string("assets/settings/walls.txt").expect("Map not found!");
    let line_count = file.lines().count();
    let column_count = file.lines().next().unwrap().len();
    let map_size = Vec2::new(column_count as f32, line_count as f32);
    spawn_walls(map_size, file, commands, asset_server);
}

fn spawn_walls(
    map_size: Vec2,
    file: String,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let wall_size = Wall::get_size();
    let y_offset = 100.0;
    let initial_pos = Vec2::new(-(map_size.x * wall_size.x) / 2.0, y_offset);
    let mut pos = Vec2::new(initial_pos.x, initial_pos.y);
    for line in file.lines() {
        for c in line.chars() {
            if let Some(wall_color) = WallColor::from_char(c) {
                let wall = Wall::from_color(wall_color);
                spawn_wall(&mut commands, wall, pos, &asset_server);
            }
            pos = Vec2::new(pos.x + wall_size.x, pos.y);
        }
        pos = Vec2::new(initial_pos.x, pos.y + wall_size.y);
    }
}

fn spawn_bounds(map_size: Vec2, mut commands: Commands, asset_server: Res<AssetServer>) {
    let bounds_size = Bounds::get_size();
    let initial_pos = Vec2::new(-(map_size.x * bounds_size.x) / 2.0, map_size.y);
}

pub fn move_paddles() {}

pub fn evaluate_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    mut wall_collision_ev: EventWriter<WallCollisionEvent>,
    mut goal_ev: EventWriter<GoalEvent>,
    ball_query: Query<Entity, With<Ball>>,
    paddle_query: Query<Entity, With<Paddle>>,
    wall_query: Query<Entity, With<Wall>>,
    goal_query: Query<Entity, With<Goal>>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(_, _, _) => {}
            CollisionEvent::Stopped(e1, e2, _) => {
                evaluate_ball_collision(
                    e1,
                    e2,
                    &ball_query,
                    &paddle_query,
                    &wall_query,
                    &goal_query,
                    &mut wall_collision_ev,
                    &mut goal_ev,
                );
            }
        }
    }
}

fn evaluate_ball_collision(
    e1: &Entity,
    e2: &Entity,
    ball_query: &Query<Entity, With<Ball>>,
    paddle_query: &Query<Entity, With<Paddle>>,
    wall_query: &Query<Entity, With<Wall>>,
    goal_query: &Query<Entity, With<Goal>>,
    wall_collision_ev: &mut EventWriter<WallCollisionEvent>,
    goal_ev: &mut EventWriter<GoalEvent>,
) {
    if let Ok(ball) = ball_query.get(*e1).or(ball_query.get(*e2)) {
        let other = if ball == *e1 { *e2 } else { *e1 };
        if wall_query.get(other).is_ok() {
            println!("wall collision");
            wall_collision_ev.send(WallCollisionEvent {
                ball: ball,
                wall: other,
            });
        } else if goal_query.get(other).is_ok() {
            println!("goal collision");
            goal_ev.send(GoalEvent { ball: ball });
        } else if paddle_query.get(other).is_ok() {
            println!("paddle collision");
        }
    }
}

pub fn handle_wall_collision(
    mut commands: Commands,
    mut wall_collision_ev: EventReader<WallCollisionEvent>,
) {
    for ev in wall_collision_ev.iter() {
        commands.entity(ev.wall).despawn();
    }
}
