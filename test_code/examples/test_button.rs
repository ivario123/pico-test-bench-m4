//! Blinks the LED on a Pico board

#![no_std]
#![no_main]

use core::arch::asm;

use bsp::{entry, hal::gpio::{bank0::{Gpio22, Gpio25}, FunctionSio, Pin, PullDown, PullUp, SioInput, SioOutput}};
use cortex_m::peripheral::{syst::SystClkSource, SYST};
use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::v2::{InputPin, OutputPin};
use panic_probe as _;

// Provide an alias for our BSP so we can switch targets quickly.
use rp_pico as bsp;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};
use symex_lib::{end_cyclecount, start_cyclecount};

type Led = Pin<Gpio25, FunctionSio<SioOutput>, PullDown>;
type Button = Pin<Gpio22, FunctionSio<SioInput>, PullDown>;


#[entry]
fn main() -> ! {
    info!("Ex1 start");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let gpioa = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led = gpioa.led.into_push_pull_output();
    let button = gpioa.gpio22.into_pull_down_input();

    let systic_reload_time: u32 = 0x00ffffff;
    let mut systic = core.SYST;
    systic.set_clock_source(SystClkSource::Core);
    systic.set_reload(systic_reload_time);
    systic.enable_counter();

    //measure_hw();
    //small_timing_test();
    //smaller_timing_test();
    //measure_symex();
    let r = measure(led, button);
    info!("r: {}", r);
    loop {}
}

#[inline(never)]
#[no_mangle]
fn measure(mut led: Led, button: Button) -> bool {
    start_cyclecount();
    unsafe {
        asm!("bkpt 1");
    }
    let value = handle_button(&mut led, &button);
    unsafe {
        asm!("bkpt 1");
    }
    end_cyclecount();
    value
}

fn handle_button(led: &mut Led, button: &Button) -> bool {
    if button.is_high().unwrap() {
        led.set_high().unwrap();
        true
    } else {
        led.set_low().unwrap();
        false
    }
}