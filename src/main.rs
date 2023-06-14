mod game_bundles;
mod game_components;
mod game_systems;
mod input;
mod spawn_utils;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::game_systems::*;

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
                GameSets::FindCollisions,
            )
                .chain(),
        )
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_systems((
            setup_cameras,
            setup_physics,
            setup_input,
            setup_match,
            spawn_walls,
        ))
        .add_systems(
            (
                spawn_paddles.run_if(can_spawn_paddles),
                apply_system_buffers,
                spawn_balls.run_if(can_spawn_balls),
            )
                .chain()
                .in_set(GameSets::Spawn),
        )
        .add_systems((move_paddles, move_balls).in_set(GameSets::ComputeForces))
        .run();
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
enum GameSets {
    Spawn,
    ComputeForces,
    FindCollisions,
}
