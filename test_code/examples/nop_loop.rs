//! Blinks the LED on a Pico board

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::*;
use defmt_rtt as _;
use hal::pac::SYST;
use panic_probe as _;
use symex_lib::end_cyclecount;
use symex_lib::start_cyclecount;

use core::arch::asm;
use core::fmt::Write;
use hal::{gpio, uarte, uarte::Uarte};
use nrf52840_hal as hal;
use nrf52840_hal::prelude::*;
use nrf52840_hal::pac;

#[entry]
fn main() -> ! {
    info!("Ex1 start");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let _clocks = hal::clocks::Clocks::new(pac.CLOCK).enable_ext_hfosc();
    let systic_reload_time: u32 = 0x00ffffff;
    let mut systic = core.SYST;
    systic.set_clock_source(cortex_m::peripheral::syst::SystClkSource::External);
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
