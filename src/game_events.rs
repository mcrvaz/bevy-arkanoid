use bevy::prelude::*;

pub struct GoalEvent {
    pub ball: Entity,
}

pub struct WallCollisionEvent {
    pub ball: Entity,
    pub wall: Entity,
}
