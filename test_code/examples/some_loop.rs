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
use symex_lib::assume;

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
    let _clocks = hal::clocks::Clocks::new(pac.CLOCK);
    let systic_reload_time: u32 = 0x00ffffff;
    let mut systic = core.SYST;
    systic.set_clock_source(cortex_m::peripheral::syst::SystClkSource::Core);
    systic.set_reload(systic_reload_time);
    systic.enable_counter();

    //measure_hw();
    //small_timing_test();
    //smaller_timing_test();
    //measure_symex();
    // unsafe {
    //     asm!("bkpt 1");
    // }
    let r = measure(0xff);
    info!("r: {}", r);
    loop {}
}

#[inline(never)]
#[no_mangle]
fn measure(v: u8) -> u16 {
    // assume(v == 0xff);
    start_cyclecount();
    unsafe {
        asm!("bkpt 1");
    }
    let r = simple_loop(v);
    unsafe {
        asm!("bkpt 1");
    }
    end_cyclecount();
    r
}

#[inline(never)]
#[no_mangle]
fn simple_loop(n: u8) -> u16 {
    let mut sum = 0;
    for i in 0..n {
        sum += simple_if(i) as u16;
    }
    sum
}

#[inline(never)]
#[no_mangle]
fn simple_if(n: u8) -> u8 {
    if n == 3 {
        1
    } else if n == 6 {
        5
    } else {
        2
    }
}
