use super::*;

pub fn play_alarm_chunk(
    b1: &mut impl OutputPin,
    b2: &mut impl OutputPin,
    b3: &mut impl OutputPin,
    b4: &mut impl OutputPin,
    b5: &mut impl OutputPin,
    b6: &mut impl OutputPin,
    b7: &mut impl OutputPin,
    state: &mut AlarmState
) {
    match state.phase {
        0 => {
            if state.step < 20 {
                let base = 1200 + state.step * 200;
                play_7voice(
                    b1,
                    base,
                    b2,
                    base + 75,
                    b3,
                    base + 150,
                    b4,
                    base + 220,
                    b5,
                    base + 50,
                    b6,
                    base + 300,
                    b7,
                    base + 120,
                    20
                );
                state.step += 1;
            } else {
                state.phase += 1;
                state.step = 0;
            }
        }

        1 => {
            if state.step < 15 {
                let base = (5000u32).saturating_sub(state.step * 250);
                play_7voice(
                    b1,
                    base,
                    b2,
                    base.saturating_sub(100),
                    b3,
                    base + 180,
                    b4,
                    base + 280,
                    b5,
                    base.saturating_sub(50),
                    b6,
                    base + 200,
                    b7,
                    base + 150,
                    18
                );
                state.step += 1;
            } else {
                state.phase += 1;
                state.step = 0;
            }
        }

        2 => {
            if state.step < 8 {
                play_7voice(
                    b1,
                    2200,
                    b2,
                    2500,
                    b3,
                    3100,
                    b4,
                    3700,
                    b5,
                    1800,
                    b6,
                    4200,
                    b7,
                    2800,
                    15
                );
                play_7voice(b1, 0, b2, 0, b3, 0, b4, 0, b5, 0, b6, 0, b7, 0, 8);
                state.step += 1;
            } else {
                state.phase += 1;
                state.step = 0;
            }
        }

        3 => {
            if state.step < 12 {
                let spread = state.step * 50;
                play_7voice(
                    b1,
                    1500 + spread,
                    b2,
                    2000 + spread,
                    b3,
                    2800 + spread,
                    b4,
                    4000 + spread,
                    b5,
                    1200 + spread,
                    b6,
                    4500 + spread,
                    b7,
                    3200 + spread,
                    25
                );
                state.step += 1;
            } else {
                state.phase += 1;
                state.step = 0;
            }
        }

        4 => {
            if state.step < 6 {
                play_7voice(
                    b1,
                    1400,
                    b2,
                    1600,
                    b3,
                    2000,
                    b4,
                    2300,
                    b5,
                    1100,
                    b6,
                    2700,
                    b7,
                    1900,
                    20
                );
                play_7voice(
                    b1,
                    3800,
                    b2,
                    4200,
                    b3,
                    4700,
                    b4,
                    5000,
                    b5,
                    3500,
                    b6,
                    4900,
                    b7,
                    4400,
                    20
                );
                state.step += 1;
            } else {
                state.phase += 1;
                state.step = 0;
            }
        }

        5 => {
            if state.step < 30 {
                let segment = state.step / 10;
                let inner_step = state.step % 10;
                let duration = (30u32).saturating_sub(segment * 8);
                let base = 2000 + inner_step * 300;
                play_7voice(
                    b1,
                    base,
                    b2,
                    base + 100,
                    b3,
                    base + 200,
                    b4,
                    base + 350,
                    b5,
                    base + 50,
                    b6,
                    base + 400,
                    b7,
                    base + 200,
                    duration
                );
                state.step += 1;
            } else {
                state.phase += 1;
                state.step = 0;
            }
        }

        6 => {
            if state.step < 10 {
                let offset = state.step * 17;
                play_7voice(
                    b1,
                    1700 + offset,
                    b2,
                    2100 + offset,
                    b3,
                    3000 + offset,
                    b4,
                    4100 + offset,
                    b5,
                    1300 + offset,
                    b6,
                    4600 + offset,
                    b7,
                    2800 + offset,
                    100
                );
                state.step += 1;
            } else {
                state.phase = 0;
                state.step = 0;
            }
        }

        _ => {
            state.phase = 0;
            state.step = 0;
        }
    }
}

pub fn alarm_rising_sweep(buzzer: &mut impl OutputPin) {
    for freq in (500..2500).step_by(50) {
        let period_us = 1_000_000 / freq / 2;
        for _ in 0..5 {
            buzzer.set_high();
            arduino_hal::delay_us(period_us as u32);
            buzzer.set_low();
            arduino_hal::delay_us(period_us as u32);
        }
    }
}

