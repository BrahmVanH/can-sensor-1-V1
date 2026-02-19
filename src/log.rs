use crate::prelude::*;

// Circular log buffer in RAM
const LOG_SIZE: usize = 64;
static mut LOG_BUFFER: [LogEntry; LOG_SIZE] = [LogEntry::empty(); LOG_SIZE];
static mut LOG_INDEX: usize = 0;

#[derive(Clone, Copy, Debug)]
struct LogEntry {
    timestamp_ms: u32,
    event: Event,
}

#[derive(Clone, Copy, Debug)]
pub enum Event {
    Empty,
    Startup,
    SensorTopStuck,
    SensorBottomStuck,
    BothSensorsStuck,
    AlarmTriggered,
    AlarmCleared,
    SensorReadInconsistent,
    Paused,
    Resumed,
}

impl LogEntry {
    const fn empty() -> Self {
        LogEntry {
            timestamp_ms: 0,
            event: Event::Empty,
        }
    }
}

pub static mut UPTIME_MS: u32 = 0;

pub fn log_event(event: Event) {
    unsafe {
        LOG_BUFFER[LOG_INDEX] = LogEntry {
            timestamp_ms: UPTIME_MS,
            event,
        };
        LOG_INDEX = (LOG_INDEX + 1) % LOG_SIZE;
    }
}

pub fn dump_logs(serial: &mut impl ufmt::uWrite) {
    uwriteln!(serial, "=== Event Log ===");

    unsafe {
        for i in 0..LOG_SIZE {
            let idx = (LOG_INDEX + i) % LOG_SIZE;
            let entry = LOG_BUFFER[idx];

            match entry.event {
                Event::Empty => {
                    continue;
                }
                Event::Startup => {
                    uwriteln!(serial, "[{}] Startup", entry.timestamp_ms);
                }
                Event::SensorTopStuck => {
                    uwriteln!(serial, "[{}] Top sensor stuck", entry.timestamp_ms);
                }
                Event::SensorBottomStuck => {
                    uwriteln!(serial, "[{}] Bottom sensor stuck", entry.timestamp_ms);
                }
                Event::AlarmTriggered => {
                    uwriteln!(serial, "[{}] ALARM", entry.timestamp_ms);
                }
                Event::AlarmCleared => {
                    uwriteln!(serial, "[{}] Alarm cleared", entry.timestamp_ms);
                }
                Event::Paused => {
                    uwriteln!(serial, "[{}] Paused", entry.timestamp_ms);
                }
                Event::Resumed => {
                    uwriteln!(serial, "[{}] Resumed", entry.timestamp_ms);
                }
                _ => {}
            }
        }
    }
    uwriteln!(serial, "=================");
}

// Health monitoring
