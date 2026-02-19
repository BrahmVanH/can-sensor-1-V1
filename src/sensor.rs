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
        const STUCK_THRESHOLD: u16 = 5000; // 100 seconds at 20ms loop

        if top && bottom {
            self.both_stuck_count = self.both_stuck_count.saturating_add(1);
            if self.both_stuck_count == STUCK_THRESHOLD {
                log_event(Event::BothSensorsStuck);
                return false; // Signal health issue
            }
        } else {
            self.both_stuck_count = 0;
        }

        // Individual sensor stuck checks
        if top {
            self.top_stuck_count = self.top_stuck_count.saturating_add(1);
            if self.top_stuck_count == STUCK_THRESHOLD {
                log_event(Event::SensorTopStuck);
                return false;
            }
        } else {
            self.top_stuck_count = 0;
        }

        if bottom {
            self.bottom_stuck_count = self.bottom_stuck_count.saturating_add(1);
            if self.bottom_stuck_count == STUCK_THRESHOLD {
                log_event(Event::SensorBottomStuck);
                return false;
            }
        } else {
            self.bottom_stuck_count = 0;
        }

        true // All healthy
    }
}
