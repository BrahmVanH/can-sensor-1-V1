#![no_std]
#![no_main]

use can_monitor_1_rust::run_program;

#[arduino_hal::entry]
fn main() -> ! {
    run_program()
}
