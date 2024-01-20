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
use symex_lib::{end_cyclecount, start_cyclecount, symbolic};

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
    let r = measure();
    info!("r: {}", r);
    loop {}
}

#[inline(never)]
#[no_mangle]
fn measure() -> [u8; 2] {
    let mut crc_type = CrcType::CrcA;
    let mut data = [42; 16];
    symbolic(&mut crc_type);
    symbolic(&mut data);
    start_cyclecount();
    unsafe {
        asm!("bkpt 1");
    }
    let r = compute_crc(crc_type, &data);
    unsafe {
        asm!("bkpt 1");
    }
    end_cyclecount();
    r
}

#[derive(Debug, PartialEq, Eq)]
pub enum CrcType {
    /// CRC for A type picc
    CrcA,
    /// CRC for B type picc
    CrcB,
}

fn update_crc(ch: u8, lpw_crc: &mut u16) {
    let mut ch_calc = ch;
    ch_calc = ch_calc ^ ((*lpw_crc & 0x00ff) as u8);
    ch_calc = 0xff & (ch_calc ^ (ch_calc << 4));
    *lpw_crc = (*lpw_crc >> 8)
        ^ ((ch_calc as u16) << 8)
        ^ ((ch_calc as u16) << 3)
        ^ ((ch_calc as u16) >> 4);
}

#[inline(never)]
#[no_mangle]
/// Computes the crc according to crc_type.
pub fn compute_crc(crc_type: CrcType, data: &[u8]) -> [u8; 2] {
    let mut w_crc = match crc_type {
        CrcType::CrcA => 0x6363,
        CrcType::CrcB => 0xFFFF,
    };

    for byte in data {
        update_crc(*byte, &mut w_crc);
    }

    if crc_type == CrcType::CrcB {
        w_crc = !w_crc;
    }

    [(w_crc & 0xff) as u8, ((w_crc >> 8) & 0xff) as u8]
}
