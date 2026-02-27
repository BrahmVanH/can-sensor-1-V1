use crate::prelude::*;

pub struct SensorHealth {
    top_stuck_count: u16,
    bottom_stuck_count: u16,
    both_stuck_count: u16,
}

impl SensorHealth {
    pub fn new() -> Self {
        SensorHealth {
            top_stuck_count: 0,
            bottom_stuck_count: 0,
            both_stuck_count: 0,
        }
    }

    pub fn check(&mut self, top: bool, bottom: bool) -> bool {
        const STUCK_THRESHOLD: u16 = 5000;

        if top && bottom {
            true;
        }

        if top && !bottom {
            self.top_stuck_count = self.top_stuck_count.saturating_add(1);
            if self.top_stuck_count == STUCK_THRESHOLD {
                log_event(Event::SensorTopStuck);
                return false;
            }
        } else {
            self.top_stuck_count = 0;
        }

        true
    }
}
