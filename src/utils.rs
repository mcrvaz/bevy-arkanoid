#![allow(dead_code)]

use bevy::prelude::*;
use rand::prelude::*;

pub fn rand_sign() -> f32 {
    if random::<f32>() >= 0.5 {
        1.0
    } else {
        -1.0
    }
}

pub fn random_horizontal() -> Vec2 {
    Vec2::new(rand_sign(), 0.0)
}

pub fn approx_eq(a: f32, b: f32, margin: f32) -> bool {
    (a - b).abs() <= margin
}
