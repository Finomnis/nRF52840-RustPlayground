#![no_main]
#![no_std]

use test_nrf52840 as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    test_nrf52840::exit()
}
