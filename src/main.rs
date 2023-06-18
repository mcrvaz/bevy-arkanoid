mod game_bundles;
mod game_components;
mod game_events;
mod game_systems;
mod input;
mod spawn_utils;
mod utils;
mod map_spawn_systems;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::game_events::*;
use crate::game_systems::*;
use crate::map_spawn_systems::*;

fn main() {
    let plugins = DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "Arkanoid!".to_string(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        })
        .set(ImagePlugin::default_nearest());

    App::new()
        .add_plugins(plugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(input::GameInput)
        .insert_resource(ClearColor(Color::BLACK))
        .configure_sets(
            (
                GameSets::Spawn,
                GameSets::ComputeForces,
                GameSets::EvaluateCollisions,
                GameSets::HandleEvents
            )
                .chain(),
        )
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_systems((
            setup_cameras,
            setup_physics,
            setup_input,
            setup_match,
            setup_map,
        ))
        .add_event::<GoalEvent>()
        .add_event::<WallCollisionEvent>()
        .add_systems(
            (
                spawn_paddles.run_if(can_spawn_paddles),
                apply_system_buffers,
                spawn_balls.run_if(can_spawn_balls),
            )
                .chain()
                .in_set(GameSets::Spawn),
        )
        .add_system(move_paddles.in_set(GameSets::ComputeForces))
        .add_system(evaluate_collisions.in_set(GameSets::EvaluateCollisions))
        .add_system(handle_wall_collision.in_set(GameSets::HandleEvents))
        .run();
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
enum GameSets {
    Spawn,
    ComputeForces,
    EvaluateCollisions,
    HandleEvents
}
