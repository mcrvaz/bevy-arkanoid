use crate::game_components::*;
use crate::spawn_utils::*;
use bevy::prelude::*;

pub fn setup_map(mut cmd: Commands, asset_sv: Res<AssetServer>) {
    let file = std::fs::read_to_string("assets/settings/walls.txt").expect("Map not found!");
    let line_count = file.lines().count();
    let column_count = file.lines().next().unwrap().len();
    let map_size = Vec2::new(column_count as f32, line_count as f32);
    let y_offset = 100.0;
    spawn_walls(map_size, y_offset, file, &mut cmd, &asset_sv);
    spawn_bounds(map_size, y_offset, &mut cmd, &asset_sv);
}

fn spawn_walls(
    map_size: Vec2,
    y_offset: f32,
    file: String,
    cmd: &mut Commands,
    asset_sv: &Res<AssetServer>,
) {
    let wall_size = Wall::get_size();
    let initial_pos = Vec2::new(-(map_size.x * wall_size.x) / 2.0, y_offset);
    let mut pos = Vec2::new(initial_pos.x, initial_pos.y);
    for line in file.lines() {
        for c in line.chars() {
            if let Some(wall_color) = WallColor::from_char(c) {
                let wall = Wall::from_color(wall_color);
                spawn_wall(cmd, wall, pos, &asset_sv);
            }
            pos = Vec2::new(pos.x + wall_size.x, pos.y);
        }
        pos = Vec2::new(initial_pos.x, pos.y + wall_size.y);
    }
}

fn spawn_bounds(map_size: Vec2, y_offset: f32, cmd: &mut Commands, asset_sv: &Res<AssetServer>) {
    let wall_size = Wall::get_size();
    let bounds_size = Bound::get_size();
    let margin = wall_size;
    let map_size = (map_size * wall_size) + margin;

    let end_pos = spawn_upper_bounds(map_size, bounds_size, y_offset, cmd, asset_sv);
    spawn_left_bounds(map_size, bounds_size, y_offset, cmd, asset_sv);
    spawn_right_bounds(map_size, bounds_size, y_offset, end_pos, cmd, asset_sv);
    spawn_lower_bounds(map_size, bounds_size, cmd, asset_sv);
}

fn spawn_upper_bounds(
    map_size: Vec2,
    bounds_size: Vec2,
    y_offset: f32,
    cmd: &mut Commands,
    asset_sv: &Res<AssetServer>,
) -> Vec2 {
    let mut pos = Vec2::new(-map_size.x / 2.0, map_size.y + y_offset);
    while pos.x <= (map_size.x / 2.0) {
        spawn_bound(cmd, pos, Quat::IDENTITY, asset_sv);
        pos += Vec2::new(bounds_size.x, 0.0);
    }
    pos
}

fn spawn_left_bounds(
    map_size: Vec2,
    bounds_size: Vec2,
    y_offset: f32,
    cmd: &mut Commands<'_, '_>,
    asset_sv: &Res<'_, AssetServer>,
) -> Vec2 {
    let mut pos = Vec2::new(
        (-map_size.x / 2.0) - (bounds_size.x / 2.0),
        map_size.y + y_offset - (bounds_size.x / 2.0),
    );
    while pos.y > -(map_size.y + y_offset) {
        spawn_bound(
            cmd,
            pos,
            Quat::from_rotation_z(f32::to_radians(90.0)),
            asset_sv,
        );
        pos -= Vec2::new(0.0, bounds_size.x);
    }
    pos
}

fn spawn_right_bounds(
    map_size: Vec2,
    bounds_size: Vec2,
    y_offset: f32,
    final_pos: Vec2,
    cmd: &mut Commands<'_, '_>,
    asset_sv: &Res<'_, AssetServer>,
) -> Vec2 {
    let mut pos = Vec2::new(
        final_pos.x - (bounds_size.x / 2.0),
        map_size.y + y_offset - (bounds_size.x / 2.0),
    );
    while pos.y > -(map_size.y + y_offset) {
        spawn_bound(
            cmd,
            pos,
            Quat::from_rotation_z(f32::to_radians(270.0)),
            asset_sv,
        );
        pos -= Vec2::new(0.0, bounds_size.x);
    }
    pos
}

fn spawn_lower_bounds(
    map_size: Vec2,
    bounds_size: Vec2,
    cmd: &mut Commands,
    asset_sv: &Res<AssetServer>,
) -> Vec2 {
    let mut pos = Vec2::new(-map_size.x / 2.0, -map_size.y);
    while pos.x <= (map_size.x / 2.0) {
        spawn_goal(cmd, pos, Quat::IDENTITY, asset_sv);
        pos += Vec2::new(bounds_size.x, 0.0);
    }
    pos
}
