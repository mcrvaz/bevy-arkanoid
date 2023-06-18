use bevy::prelude::*;

#[derive(Copy, Clone)]
pub enum WallColor {
    Blue,
    Pink,
    Gold,
    Green,
    Orange,
    Red,
    White,
    Yellow,
}

impl WallColor {
    pub fn from_char(c: char) -> Option<WallColor> {
        let digit = c.to_digit(10);
        match digit {
            Some(1) => Some(WallColor::Blue),
            Some(2) => Some(WallColor::Pink),
            Some(3) => Some(WallColor::Gold),
            Some(4) => Some(WallColor::Green),
            Some(5) => Some(WallColor::Orange),
            Some(6) => Some(WallColor::Red),
            Some(7) => Some(WallColor::White),
            Some(8) => Some(WallColor::Yellow),
            _ => None,
        }
    }

    pub fn get_file_name(self) -> String {
        match self {
            WallColor::Blue => "wall_blue.png".to_string(),
            WallColor::Pink => "wall_pink.png".to_string(),
            WallColor::Gold => "wall_gold.png".to_string(),
            WallColor::Green => "wall_green.png".to_string(),
            WallColor::Orange => "wall_orange.png".to_string(),
            WallColor::Red => "wall_red.png".to_string(),
            WallColor::White => "wall_white.png".to_string(),
            WallColor::Yellow => "wall_yellow.png".to_string(),
        }
    }
}

#[derive(Copy, Clone)]
pub enum PowerUpColor {
    Blue,
    Cyan,
    Green,
    Gray,
    Orange,
    Pink,
    Red,
}

impl PowerUpColor {
    pub fn from_char(c: char) -> Option<PowerUpColor> {
        let digit = c.to_digit(10);
        match digit {
            Some(1) => Some(PowerUpColor::Blue),
            Some(2) => Some(PowerUpColor::Cyan),
            Some(3) => Some(PowerUpColor::Green),
            Some(4) => Some(PowerUpColor::Gray),
            Some(5) => Some(PowerUpColor::Orange),
            Some(6) => Some(PowerUpColor::Pink),
            Some(7) => Some(PowerUpColor::Red),
            _ => None,
        }
    }

    pub fn get_file_name(self) -> String {
        match self {
            PowerUpColor::Blue => "powerup_blue.png".to_string(),
            PowerUpColor::Cyan => "powerup_cyan.png".to_string(),
            PowerUpColor::Green => "powerup_green.png".to_string(),
            PowerUpColor::Gray => "powerup_gray.png".to_string(),
            PowerUpColor::Orange => "powerup_orange.png".to_string(),
            PowerUpColor::Pink => "powerup_pink.png".to_string(),
            PowerUpColor::Red => "powerup_red.png".to_string(),
        }
    }
}

#[derive(Component)]
pub struct Paddle {
    pub speed: f32,
}

#[derive(Component)]
pub struct Ball {}

#[derive(Component)]
pub struct Wall {
    pub score: i32,
    pub durability: i32,
    pub color: WallColor,
}

impl Wall {
    pub const fn get_size() -> Vec2 {
        // Vec2::new(16.0, 8.0) // original size
        Vec2::new(8.0, 8.0)
    }

    pub fn from_color(color: WallColor) -> Self {
        match color {
            WallColor::Blue => Wall {
                score: 60,
                durability: 1,
                color,
            },
            WallColor::Pink => Wall {
                score: 100,
                durability: 1,
                color,
            },
            WallColor::Gold => Wall {
                score: 0,
                durability: i32::MAX,
                color,
            },
            WallColor::Green => Wall {
                score: 70,
                durability: 1,
                color,
            },
            WallColor::Orange => Wall {
                score: 90,
                durability: 2,
                color,
            },
            WallColor::Red => Wall {
                score: 110,
                durability: 3,
                color,
            },
            WallColor::White => Wall {
                score: 50,
                durability: 1,
                color,
            },
            WallColor::Yellow => Wall {
                score: 80,
                durability: 1,
                color,
            },
        }
    }
}

#[derive(Default, Component)]
pub struct MatchData {
    pub score: i32,
    pub lives: i32,
    pub max_ball_count: i32,
    pub max_paddle_count: i32,
}

#[derive(Component)]
pub struct Bounds {}

impl Bounds {
    pub const fn get_size() -> Vec2 {
        Vec2::new(32.0, 7.0)
    }
}

#[derive(Component)]
pub struct Goal {}

#[derive(Component)]
pub struct PowerUp {
    pub color: PowerUpColor,
}
