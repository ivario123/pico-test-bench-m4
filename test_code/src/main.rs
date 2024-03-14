//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use core::fmt::Write;
use hal::{gpio, uarte, uarte::Uarte};
use nrf52840_hal as hal;
use nrf52840_hal::prelude::*;

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = hal::pac::Peripherals::take().unwrap();
    let mut timer = hal::Timer::one_shot(pac.TIMER0);

    let p0 = hal::gpio::p0::Parts::new(pac.P0);
    let mut led = p0.p0_13.into_push_pull_output(gpio::Level::High).degrade();

    loop {
        info!("on!");
        led.set_high().unwrap();
        timer.delay_ms(500u32);
        info!("off!");
        led.set_low().unwrap();
        timer.delay_ms(500u32);
    }
}