pub fn alarm_falling_sweep(buzzer: &mut impl OutputPin) {
    for freq in (500..2500).step_by(50).rev() {
        let period_us = 1_000_000 / freq / 2;
        for _ in 0..5 {
            buzzer.set_high();
            arduino_hal::delay_us(period_us as u32);
            buzzer.set_low();
            arduino_hal::delay_us(period_us as u32);
        }
    }
}

pub fn alarm_monkey(buzzer: &mut impl OutputPin) {
    for _ in 0..3 {
        for freq in (550u32..700).step_by(15) {
            let half_period = 1_000_000u32 / freq / 2;
            for _ in 0..3 {
                buzzer.set_high();
                arduino_hal::delay_us(half_period);
                buzzer.set_low();
                arduino_hal::delay_us(half_period);
            }
        }
        arduino_hal::delay_ms(90);
    }

    arduino_hal::delay_ms(60);

    for i in 0u8..5 {
        let base: u32 = 1200 + (i as u32) * 150;
        let top: u32 = base + 800;
        for freq in (base..top).step_by(80) {
            let half_period = 1_000_000u32 / freq / 2;
            for _ in 0..2 {
                buzzer.set_high();
                arduino_hal::delay_us(half_period);
                buzzer.set_low();
                arduino_hal::delay_us(half_period);
            }
        }
        let gap = (70u32).saturating_sub((i as u32) * 10);
        arduino_hal::delay_ms(gap);
    }

    arduino_hal::delay_ms(40);

    for _ in 0..8 {
        for _ in 0..6 {
            buzzer.set_high();
            arduino_hal::delay_us(200);
            buzzer.set_low();
            arduino_hal::delay_us(200);
        }
        for _ in 0..4 {
            buzzer.set_high();
            arduino_hal::delay_us(450);
            buzzer.set_low();
            arduino_hal::delay_us(450);
        }
    }

    arduino_hal::delay_ms(30);

    for freq in (600u32..3500).step_by(30) {
        let half_period = 1_000_000u32 / freq / 2;
        for _ in 0..2 {
            buzzer.set_high();
            arduino_hal::delay_us(half_period);
            buzzer.set_low();
            arduino_hal::delay_us(half_period);
        }
    }

    arduino_hal::delay_ms(150);
}

pub fn alarm_low_to_mid(buzzer: &mut impl OutputPin) {
    let mut freq: u32 = 20;
    while freq <= 10_000 {
        let half_period = 1_000_000u32 / freq / 2;

        let duration_us: u32 = 18_000;
        let cycle_period = half_period * 2;
        let cycles = if cycle_period > 0 { (duration_us / cycle_period).max(1) } else { 1 };

        for _ in 0..cycles {
            buzzer.set_high();
            arduino_hal::delay_us(half_period);
            buzzer.set_low();
            arduino_hal::delay_us(half_period);
        }

        if freq < 100 {
            freq += 5;
        } else if freq < 500 {
            freq += 20;
        } else if freq < 2000 {
            freq += 50;
        } else {
            freq += 200;
        }
    }

    arduino_hal::delay_ms(100);
}

pub fn alarm_siren(buzzer: &mut impl OutputPin) {
    for freq in (800..2000).step_by(100) {
        let period_us = 1_000_000 / freq / 2;
        for _ in 0..10 {
            buzzer.set_high();
            arduino_hal::delay_us(period_us as u32);
            buzzer.set_low();
            arduino_hal::delay_us(period_us as u32);
        }
    }
    for freq in (800..2000).step_by(100).rev() {
        let period_us = 1_000_000 / freq / 2;
        for _ in 0..10 {
            buzzer.set_high();
            arduino_hal::delay_us(period_us as u32);
            buzzer.set_low();
            arduino_hal::delay_us(period_us as u32);
        }
    }
}

