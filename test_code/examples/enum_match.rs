//! Tests enum matching.
//!
//! This example works as of May the 4th 2024.
#![no_std]
#![no_main]

use core::arch::asm;

use cortex_m_rt::entry;
use defmt::*;
use defmt_rtt as _;
use nrf52840_hal as hal;
use nrf52840_hal::pac;
use panic_probe as _;
use symex_lib::{any, end_cyclecount, start_cyclecount, symbolic, Any};

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

    let r = measure();
    info!("r: {}", r);
    let r = measure();
    info!("r: {}", r);
    let r = measure();
    info!("r: {}", r);
    let r = measure();
    info!("r: {}", r);
    let r = measure();
    info!("r: {}", r);
    let val = test_any();
    info!("any: {}", val);
    loop {}
}

enum Inner {
    One,
    Two(u16, u16),
}

impl Any for Inner {
    fn any() -> Self {
        match u8::any() {
            0 => Inner::One,
            _ => Inner::Two(u16::any(), u16::any()),
        }
    }
}

enum TestEnum {
    One,
    Two,
    Three(u16),
    Four(Inner),
    Five(u16),
}

impl Any for TestEnum {
    fn any() -> Self {
        let mut n = 1u8;
        symbolic(&mut n);
        match n {
            0 => TestEnum::One,
            1 => TestEnum::Two,
            2 => TestEnum::Three(u16::any()),
            3 => TestEnum::Four(Inner::any()),
            _ => TestEnum::Five(u16::any()),
        }
    }
}

#[inline(never)]
#[no_mangle]
fn measure() -> u16 {
    let input: TestEnum = any();
    start_cyclecount();
    unsafe {
        asm!("bkpt 1");
    }
    let r = handle_test_enum(input);
    unsafe {
        asm!("bkpt 1");
    }
    end_cyclecount();
    r
}

#[inline(never)]
#[no_mangle]
fn test_any() -> u16 {
    let input: TestEnum = any();
    let r = handle_test_enum(input);
    r
}

#[inline(never)]
#[no_mangle]
fn handle_test_enum(n: TestEnum) -> u16 {
    match n {
        TestEnum::One => 1,
        TestEnum::Two => simple_if(2),
        TestEnum::Three(v) => v,
        TestEnum::Four(i) => match i {
            Inner::One => 1,
            Inner::Two(a, b) => a + b,
        },
        TestEnum::Five(v) => simple_if(v),
    }
}

#[inline(never)]
#[no_mangle]
fn simple_if(n: u16) -> u16 {
    if n == 3 {
        1
    } else if n == 6 {
        5
    } else {
        2
    }
}
