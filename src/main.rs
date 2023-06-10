mod game_bundles;
mod game_components;
mod game_systems;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::game_systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Arkanoid!".to_string(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        // .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .insert_resource(ClearColor(Color::BLACK))
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_systems((setup_cameras, setup_physics))
        .add_startup_systems((spawn_paddle, spawn_walls))
        .add_system(bevy::window::close_on_esc)
        .add_system(spawn_balls)
        .run();
}
