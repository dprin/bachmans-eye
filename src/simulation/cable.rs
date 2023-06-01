use bevy::prelude::*;

use super::connection::{Sender, Receiver};

#[derive(Component, Default)]
pub struct Cable {
    pub error_rate: f32,
}

impl Cable {
    pub fn new(error_rate: f32) -> Self {
        Self { error_rate }
    }
}

#[derive(Default, Bundle)]
pub struct CableBundle {
    pub cable: Cable,
    pub sender: Sender,
    pub receiver: Receiver,
}
