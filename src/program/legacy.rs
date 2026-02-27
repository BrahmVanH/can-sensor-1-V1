use crate::{
    alarm::*,
    hardware::{ get_peripherals, get_pin_input_device, get_pin_output_device },
    log::{ Event, UPTIME_MS, dump_logs, log_event },
    prelude::*,
    sensor::SensorHealth,
};

// Sensor top - pin d4 - solid orange sensor wire
// Sensor bottom - pin d6 - orange stripe sensor wire
// Toggle button - pin d9 (also drives LED)
// Solid brown 24awg wire - power to sensors
// Solid blue 24awg wire - ground to sensors
// Alarm out - a4 - to 12v powered board

pub fn run_program_legacy() -> ! {
    let dp = get_peripherals();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let sensor_top = get_pin_input_device(pins.d4);
    let sensor_bottom = get_pin_input_device(pins.d6);

    let mut buzzer_1 = get_pin_output_device(pins.a3);
    let mut buzzer_2 = get_pin_output_device(pins.a0);
    let mut buzzer_3 = get_pin_output_device(pins.a1);
    let mut buzzer_4 = get_pin_output_device(pins.a5);
    let mut buzzer_6 = get_pin_output_device(pins.d3);
    let mut buzzer_7 = get_pin_output_device(pins.a4);

    health_check_beep(&mut buzzer_1);
    log_event(Event::Startup);

    let mut alarm_count: u8 = 0;
    const ALARM_THRESHOLD: u8 = 10;
    let mut health = SensorHealth::new();
    let mut loop_count: u32 = 0;

    let mut alarm_state = AlarmState::new();
    let mut paused = false;

    uwriteln!(&mut serial, "Commands: p=pause, r=resume, d=dump logs").ok();

    loop {
        if let Ok(byte) = serial.read() {
            uwriteln!(&mut serial, "rx: {}", byte).ok();
            match byte {
                b'p' | b'P' => {
                    if !paused {
                        paused = true;
                        log_event(Event::Paused);
                        dump_logs(&mut serial);
                        uwriteln!(&mut serial, "--- PAUSED (send 'r' to resume) ---").ok();
                    }
                }
                b'r' | b'R' => {
                    if paused {
                        paused = false;
                        log_event(Event::Resumed);
                        uwriteln!(&mut serial, "--- RESUMED ---").ok();
                    }
                }
                b'd' | b'D' => {
                    dump_logs(&mut serial);
                }
                _ => {}
            }
        }

        if paused {
            arduino_hal::delay_ms(50);
            continue;
        }

        unsafe {
            UPTIME_MS = UPTIME_MS.wrapping_add(5);
        }
        let top = sensor_top.is_low();
        let bottom = sensor_bottom.is_low();

        let sensors_healthy = health.check(top, bottom);

        if !sensors_healthy {
            health_check_beep(&mut buzzer_1);
            dump_logs(&mut serial);
        } else {
        }

        if bottom && !top {
            alarm_count = alarm_count.saturating_add(1);

            if alarm_count >= ALARM_THRESHOLD && !alarm_state.is_playing {
                log_event(Event::AlarmTriggered);
                alarm_state.is_playing = true;
                alarm_state.phase = 0;
                alarm_state.step = 0;
            }
        } else {
            alarm_count = 0;
            if alarm_state.is_playing {
                log_event(Event::AlarmCleared);
                alarm_state.reset();
                let _ = buzzer_1.set_low();
                let _ = buzzer_2.set_low();
                let _ = buzzer_3.set_low();
                let _ = buzzer_4.set_low();
                let _ = buzzer_6.set_low();
                let _ = buzzer_7.set_low();
            }
        }

        if alarm_state.is_playing {
            alarm_passive_buzzer_sweet_spot(&mut buzzer_7);
        }

        loop_count += 1;
        if loop_count % 30000 == 0 {
            dump_logs(&mut serial);
        }
        arduino_hal::delay_ms(20);
    }
}
