//! Blinks the LED on a Pico board

#![no_std]
#![no_main]
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
