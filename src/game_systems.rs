use std::path::Path;

use crate::game_bundles::*;
use crate::game_components::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn setup_physics(mut physics: ResMut<RapierConfiguration>) {
    physics.gravity = Vec2::ZERO;
}

pub fn setup_cameras(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn spawn_paddle(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sprite = SpriteBundle {
        texture: asset_server.load("sprites/paddle.png"),
        transform: Transform::from_translation(Vec3::new(0.0, -100.0, 0.0)),
        ..default()
    };
    commands.spawn(PaddleBundle {
        paddle: Paddle { speed: 20.0 },
        sprite: sprite,
        physics: Physics {
            collider: Collider::cuboid(32.0 / 2.0, 8.0 / 2.0),
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
    asset_server: Res<AssetServer>,
    paddle_query: Query<(&Transform, &Sprite), With<Paddle>>,
) {
    let (player_transform, player_sprite) = paddle_query.single();
    spawn_ball(&mut commands, Vec2::ZERO, &asset_server);
}

fn spawn_ball(commands: &mut Commands, position: Vec2, asset_server: &Res<AssetServer>) {
    println!("spawn ball!");
}

pub fn spawn_walls(mut commands: Commands, asset_server: Res<AssetServer>) {
    let file = std::fs::read_to_string("assets/settings/walls.txt").expect("Map not found!");
    let line_count = file.lines().count();
    let column_count = file.lines().next().unwrap().len();
    let map_size = Vec2::new(column_count as f32, line_count as f32);
    let wall_size = Wall::get_size();
    let initial_pos = Vec2::new(-(map_size.x * wall_size.x) / 2.0, 100.0);
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
