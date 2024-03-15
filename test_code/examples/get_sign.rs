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
use symex_lib::symbolic;

use core::arch::asm;
use core::fmt::Write;
use hal::{gpio, uarte, uarte::Uarte};
use nrf52840_hal as hal;
use nrf52840_hal::pac;
use nrf52840_hal::prelude::*;

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
    let r = measure(1);
    info!("r: {}", r);
    loop {}
}

#[inline(never)]
#[no_mangle]
fn measure(v: i32) -> i32 {
    start_cyclecount();
    unsafe {
        asm!("bkpt 1");
    }
    let r = get_sign(v);
    unsafe {
        asm!("bkpt 1");
    }
    end_cyclecount();
    r
}

#[inline(never)]
#[no_mangle]
fn get_sign(v: i32) -> i32 {
    if v > 0 {
        return 1;
    } else if v == 0 {
        return 0;
    } else {
        return -1;
    }
}
