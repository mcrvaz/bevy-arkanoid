use crate::game_components::*;
use crate::game_events::*;
use crate::input::*;
use crate::spawn_utils::*;
use crate::ui_spawn_utils::spawn_hud;
use crate::utils::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::collections::{HashMap, HashSet};

pub fn setup_input(mut cmd: Commands) {
    let horizontal_axis = InputAxis {
        axis_id: crate::input::Axis::Horizontal,
        positive_key_codes: HashSet::from([KeyCode::Right, KeyCode::D]),
        negative_key_codes: HashSet::from([KeyCode::Left, KeyCode::A]),
        val: 0.0,
    };

    let axes = InputAxes {
        val: HashMap::from([(horizontal_axis.axis_id, horizontal_axis)]),
    };

    cmd.spawn(axes);
}

pub fn setup_physics(mut physics: ResMut<RapierConfiguration>) {
    physics.gravity = Vec2::ZERO;
}

pub fn setup_cameras(mut cmd: Commands) {
    let bundle = Camera2dBundle::default();
    cmd.spawn(bundle);
}

pub fn setup_match(mut cmd: Commands) {
    cmd.spawn(MatchData {
        lives: 3,
        max_ball_count: i32::pow(2, 10),
        ..default()
    });
}

pub fn spawn_ui(cmd: Commands, asset_sv: Res<AssetServer>) {
    spawn_hud(cmd, asset_sv)
}

pub fn spawn_paddles(mut cmd: Commands, asset_sv: Res<AssetServer>) {
    let pos = Vec2::new(0.0, -100.0);
    spawn_paddle(&mut cmd, pos, &asset_sv);
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
    mut cmd: Commands,
    paddle_query: Query<(&Transform, &Sprite), With<Paddle>>,
    asset_sv: Res<AssetServer>,
) {
    let (player_transform, player_sprite) = paddle_query.single();
    let player_pos = player_transform.translation.truncate();
    let sprite_size = player_sprite.custom_size.unwrap_or(Vec2::ZERO);
    let pos = Vec2::new(player_pos.x, player_pos.y + sprite_size.y + 10.0);
    let speed = 150.0;
    let vel = Vec2::new(rand_sign(), 1.0) * speed;
    spawn_ball(&mut cmd, pos, vel, &asset_sv);
}

pub fn can_spawn_balls(ball_query: Query<&Ball>, match_data_query: Query<&MatchData>) -> bool {
    let ball_count = ball_query.iter().count() as i32;
    let match_data = match_data_query.single();
    return ball_count <= match_data.max_ball_count;
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
            // println!("wall collision");
            wall_collision_ev.send(WallCollisionEvent {
                ball: ball,
                wall: other,
            });
        } else if goal_query.get(other).is_ok() {
            // println!("goal collision");
            goal_ev.send(GoalEvent { ball: ball });
        } else if paddle_query.get(other).is_ok() {
            // println!("paddle collision");
        }
    }
}

pub fn handle_wall_collision(
    mut cmd: Commands,
    mut wall_collision_ev: EventReader<WallCollisionEvent>,
) {
    for ev in wall_collision_ev.iter() {
        cmd.entity(ev.wall).despawn();
    }
}