pub fn play_7voice(
    b1: &mut impl OutputPin,
    f1: u32,
    b2: &mut impl OutputPin,
    f2: u32,
    b3: &mut impl OutputPin,
    f3: u32,
    b4: &mut impl OutputPin,
    f4: u32,
    b5: &mut impl OutputPin,
    f5: u32,
    b6: &mut impl OutputPin,
    f6: u32,
    b7: &mut impl OutputPin,
    f7: u32,
    duration_ms: u32
) {
    const TICK: u32 = 10;
    let total = duration_ms * 100;

    let hp = |f: u32| -> u32 {
        if f == 0 { u32::MAX } else { 500_000 / f }
    };
    let (h1, h2, h3, h4, h5, h6, h7) = (hp(f1), hp(f2), hp(f3), hp(f4), hp(f5), hp(f6), hp(f7));
    let (mut a1, mut a2, mut a3, mut a4, mut a5, mut a6, mut a7) = (0u32, 0, 0, 0, 0, 0, 0);
    let (mut s1, mut s2, mut s3, mut s4, mut s5, mut s6, mut s7) = (
        false,
        false,
        false,
        false,
        false,
        false,
        false,
    );

    for _ in 0..total {
        a1 += TICK;
        if a1 >= h1 {
            a1 -= h1;
            s1 = !s1;
            if s1 {
                b1.set_high();
            } else {
                b1.set_low();
            }
        }
        a2 += TICK;
        if a2 >= h2 {
            a2 -= h2;
            s2 = !s2;
            if s2 {
                b2.set_high();
            } else {
                b2.set_low();
            }
        }
        a3 += TICK;
        if a3 >= h3 {
            a3 -= h3;
            s3 = !s3;
            if s3 {
                b3.set_high();
            } else {
                b3.set_low();
            }
        }
        a4 += TICK;
        if a4 >= h4 {
            a4 -= h4;
            s4 = !s4;
            if s4 {
                b4.set_high();
            } else {
                b4.set_low();
            }
        }
        a5 += TICK;
        if a5 >= h5 {
            a5 -= h5;
            s5 = !s5;
            if s5 {
                b5.set_high();
            } else {
                b5.set_low();
            }
        }
        a6 += TICK;
        if a6 >= h6 {
            a6 -= h6;
            s6 = !s6;
            if s6 {
                b6.set_high();
            } else {
                b6.set_low();
            }
        }
        a7 += TICK;
        if a7 >= h7 {
            a7 -= h7;
            s7 = !s7;
            if s7 {
                b7.set_high();
            } else {
                b7.set_low();
            }
        }

        arduino_hal::delay_us(TICK);
    }

    b1.set_low();
    b2.set_low();
    b3.set_low();
    b4.set_low();
    b5.set_low();
    b6.set_low();
    b7.set_low();
}

