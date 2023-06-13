#![allow(dead_code)]

use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

pub struct GameInput;
impl Plugin for GameInput {
    fn build(&self, app: &mut App) {
        app.add_system(gather_input.in_base_set(CoreSet::PreUpdate))
            .add_system(bevy::window::close_on_esc.in_base_set(CoreSet::PreUpdate));
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum Axis {
    Vertical = 0,
    Horizontal = 1,
}

#[derive(Component)]
pub struct InputAxes {
    pub val: HashMap<Axis, InputAxis>,
}

pub struct InputAxis {
    pub val: f32,
    pub axis_id: Axis,
    pub positive_key_codes: HashSet<KeyCode>,
    pub negative_key_codes: HashSet<KeyCode>,
}

impl InputAxis {
    fn set_val(&mut self, v: f32) {
        self.val = v.clamp(-1.0, 1.0);
    }
}

fn gather_input(
    keyboard_input: Res<bevy::input::Input<KeyCode>>,
    mut input_axes: Query<&mut InputAxes>,
) {
    for mut axes in input_axes.iter_mut() {
        for axis in axes.val.values_mut() {
            let positive = axis
                .positive_key_codes
                .iter()
                .any(|k| keyboard_input.pressed(*k)) as i32 as f32;
            let negative = axis
                .negative_key_codes
                .iter()
                .any(|k| keyboard_input.pressed(*k)) as i32 as f32;

            axis.set_val(positive - negative);
        }
    }
}
