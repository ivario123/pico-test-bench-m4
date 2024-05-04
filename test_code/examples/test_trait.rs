//! Tests trait implementations.
//!
//! This test works as of may the 4th 2024.
#![no_std]
#![no_main]

use core::arch::asm;

use cortex_m_rt::entry;
use defmt::*;
use defmt_rtt as _;
use nrf52840_hal as hal;
use nrf52840_hal::pac;
use panic_probe as _;
use symex_lib::{end_cyclecount, start_cyclecount};

#[entry]
fn main() -> ! {
    info!("Ex1 start");
    let pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let _clocks = hal::clocks::Clocks::new(pac.CLOCK).enable_ext_hfosc();
    let systic_reload_time: u32 = 0x00ffffff;
    let mut systic = core.SYST;
    systic.set_clock_source(cortex_m::peripheral::syst::SystClkSource::External);
    systic.set_reload(systic_reload_time);
    systic.enable_counter();

    let r = measure([1, 2, 3]);
    info!("r: {}", r);
    let r = reverser([1, 2]);
    info!("r: {}", r);
    loop {}
}

#[inline(never)]
#[no_mangle]
fn measure(data1: [u8; 3]) -> [u8; 3] {
    start_cyclecount();
    unsafe {
        asm!("bkpt 1");
    }
    let data1 = reverser(data1);
    unsafe {
        asm!("bkpt 1");
    }
    end_cyclecount();
    data1
}

#[inline(never)]
fn reverser<T: Reversible>(mut data: T) -> T {
    data.reverse();
    data
}

trait Reversible {
    fn reverse(&mut self);
}

impl Reversible for [u8; 2] {
    fn reverse(&mut self) {
        let old = self.clone();
        self[0] = old[1];
        self[1] = old[0];
    }
}

impl Reversible for [u8; 3] {
    fn reverse(&mut self) {
        let old = self.clone();
        self[0] = old[2];
        self[2] = old[0];
    }
}
