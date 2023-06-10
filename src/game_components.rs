use bevy::prelude::*;

pub enum WallColor {
    Blue,
    Pink,
}

impl WallColor {
    pub fn from_char(c: char) -> Option<WallColor> {
        let digit = c.to_digit(10);
        match digit {
            Some(1) => Some(WallColor::Blue),
            Some(2) => Some(WallColor::Pink),
            _ => None,
        }
    }

    pub fn get_file_name(self) -> String {
        match self {
            WallColor::Blue => "blueWall.png".to_string(),
            WallColor::Pink => "pinkWall.png".to_string(),
        }
    }
}

#[derive(Component)]
pub struct Paddle {
    pub speed: f32,
}

#[derive(Component)]
pub struct Ball {
    pub speed: f32,
}

#[derive(Component)]
pub struct Wall {
    pub score: i32,
}

impl Wall {
    pub const fn get_size() -> Vec2 {
        // Vec2::new(16.0, 8.0)
        Vec2::new(8.0, 8.0)
    }
}

#[derive(Component)]
pub struct Bounds {}

#[derive(Component)]
pub struct Goal {}

#[derive(Component)]
pub struct PlayerData {
    pub score: i32,
    pub lives: i32,
}
