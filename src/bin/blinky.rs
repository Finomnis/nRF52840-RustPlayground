#![no_main]
#![no_std]

use test_nrf52840 as _; // global logger + panicking-behavior + memory layout
use test_nrf52840::hal;

use hal::{gpio, prelude::*};

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = hal::pac::Peripherals::take().unwrap();
    let port0 = hal::gpio::p0::Parts::new(p.P0);
    let port1 = hal::gpio::p1::Parts::new(p.P1);
    let button = port1.p1_06.into_pullup_input();
    // let mut led = port0.p0_06.into_push_pull_output(gpio::Level::High);

    let mut led_red = port0.p0_08.into_push_pull_output(gpio::Level::High);
    let mut led_green = port1.p1_09.into_push_pull_output(gpio::Level::High);
    // let mut led_blue = port0.p0_12.into_push_pull_output(gpio::Level::High);

    defmt::info!("Blinky button demo starting");
    loop {
        if button.is_high().unwrap() {
            led_red.set_low().unwrap();
            led_green.set_high().unwrap();
        } else {
            led_red.set_high().unwrap();
            led_green.set_low().unwrap();
        }
    }
}
