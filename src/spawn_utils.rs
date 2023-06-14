#![allow(dead_code)]

use crate::game_bundles::*;
use crate::game_components::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::path::Path;

pub fn spawn_paddle(
    commands: &mut Commands,
    position: Vec2,
    asset_server: &Res<AssetServer>,
) -> Entity {
    let size = Vec2::new(32.0, 8.0);
    let sprite = SpriteBundle {
        texture: asset_server.load("sprites/paddle.png"),
        transform: Transform::from_translation(position.extend(0.0)),
        sprite: Sprite {
            custom_size: Option::Some(size),
            ..default()
        },
        ..default()
    };
    commands
        .spawn(PaddleBundle {
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
        })
        .id()
}

pub fn spawn_ball(
    commands: &mut Commands,
    position: Vec2,
    asset_server: &Res<AssetServer>,
) -> Entity {
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
    commands
        .spawn(BallBundle {
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
        })
        .id()
}

pub fn spawn_wall(
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