pub fn alarm_monkey_chorus(
    b1: &mut impl OutputPin,
    b2: &mut impl OutputPin,
    b3: &mut impl OutputPin,
    b4: &mut impl OutputPin,
    b5: &mut impl OutputPin,
    b6: &mut impl OutputPin,
    b7: &mut impl OutputPin
) {
    for hoot in 0u32..3 {
        let f = 300 + hoot * 30;
        for step in 0u32..5 {
            let ff = f + step * 8;
            play_7voice(b1, ff * 2, b2, ff * 3, b3, ff * 4, b4, ff * 5, b5, 0, b6, 0, b7, ff, 20);
        }
        arduino_hal::delay_ms(120);
    }

    arduino_hal::delay_ms(80);

    for phrase in 0u32..2 {
        let base = 350 + phrase * 120;

        play_7voice(b1, base, b2, base + 40, b3, 0, b4, 0, b5, 0, b6, 0, b7, 0, 70);

        for step in 0u32..7 {
            let f = base + step * 180;
            let v = step + 1;
            play_7voice(
                b1,
                f,
                b2,
                if v >= 2 {
                    f + 25
                } else {
                    0
                },
                b3,
                if v >= 3 {
                    f * 2
                } else {
                    0
                },
                b4,
                if v >= 4 {
                    f * 2 + 40
                } else {
                    0
                },
                b5,
                if v >= 5 {
                    f * 3
                } else {
                    0
                },
                b6,
                if v >= 6 {
                    f * 3 + 60
                } else {
                    0
                },
                b7,
                if v >= 7 {
                    f + 10
                } else {
                    0
                },
                16
            );
        }
        arduino_hal::delay_ms(80);
    }

    arduino_hal::delay_ms(40);

    for round in 0u32..3 {
        let dur = (20u32).saturating_sub(round * 4);
        let gap = (25u32).saturating_sub(round * 8);

        play_7voice(b1, 1400, b2, 1800, b3, 0, b4, 0, b5, 0, b6, 0, b7, 0, dur);
        arduino_hal::delay_ms(gap);
        play_7voice(b1, 0, b2, 0, b3, 2200, b4, 1600, b5, 0, b6, 0, b7, 0, dur);
        arduino_hal::delay_ms(gap);
        play_7voice(b1, 0, b2, 0, b3, 0, b4, 0, b5, 1200, b6, 2500, b7, 0, dur);
        arduino_hal::delay_ms(gap);
        play_7voice(b1, 1400, b2, 1800, b3, 2200, b4, 1600, b5, 1200, b6, 2500, b7, 2000, dur);
        arduino_hal::delay_ms(gap);
    }

    arduino_hal::delay_ms(30);

    for step in 0u32..15 {
        let center = 1800 + step * 120;
        play_7voice(
            b1,
            center.saturating_sub(60),
            b2,
            center.saturating_sub(40),
            b3,
            center.saturating_sub(20),
            b4,
            center + 20,
            b5,
            center + 40,
            b6,
            center + 60,
            b7,
            center,
            25
        );
    }

    arduino_hal::delay_ms(50);

    for cycle in 0u32..3 {
        let speed = (12u32).saturating_sub(cycle * 2);
        for s in 0u32..8 {
            let f_up = 600 + s * 200;
            let f_hold = 1500 + cycle * 200;
            play_7voice(
                b1,
                f_up,
                b2,
                f_up + 30,
                b3,
                f_hold,
                b4,
                f_hold + 40,
                b5,
                (3000u32).saturating_sub(s * 150),
                b6,
                (3050u32).saturating_sub(s * 150),
                b7,
                f_up + 15,
                speed
            );
        }
        play_7voice(b1, 2800, b2, 2850, b3, 2900, b4, 2950, b5, 3000, b6, 3050, b7, 2900, 30);
        arduino_hal::delay_ms(40);
    }

    for step in 0u32..10 {
        let f = (2500u32).saturating_sub(step * 200);
        play_7voice(
            b1,
            f,
            b2,
            f + 20,
            b3,
            f / 2,
            b4,
            f / 2 + 15,
            b5,
            f / 3,
            b6,
            f / 3 + 10,
            b7,
            f / 2,
            20
        );
    }

    arduino_hal::delay_ms(200);
}
pub fn alarm_nightscape(
    b1: &mut impl OutputPin,
    b2: &mut impl OutputPin,
    b3: &mut impl OutputPin,
    b4: &mut impl OutputPin,
    b5: &mut impl OutputPin,
    b6: &mut impl OutputPin,
    b7: &mut impl OutputPin
) {
    for _ in 0..1 {
        play_7voice(b1, 4000, b2, 0, b3, 0, b4, 0, b5, 0, b6, 0, b7, 0, 25);
    }
    arduino_hal::delay_ms(500);

    for step in 0u32..15 {
        let peep_f = 2800 + step * 60;
        play_7voice(b1, 0, b2, 0, b3, 0, b4, peep_f, b5, 0, b6, 0, b7, 0, 30);
    }
    arduino_hal::delay_ms(800);

    for _ in 0..2 {
        play_7voice(b1, 0, b2, 4300, b3, 0, b4, 0, b5, 0, b6, 0, b7, 0, 28);
        arduino_hal::delay_ms(400);
    }
    arduino_hal::delay_ms(700);

    for step in 0u32..12 {
        let peep_f = 3100 + step * 50;
        play_7voice(b1, 0, b2, 0, b3, 0, b4, 0, b5, peep_f, b6, 0, b7, 0, 35);
    }
    arduino_hal::delay_ms(1000);

    play_7voice(b1, 4000, b2, 0, b3, 0, b4, 0, b5, 0, b6, 0, b7, 0, 26);
    arduino_hal::delay_ms(900);

    for step in 0u32..10 {
        let peep_f = 2500 + step * 80;
        play_7voice(b1, 0, b2, 0, b3, 0, b4, 0, b5, 0, b6, peep_f, b7, 0, 40);
    }
    arduino_hal::delay_ms(1200);

    for step in 0u32..10 {
        let pa = 2850 + step * 65;
        let pb = 3150 + step * 55;
        play_7voice(b1, 0, b2, 0, b3, 0, b4, pa, b5, pb, b6, 0, b7, 0, 32);
    }
    arduino_hal::delay_ms(1000);

    for _ in 0..2 {
        play_7voice(b1, 0, b2, 0, b3, 3800, b4, 0, b5, 0, b6, 0, b7, 0, 27);
        arduino_hal::delay_ms(450);
    }
    arduino_hal::delay_ms(800);

    for step in 0u32..14 {
        let peep_f = 2900 + step * 55;
        play_7voice(b1, 0, b2, 0, b3, 0, b4, peep_f, b5, 0, b6, 0, b7, 0, 35);
    }
    arduino_hal::delay_ms(1100);

    for step in 0u32..8 {
        let pb = 3200 + step * 50;
        play_7voice(b1, 4000, b2, 0, b3, 0, b4, 0, b5, pb, b6, 0, b7, 0, 30);
    }
    arduino_hal::delay_ms(900);

    for step in 0u32..9 {
        let peep_f = 2550 + step * 75;
        play_7voice(b1, 0, b2, 0, b3, 0, b4, 0, b5, 0, b6, peep_f, b7, 0, 38);
    }
    arduino_hal::delay_ms(1300);

    for step in 0u32..11 {
        let pa = 2800 + step * 55;
        let pb = 3050 + step * 60;
        let pc = 2600 + step * 70;
        play_7voice(b1, 0, b2, 0, b3, 0, b4, pa, b5, pb, b6, pc, b7, 0, 33);
    }
    arduino_hal::delay_ms(1200);

    play_7voice(b1, 0, b2, 4300, b3, 0, b4, 0, b5, 0, b6, 0, b7, 0, 25);
    arduino_hal::delay_ms(1100);

    for step in 0u32..8 {
        let peep_f = 2750 + step * 50;
        play_7voice(b1, 0, b2, 0, b3, 0, b4, peep_f, b5, 0, b6, 0, b7, 0, 32);
    }

    arduino_hal::delay_ms(500);
}
pub fn alarm_thunderstorm(
    b1: &mut impl OutputPin,
    b2: &mut impl OutputPin,
    b3: &mut impl OutputPin,
    b4: &mut impl OutputPin,
    b5: &mut impl OutputPin,
    b6: &mut impl OutputPin,
    b7: &mut impl OutputPin
) {
    for _storm in 0u32..2 {
        let rain_freqs: [u32; 7] = [3200, 4100, 3700, 4500, 3400, 4800, 3900];
        for i in 0u32..14 {
            let rf1 = rain_freqs[(i as usize) % 7];
            let rf2 = rain_freqs[((i as usize) + 3) % 7];
            play_7voice(b1, rf1, b2, rf2, b3, 0, b4, 0, b5, 0, b6, 0, b7, 0, 8);
            let gap = 15 + (i % 3) * 10;
            arduino_hal::delay_ms(gap);
        }

        for step in 0u32..12 {
            let wind = 400 + step * 60;
            let wind2 = wind + 30;
            let rf1 = rain_freqs[(step as usize) % 7];
            let rf2 = rain_freqs[((step as usize) + 4) % 7];
            play_7voice(b1, rf1, b2, rf2, b3, wind, b4, wind2, b5, 0, b6, 0, b7, 0, 20);
        }

        play_7voice(b1, 3000, b2, 4500, b3, 2000, b4, 5000, b5, 1500, b6, 3500, b7, 4000, 15);
        for step in 0u32..6 {
            let base = (4500u32).saturating_sub(step * 600);
            play_7voice(
                b1,
                base,
                b2,
                base + 200,
                b3,
                base + 500,
                b4,
                base + 800,
                b5,
                base / 2,
                b6,
                base / 2 + 100,
                b7,
                base + 100,
                8
            );
        }

        arduino_hal::delay_ms(30);

        for step in 0u32..20 {
            let rumble = if step % 2 == 0 { 120 } else { 140 };
            let harm1 = rumble * 2 + (step % 3) * 10;
            let harm2 = rumble * 3 + (step % 4) * 8;
            let rf1 = rain_freqs[(step as usize) % 7];
            let rf2 = rain_freqs[((step as usize) + 2) % 7];
            play_7voice(b1, rf1, b2, rf2, b3, 0, b4, 0, b5, harm1, b6, harm2, b7, rumble, 25);
        }

        for step in 0u32..8 {
            let rumble = (100u32).saturating_sub(step * 5);
            let dur = (20u32).saturating_sub(step * 2);
            if rumble > 0 {
                play_7voice(
                    b1,
                    0,
                    b2,
                    0,
                    b3,
                    0,
                    b4,
                    0,
                    b5,
                    if step < 5 {
                        rumble * 2
                    } else {
                        0
                    },
                    b6,
                    if step < 3 {
                        rumble * 3
                    } else {
                        0
                    },
                    b7,
                    if step < 7 {
                        rumble
                    } else {
                        0
                    },
                    dur
                );
            }
            arduino_hal::delay_ms(40 + step * 15);
        }

        for i in 0u32..8 {
            let rf1 = rain_freqs[(i as usize) % 7];
            play_7voice(b1, rf1, b2, 0, b3, 0, b4, 0, b5, 0, b6, 0, b7, 0, 6);
            arduino_hal::delay_ms(30 + i * 8);
        }

        arduino_hal::delay_ms(200);
    }
}

