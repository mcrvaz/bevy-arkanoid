use std::path::Path;

use crate::game_bundles::*;
use crate::game_components::*;
use crate::input::*;
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

pub fn spawn_paddle(mut commands: Commands, asset_server: Res<AssetServer>) {
    let size = Vec2::new(32.0, 8.0);
    let sprite = SpriteBundle {
        texture: asset_server.load("sprites/paddle.png"),
        transform: Transform::from_translation(Vec3::new(0.0, -100.0, 0.0)),
        sprite: Sprite {
            custom_size: Option::Some(size),
            ..default()
        },
        ..default()
    };
    commands.spawn(PaddleBundle {
        paddle: Paddle { speed: 20.0 },
        sprite: sprite,
        physics: Physics {
            collider: Collider::cuboid(size.x / 2.0, size.y / 2.0),
            rb: RigidBody::KinematicVelocityBased,
            coll_events: ActiveEvents::COLLISION_EVENTS,
            locked_axes: LockedAxes::all(),
            velocity: Velocity::zero(),
            restitution: Restitution::coefficient(1.0),
            friction: Friction::coefficient(0.0),
        },
    });
}

pub fn spawn_balls(
    mut commands: Commands,
    mut match_data_query: Query<&mut MatchData>,
    paddle_query: Query<(&Transform, &Sprite), With<Paddle>>,
    asset_server: Res<AssetServer>,
) {
    let mut match_data = match_data_query.single_mut();
    if match_data.ball_count > 0 {
        return;
    }

    let (player_transform, player_sprite) = paddle_query.single();
    let player_pos = player_transform.translation.truncate();
    let sprite_size = player_sprite.custom_size.unwrap_or(Vec2::ZERO);
    let pos = Vec2::new(player_pos.x, player_pos.y + sprite_size.y);
    spawn_ball(&mut commands, pos, &asset_server);
    match_data.ball_count += 1;
}

fn spawn_ball(commands: &mut Commands, position: Vec2, asset_server: &Res<AssetServer>) {
    println!("ball spawn {}", position);
    let size = Vec2::new(5.0, 4.0);
    let center_pos = Vec2::new(position.x, position.y + (size.y / 2.0));
    let sprite = SpriteBundle {
        texture: asset_server.load("sprites/ball.png"),
        transform: Transform::from_translation(center_pos.extend(0.0)),
        sprite: Sprite {
            custom_size: Option::Some(size),
            ..default()
        },
        ..default()
    };
    commands.spawn(BallBundle {
        ball: Ball { speed: 0.0 },
        sprite: sprite,
        physics: Physics {
            collider: Collider::cuboid(size.x / 2.0, size.y / 2.0),
            rb: RigidBody::Dynamic,
            coll_events: ActiveEvents::COLLISION_EVENTS,
            locked_axes: LockedAxes::all(),
            velocity: Velocity::zero(),
            restitution: Restitution::coefficient(1.0),
            friction: Friction::coefficient(0.0),
        },
    });
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

fn spawn_wall(
    commands: &mut Commands,
    wall_color: WallColor,
    position: Vec2,
    asset_server: &Res<AssetServer>,
) -> Entity {
    let size = Wall::get_size();
    let path = Path::new("sprites")
        .join("walls")
        .join(wall_color.get_file_name());
    let sprite = SpriteBundle {
        texture: asset_server.load(path),
        sprite: Sprite {
            custom_size: Option::Some(Wall::get_size()),
            ..default()
        },
        transform: Transform::from_translation(position.extend(0.0)),
        ..default()
    };
    commands
        .spawn(WallBundle {
            sprite: sprite,
            physics: Physics {
                rb: RigidBody::Fixed,
                collider: Collider::cuboid(size.x / 2.0, size.y / 2.0),
                coll_events: ActiveEvents::COLLISION_EVENTS,
                locked_axes: LockedAxes::all(),
                velocity: Velocity::zero(),
                restitution: Restitution::coefficient(1.0),
                friction: Friction::coefficient(0.0),
            },
        })
        .id()
}
