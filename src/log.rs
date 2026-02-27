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
    uwriteln!(serial, "=== Event Log ===").ok();

    unsafe {
        for i in 0..LOG_SIZE {
            let idx = (LOG_INDEX + i) % LOG_SIZE;
            let entry = LOG_BUFFER[idx];

            match entry.event {
                Event::Empty => {
                    continue;
                }
                Event::Startup => {
                    uwriteln!(serial, "[{}] Startup", entry.timestamp_ms).ok();
                }
                Event::SensorTopStuck => {
                    uwriteln!(serial, "[{}] Top sensor stuck", entry.timestamp_ms).ok();
                }
                Event::SensorBottomStuck => {
                    uwriteln!(serial, "[{}] Bottom sensor stuck", entry.timestamp_ms).ok();
                }
                Event::AlarmTriggered => {
                    uwriteln!(serial, "[{}] ALARM", entry.timestamp_ms).ok();
                }
                Event::AlarmCleared => {
                    uwriteln!(serial, "[{}] Alarm cleared", entry.timestamp_ms).ok();
                }
                Event::Paused => {
                    uwriteln!(serial, "[{}] Paused", entry.timestamp_ms).ok();
                }
                Event::Resumed => {
                    uwriteln!(serial, "[{}] Resumed", entry.timestamp_ms).ok();
                }
                _ => {}
            }
        }
    }
    uwriteln!(serial, "=================").ok();
}

// EEPROM persistent logging

use avr_device::atmega328p::EEPROM;

const EEPROM_MAGIC: u8 = 0xab;
const EEPROM_MAGIC_ADDR: u16 = 0;
const EEPROM_PTR_LO_ADDR: u16 = 1;
const EEPROM_PTR_HI_ADDR: u16 = 2;
const EEPROM_DATA_START: u16 = 3;
const EEPROM_ENTRY_SIZE: u16 = 5; // 1 byte event + 4 bytes uptime_ms
const EEPROM_MAX_ENTRIES: u16 = (1024 - 3) / 5; // 204 entries

impl Event {
    pub fn to_u8(self) -> u8 {
        match self {
            Event::Empty => 0,
            Event::Startup => 1,
            Event::SensorTopStuck => 2,
            Event::SensorBottomStuck => 3,
            Event::BothSensorsStuck => 4,
            Event::AlarmTriggered => 5,
            Event::AlarmCleared => 6,
            Event::SensorReadInconsistent => 7,
            Event::Paused => 8,
            Event::Resumed => 9,
        }
    }

    pub fn from_u8(v: u8) -> Self {
        match v {
            1 => Event::Startup,
            2 => Event::SensorTopStuck,
            3 => Event::SensorBottomStuck,
            4 => Event::BothSensorsStuck,
            5 => Event::AlarmTriggered,
            6 => Event::AlarmCleared,
            7 => Event::SensorReadInconsistent,
            8 => Event::Paused,
            9 => Event::Resumed,
            _ => Event::Empty,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Event::Empty => "Empty",
            Event::Startup => "Startup",
            Event::SensorTopStuck => "Top sensor stuck",
            Event::SensorBottomStuck => "Bottom sensor stuck",
            Event::BothSensorsStuck => "Both sensors stuck",
            Event::AlarmTriggered => "ALARM",
            Event::AlarmCleared => "Alarm cleared",
            Event::SensorReadInconsistent => "Sensor inconsistent",
            Event::Paused => "Paused",
            Event::Resumed => "Resumed",
        }
    }
}

fn ee_wait(eeprom: &EEPROM) {
    while eeprom.eecr().read().eepe().bit_is_set() {}
}

fn ee_read(eeprom: &EEPROM, addr: u16) -> u8 {
    ee_wait(eeprom);
    eeprom.eear().write(|w| unsafe { w.bits(addr) });
    eeprom.eecr().modify(|_, w| w.eere().set_bit());
    eeprom.eedr().read().bits()
}

fn ee_write(eeprom: &EEPROM, addr: u16, data: u8) {
    ee_wait(eeprom);
    eeprom.eear().write(|w| unsafe { w.bits(addr) });
    eeprom.eedr().write(|w| unsafe { w.bits(data) });
    eeprom.eecr().modify(|_, w| w.eempe().set_bit());
    eeprom.eecr().modify(|_, w| w.eepe().set_bit());
}

fn ee_read_ptr(eeprom: &EEPROM) -> u16 {
    let lo = ee_read(eeprom, EEPROM_PTR_LO_ADDR) as u16;
    let hi = ee_read(eeprom, EEPROM_PTR_HI_ADDR) as u16;
    (hi << 8) | lo
}

fn ee_write_ptr(eeprom: &EEPROM, ptr: u16) {
    ee_write(eeprom, EEPROM_PTR_LO_ADDR, (ptr & 0xff) as u8);
    ee_write(eeprom, EEPROM_PTR_HI_ADDR, (ptr >> 8) as u8);
}

pub fn eeprom_init(eeprom: &EEPROM) {
    if ee_read(eeprom, EEPROM_MAGIC_ADDR) != EEPROM_MAGIC {
        ee_write(eeprom, EEPROM_MAGIC_ADDR, EEPROM_MAGIC);
        ee_write_ptr(eeprom, 0);
    }
}

pub fn eeprom_log_event(eeprom: &EEPROM, event: Event) {
    let ptr = ee_read_ptr(eeprom);
    if ptr >= EEPROM_MAX_ENTRIES {
        return;
    }
    let base = EEPROM_DATA_START + ptr * EEPROM_ENTRY_SIZE;
    let uptime = unsafe { UPTIME_MS };
    ee_write(eeprom, base, event.to_u8());
    ee_write(eeprom, base + 1, (uptime & 0xff) as u8);
    ee_write(eeprom, base + 2, ((uptime >> 8) & 0xff) as u8);
    ee_write(eeprom, base + 3, ((uptime >> 16) & 0xff) as u8);
    ee_write(eeprom, base + 4, ((uptime >> 24) & 0xff) as u8);
    ee_write_ptr(eeprom, ptr + 1);
}

/// Dump all EEPROM log entries to serial. Send 'e' command to trigger.
pub fn eeprom_dump_logs(eeprom: &EEPROM, serial: &mut impl ufmt::uWrite) {
    let count = ee_read_ptr(eeprom).min(EEPROM_MAX_ENTRIES);
    uwriteln!(serial, "=== EEPROM Log ({} entries) ===", count).ok();
    for i in 0..count {
        let base = EEPROM_DATA_START + i * EEPROM_ENTRY_SIZE;
        let event = Event::from_u8(ee_read(eeprom, base));
        let t =
            (ee_read(eeprom, base + 1) as u32) |
            ((ee_read(eeprom, base + 2) as u32) << 8) |
            ((ee_read(eeprom, base + 3) as u32) << 16) |
            ((ee_read(eeprom, base + 4) as u32) << 24);
        uwriteln!(serial, "[{}] {} ({}ms)", i, event.name(), t).ok();
    }
    uwriteln!(serial, "================================").ok();
}

/// Reset the EEPROM log pointer. Send 'c' command to trigger.
/// Does not erase data — just resets the write head so old entries get overwritten.
pub fn eeprom_clear_logs(eeprom: &EEPROM) {
    ee_write_ptr(eeprom, 0);
}
