//! Blinks the LED on a Pico board

#![no_std]
#![no_main]

use core::arch::asm;

use bsp::entry;
use cortex_m::peripheral::{syst::SystClkSource, SYST};
use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::v2::OutputPin;
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

    let systic_reload_time: u32 = 0x00ffffff;
    let mut systic = core.SYST;
    systic.set_clock_source(SystClkSource::Core);
    systic.set_reload(systic_reload_time);
    systic.enable_counter();

    //measure_hw();
    //small_timing_test();
    //smaller_timing_test();
    //measure_symex();
    measure();
    loop {}
}

#[inline(never)]
#[no_mangle]
fn measure() {
    start_cyclecount();
    unsafe {
        asm!("bkpt 1");
    }
    nop_loop();
    unsafe {
        asm!("bkpt 1");
    }
    end_cyclecount();
}

#[inline(never)]
#[no_mangle]
fn measure_symex() {
    start_cyclecount();
    nop_loop();
    end_cyclecount();
}
// symex: 40028

#[inline(never)]
#[no_mangle]
fn nop_loop() {
    for _ in 0..10000 {
        unsafe {
            asm!("nop");
        }
    }
}

#[inline(never)]
#[no_mangle]
fn small_timing_test() {
    unsafe {
        asm!("bkpt 2", "nop", "nop", "bkpt 2",);
    }
}

#[inline(never)]
#[no_mangle]
fn smaller_timing_test() {
    unsafe {
        asm!("bkpt 2", "nop", "bkpt 2",);
    }
}

#[inline(never)]
#[no_mangle]
fn measure_hw() -> u32 {
    let start = SYST::get_current();
    unsafe {
        asm!("bkpt 1");
    }
    nop_loop();
    unsafe {
        asm!("bkpt 1");
    }
    let stop = SYST::get_current();
    start - stop
}
// HW 40015
