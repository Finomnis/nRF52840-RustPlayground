#![no_main]
#![no_std]

use test_nrf52840 as _; // global logger + panicking-behavior + memory layout
use test_nrf52840::hal;

use hal::{gpio, prelude::*, pwm, pwm::Pwm, timer::Timer};

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = hal::pac::Peripherals::take().unwrap();
    let port0 = hal::gpio::p0::Parts::new(p.P0);
    let port1 = hal::gpio::p1::Parts::new(p.P1);
    let button = port1.p1_06.into_pullup_input();
    // let mut led = port0.p0_06.into_push_pull_output(gpio::Level::High);

    let mut led_red = port0.p0_08.into_push_pull_output(gpio::Level::High);
    let led_green = port1.p1_09.into_push_pull_output(gpio::Level::High);
    let mut led_blue = port0.p0_12.into_push_pull_output(gpio::Level::High);

    let mut timer = Timer::new(p.TIMER0);

    let led_pwm = Pwm::new(p.PWM0);
    led_pwm.set_output_pin(pwm::Channel::C0, led_green.degrade());
    led_pwm.set_period(500u32.hz());

    defmt::info!("Blinky button demo starting");
    let duty_max = led_pwm.get_max_duty() / 8;
    let wait_time = 2_000_000u32 / duty_max as u32;

    let mut wait_for_timer = |time| {
        timer.start(time);
        loop {
            if let Ok(()) = timer.wait() {
                break;
            }

            if button.is_high().unwrap() {
                led_red.set_low().unwrap();
                led_blue.set_high().unwrap();
            } else {
                led_red.set_high().unwrap();
                led_blue.set_low().unwrap();
            }
        }
    };

    loop {
        for duty in 0..duty_max {
            led_pwm.set_duty_on_common(duty);
            wait_for_timer(wait_time);
        }
        defmt::info!("Duty: {}", duty_max);
        for duty in (0..duty_max).rev() {
            led_pwm.set_duty_on_common(duty);
            wait_for_timer(wait_time);
        }
        defmt::info!("Duty: {}", 0);
    }
}
