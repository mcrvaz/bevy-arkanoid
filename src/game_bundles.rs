use super::game_components::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Bundle)]
pub struct PaddleBundle {
    pub paddle: Paddle,
    pub sprite: SpriteBundle,
    pub physics: Physics,
}

#[derive(Bundle)]
pub struct BallBundle {
    pub ball: Ball,
    pub sprite: SpriteBundle,
    pub physics: Physics,
}

#[derive(Bundle)]
pub struct BoundsBundle {
    pub bounds: Bounds,
    pub sprite: SpriteBundle,
    pub collider: Collider,
}

#[derive(Bundle)]
pub struct Physics {
    pub rb: RigidBody,
    pub collider: Collider,
    pub coll_events: ActiveEvents,
    pub locked_axes: LockedAxes,
    pub velocity: Velocity,
    pub restitution: Restitution,
    pub friction: Friction,
}

#[derive(Bundle)]
pub struct WallBundle {
    pub sprite: SpriteBundle,
    pub physics: Physics,
}
