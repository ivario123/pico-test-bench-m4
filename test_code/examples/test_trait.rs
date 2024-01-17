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
    let r = measure([1, 2, 3]);
    info!("r: {}", r);
    let r = reverser([1, 2]);
    info!("r: {}", r);
    loop {}
}

#[inline(never)]
#[no_mangle]
fn measure(mut data1: [u8; 3]) -> [u8; 3] {
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