pub fn alarm_ultimate_chaos(
    b1: &mut impl OutputPin,
    b2: &mut impl OutputPin,
    b3: &mut impl OutputPin,
    b4: &mut impl OutputPin,
    b5: &mut impl OutputPin,
    b6: &mut impl OutputPin,
    b7: &mut impl OutputPin
) {
    for step in 0u32..20 {
        let base = 1200 + step * 200;
        play_7voice(
            b1,
            base,
            b2,
            base + 75,
            b3,
            base + 150,
            b4,
            base + 220,
            b5,
            base + 50,
            b6,
            base + 300,
            b7,
            base + 120,
            20
        );
    }

    for step in 0u32..15 {
        let base = (5000u32).saturating_sub(step * 250);
        play_7voice(
            b1,
            base,
            b2,
            base.saturating_sub(100),
            b3,
            base + 180,
            b4,
            base + 280,
            b5,
            base.saturating_sub(50),
            b6,
            base + 200,
            b7,
            base + 150,
            18
        );
    }

    for _beat in 0u32..8 {
        play_7voice(b1, 2200, b2, 2500, b3, 3100, b4, 3700, b5, 1800, b6, 4200, b7, 2800, 15);
        play_7voice(b1, 0, b2, 0, b3, 0, b4, 0, b5, 0, b6, 0, b7, 0, 8);
    }

    for step in 0u32..12 {
        let spread = step * 50;
        play_7voice(
            b1,
            (1500u32).saturating_add(spread),
            b2,
            (2000u32).saturating_add(spread),
            b3,
            (2800u32).saturating_add(spread),
            b4,
            (4000u32).saturating_add(spread),
            b5,
            (1200u32).saturating_add(spread),
            b6,
            (4500u32).saturating_add(spread),
            b7,
            (3200u32).saturating_add(spread),
            25
        );
    }

    for _cycle in 0u32..6 {
        play_7voice(b1, 1400, b2, 1600, b3, 2000, b4, 2300, b5, 1100, b6, 2700, b7, 1900, 20);
        play_7voice(b1, 3800, b2, 4200, b3, 4700, b4, 5000, b5, 3500, b6, 4900, b7, 4400, 20);
    }

    for (segment, _) in (0u32..3).enumerate() {
        let duration = (30u32).saturating_sub((segment as u32) * 8);
        for step in 0u32..10 {
            let base = 2000 + step * 300;
            play_7voice(
                b1,
                base,
                b2,
                base + 100,
                b3,
                base + 200,
                b4,
                base + 350,
                b5,
                base + 50,
                b6,
                base + 400,
                b7,
                base + 200,
                duration
            );
        }
    }

    for iteration in 0u32..10 {
        let offset = iteration * 17;
        play_7voice(
            b1,
            (1700u32).saturating_add(offset),
            b2,
            (2100u32).saturating_add(offset),
            b3,
            (3000u32).saturating_add(offset),
            b4,
            (4100u32).saturating_add(offset),
            b5,
            (1300u32).saturating_add(offset),
            b6,
            (4600u32).saturating_add(offset),
            b7,
            (2800u32).saturating_add(offset),
            100
        );
    }
}

pub fn health_check_beep(buzzer: &mut impl OutputPin) {
    // Single chirp: quick rising high-pitched tone - ~100ms total
    for freq in (3500u32..5500).step_by(100) {
        let half_period = 1_000_000u32 / freq / 2;
        for _ in 0..5 {
            buzzer.set_high();
            arduino_hal::delay_us(half_period);
            buzzer.set_low();
            arduino_hal::delay_us(half_period);
        }
    }
    arduino_hal::delay_ms(50);
}

pub fn health_check_double_beep(buzzer: &mut impl OutputPin) {
    // Two quick tones with gap - 150ms total
    for _ in 0..2 {
        for _ in 0..8 {
            buzzer.set_high();
            arduino_hal::delay_us(300);
            buzzer.set_low();
            arduino_hal::delay_us(300);
        }
        arduino_hal::delay_ms(100);
    }
}
