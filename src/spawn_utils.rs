#![allow(dead_code)]

use crate::game_bundles::*;
use crate::game_components::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::path::Path;
use std::path::PathBuf;

pub fn spawn_paddle(cmd: &mut Commands, position: Vec2, asset_sv: &Res<AssetServer>) -> Entity {
    let size = Vec2::new(32.0, 8.0);
    let sprite = SpriteBundle {
        texture: asset_sv.load("sprites/paddle.png"),
        transform: Transform::from_translation(position.extend(0.0)),
        sprite: Sprite {
            custom_size: Option::Some(size),
            ..default()
        },
        ..default()
    };
    cmd.spawn(PaddleBundle {
        paddle: Paddle { speed: 20.0 },
        sprite: sprite,
        physics: Physics {
            collider: Collider::cuboid(size.x / 2.0, size.y / 2.0),
            rb: RigidBody::KinematicVelocityBased,
            coll_events: ActiveEvents::COLLISION_EVENTS,
            locked_axes: LockedAxes::all(),
            velocity: Velocity::zero(),
            restitution: Restitution {
                coefficient: 1.0,
                combine_rule: CoefficientCombineRule::Max,
            },
            friction: Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            sleeping: Sleeping::disabled(),
        },
    })
    .id()
}

pub fn spawn_ball(
    cmd: &mut Commands,
    position: Vec2,
    velocity: Vec2,
    asset_sv: &Res<AssetServer>,
) -> Entity {
    let size = Vec2::new(5.0, 4.0);
    let center_pos = Vec2::new(position.x, position.y + (size.y / 2.0));
    let sprite = SpriteBundle {
        texture: asset_sv.load("sprites/ball.png"),
        transform: Transform::from_translation(center_pos.extend(0.0)),
        sprite: Sprite {
            custom_size: Option::Some(size),
            ..default()
        },
        ..default()
    };
    cmd.spawn(BallBundle {
        ball: Ball {},
        sprite: sprite,
        physics: Physics {
            collider: Collider::cuboid(size.x / 2.0, size.y / 2.0),
            rb: RigidBody::Dynamic,
            coll_events: ActiveEvents::COLLISION_EVENTS,
            locked_axes: LockedAxes::ROTATION_LOCKED,
            velocity: Velocity::linear(velocity),
            restitution: Restitution {
                coefficient: 1.0,
                combine_rule: CoefficientCombineRule::Max,
            },
            friction: Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            sleeping: Sleeping::disabled(),
        },
    })
    .insert(CollisionGroups::new(Group::GROUP_1, Group::ALL - Group::GROUP_1))
    .id()
}

pub fn spawn_wall(
    cmd: &mut Commands,
    wall: Wall,
    position: Vec2,
    asset_sv: &Res<AssetServer>,
) -> Entity {
    let size = Wall::get_size();
    let path = Path::new("sprites")
        .join("walls")
        .join(wall.color.get_file_name());
    let sprite = SpriteBundle {
        texture: asset_sv.load(path),
        sprite: Sprite {
            custom_size: Option::Some(size),
            ..default()
        },
        transform: Transform::from_translation(position.extend(0.0)),
        ..default()
    };
    cmd.spawn(WallBundle {
        wall,
        sprite,
        collider: Collider::cuboid(size.x / 2.0, size.y / 2.0),
    })
    .id()
}

pub fn spawn_bound(
    cmd: &mut Commands,
    position: Vec2,
    rotation: Quat,
    asset_sv: &Res<AssetServer>,
) -> Entity {
    let path = Path::new("sprites").join("bounds.png");
    spawn_generic_bound(cmd, path, position, rotation, asset_sv)
}

pub fn spawn_goal(
    cmd: &mut Commands,
    position: Vec2,
    rotation: Quat,
    asset_sv: &Res<AssetServer>,
) -> Entity {
    let path = Path::new("sprites").join("goal.png");
    let entity = spawn_generic_bound(cmd, path, position, rotation, asset_sv);
    cmd.entity(entity).insert(Goal {});
    entity
}

fn spawn_generic_bound(
    cmd: &mut Commands,
    path: PathBuf,
    position: Vec2,
    rotation: Quat,
    asset_sv: &Res<AssetServer>,
) -> Entity {
    let size = Bound::get_size();
    let sprite = SpriteBundle {
        texture: asset_sv.load(path),
        sprite: Sprite {
            custom_size: Option::Some(size),
            ..default()
        },
        transform: Transform {
            translation: position.extend(0.0),
            rotation,
            ..default()
        },
        ..default()
    };
    cmd.spawn(BoundsBundle {
        bounds: Bound {},
        sprite,
        collider: Collider::cuboid(size.x / 2.0, size.y / 2.0),
    })
    .id()
}
