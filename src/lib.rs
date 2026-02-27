#![no_std]
#![no_main]

pub mod alarm;
pub mod hardware;
pub mod log;
pub mod prelude;
pub mod program;
pub mod sensor;

pub use program::run_program;
pub use program::legacy::run_program_legacy;
