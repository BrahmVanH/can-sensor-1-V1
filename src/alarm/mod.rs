use crate::prelude::*;

mod buzzer;

pub use buzzer::*;
pub struct AlarmState {
    pub is_playing: bool,
    pub phase: u32,
    pub step: u32,
}

impl AlarmState {
    pub fn new() -> Self {
        AlarmState {
            is_playing: false,
            phase: 0,
            step: 0,
        }
    }

    pub fn reset(&mut self) {
        self.is_playing = false;
        self.phase = 0;
        self.step = 0;
    }
}
