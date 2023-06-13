mod game_bundles;
mod game_components;
mod game_systems;
mod input;

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
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_systems((setup_cameras, setup_physics, setup_input))
        .add_startup_systems((setup_match, spawn_paddle, spawn_walls))
        .add_system(spawn_balls)
        .run();
}
