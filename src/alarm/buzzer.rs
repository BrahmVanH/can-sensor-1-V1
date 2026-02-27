use super::*;
pub fn find_resonance(buzzer: &mut impl OutputPin) {
    for freq in (2000u32..6000).step_by(200) {
        let half_period = 1_000_000u32 / freq / 2;
        for _ in 0..500 {
            let _ = buzzer.set_high();
            arduino_hal::delay_us(half_period);
            let _ = buzzer.set_low();
            arduino_hal::delay_us(half_period);
        }
        arduino_hal::delay_ms(200); // gap between tones
    }
}

pub fn alarm_siren(buzzer: &mut impl OutputPin) {
    for freq in (800..2000).step_by(100) {
        let period_us = 1_000_000 / freq / 2;
        for _ in 0..10 {
            let _ = buzzer.set_high();
            arduino_hal::delay_us(period_us as u32);
            let _ = buzzer.set_low();
            arduino_hal::delay_us(period_us as u32);
        }
    }
    for freq in (800..2000).step_by(100).rev() {
        let period_us = 1_000_000 / freq / 2;
        for _ in 0..10 {
            let _ = buzzer.set_high();
            arduino_hal::delay_us(period_us as u32);
            let _ = buzzer.set_low();
            arduino_hal::delay_us(period_us as u32);
        }
    }
}

pub fn alarm_passive_buzzer_sweet_spot(buzzer: &mut impl OutputPin) {
    let half_period: u32 = 1_000_000 / 2000 / 2; // 250us → 2000 Hz
    let cycles_per_blast: u32 = 2000 / 2; // 0.5s worth of cycles (1000)

    for _repeat in 0..2 {
        for _blast in 0..3 {
            for _ in 0..cycles_per_blast {
                let _ = buzzer.set_high();
                arduino_hal::delay_us(half_period);
                let _ = buzzer.set_low();
                arduino_hal::delay_us(half_period);
            }
            arduino_hal::delay_ms(500);
        }
        arduino_hal::delay_ms(1500);
    }
}

pub fn alarm_smoke_siren(buzzer: &mut impl OutputPin) {
    let half_period: u32 = 1_000_000 / 3400 / 2; // ~147us → 3400 Hz
    let cycles_per_blast: u32 = 3400 / 2; // 0.5s worth of cycles (1700)

    for _repeat in 0..2 {
        for _blast in 0..3 {
            for _ in 0..cycles_per_blast {
                let _ = buzzer.set_high();
                arduino_hal::delay_us(half_period);
                let _ = buzzer.set_low();
                arduino_hal::delay_us(half_period);
            }
            arduino_hal::delay_ms(500);
        }
        arduino_hal::delay_ms(1500);
    }
}

pub fn health_check_beep(buzzer: &mut impl OutputPin) {
    let half_period: u32 = 1_000_000 / 2200 / 2; // ~227us
    for _ in 0..220 {
        let _ = buzzer.set_high();
        arduino_hal::delay_us(half_period);
        let _ = buzzer.set_low();
        arduino_hal::delay_us(half_period);
    }
    arduino_hal::delay_ms(50);
}

pub fn health_check_double_beep(buzzer: &mut impl OutputPin) {
    let half_period: u32 = 1_000_000 / 2200 / 2; // ~227us
    for _ in 0..2 {
        for _ in 0..110 {
            let _ = buzzer.set_high();
            arduino_hal::delay_us(half_period);
            let _ = buzzer.set_low();
            arduino_hal::delay_us(half_period);
        }
        arduino_hal::delay_ms(100);
    }
}
