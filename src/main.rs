#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Sensors on D4 and D6
    let sensor_top = pins.d4.into_pull_up_input();
    let sensor_bottom = pins.d6.into_pull_up_input();

    // Buzzer on A3
    let mut buzzer = pins.a3.into_output();

    let mut alarm_count: u8 = 0;
    const ALARM_THRESHOLD: u8 = 10;

    loop {
        let top = sensor_top.is_low();
        let bottom = sensor_bottom.is_low();

        if bottom && !top {
            alarm_count = alarm_count.saturating_add(1);

            if alarm_count >= ALARM_THRESHOLD {
                for _ in 0..40 {
                    buzzer.set_high();
                    arduino_hal::delay_us(250);
                    buzzer.set_low();
                    arduino_hal::delay_us(250);
                }
                buzzer.set_high();
            }
        } else {
            alarm_count = 0;
            buzzer.set_low();
        }

        arduino_hal::delay_ms(20);
    }
}
