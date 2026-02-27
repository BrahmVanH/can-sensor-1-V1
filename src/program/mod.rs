pub mod legacy;

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

pub fn run_program() -> ! {
    let dp = get_peripherals();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    // current hardware configuration is led backlit doorbell switch - two terminal
    let mut toggle_input: Option<Pin<Input<PullUp>, _>> = Some(pins.d9.into_pull_up_input());
    let mut toggle_output: Option<Pin<Output, _>> = None;

    // current hardware configuration is two Comimark E18-D80NK
    let sensor_top = get_pin_input_device(pins.d4);
    let sensor_bottom = get_pin_input_device(pins.d6);

    // current hardware configuration is 4 12085 passive buzzer modules
    let mut buzzer = get_pin_output_device(pins.a4);

    health_check_beep(&mut buzzer);

    log_event(Event::Startup);

    let mut alarm_count: u8 = 0;

    const ALARM_THRESHOLD: u8 = 10;

    let mut health = SensorHealth::new();
    let mut loop_count: u32 = 0;

    let mut alarm_state = AlarmState::new();
    let mut paused = false;

    uwriteln!(&mut serial, "Commands: p=pause, r=resume, d=dump logs").ok();

    let mut running = false;
    let mut last_button_state = true;

    // doorbell button receives cycled power to blink backlight LED when running, solid on when paused
    let mut blink_counter: u16 = 0;

    const BLINK_HALF_PERIOD: u16 = 25;

    loop {
        if let Some(p) = toggle_output.take() {
            toggle_input = Some(p.into_pull_up_input());
        }
        let button_pressed = toggle_input.as_ref().map_or(false, |p| p.is_low());

        // button must be depressed for at least 25ms
        if button_pressed && !last_button_state {
            arduino_hal::delay_ms(25);

            let still_pressed = toggle_input.as_ref().map_or(false, |p| p.is_low());

            if still_pressed {
                running = !running;
                blink_counter = 0;
                if running {
                    log_event(Event::Resumed);
                } else {
                    log_event(Event::Paused);
                }
            }
        }

        last_button_state = button_pressed;

        if running {
            blink_counter = blink_counter.wrapping_add(1);
            if blink_counter >= BLINK_HALF_PERIOD * 2 {
                blink_counter = 0;
            }
            if blink_counter >= BLINK_HALF_PERIOD {
                if let Some(p) = toggle_input.take() {
                    let mut op = p.into_output();
                    let _ = op.set_low();
                    toggle_output = Some(op);
                }
            }
        }

        if !running {
            arduino_hal::delay_ms(25);
            continue;
        }

        // check for serial input
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
            health_check_double_beep(&mut buzzer);
            dump_logs(&mut serial);
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
                let _ = buzzer.set_low();
            }
        }

        if alarm_state.is_playing {
            // alarm_smoke_siren(&mut buzzer);
            alarm_passive_buzzer_sweet_spot(&mut buzzer);
        }

        loop_count += 1;
        if loop_count % 30000 == 0 {
            dump_logs(&mut serial);
        }
        arduino_hal::delay_ms(20);
    }
}
