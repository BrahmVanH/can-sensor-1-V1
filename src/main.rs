#![no_std]
#![no_main]

use can_monitor_1_rust::run_program;
// use can_monitor_1_rust::run_program_legacy;

#[arduino_hal::entry]
fn main() -> ! {
    run_program()
    // run_program_legacy()
}
