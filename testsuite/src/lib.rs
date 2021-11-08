#![no_std]
#![cfg_attr(test, no_main)]

use test_nrf52840 as _; // memory layout + panic handler

#[defmt_test::tests]
mod tests {}
