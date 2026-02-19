use crate::prelude::*;

pub fn get_peripherals() -> arduino_hal::Peripherals {
    arduino_hal::Peripherals::take().unwrap()
}

pub fn get_pin_input_device<T: PinOps>(pin: Pin<Input<Floating>, T>) -> Pin<Input<PullUp>, T> {
    pin.into_pull_up_input()
}

pub fn get_pin_output_device<T: PinOps>(
    pin: Pin<Input<Floating>, T>
) -> Pin<arduino_hal::port::mode::Output, T> {
    pin.into_output()
}
