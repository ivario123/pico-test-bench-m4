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
use symex_lib::{end_cyclecount, start_cyclecount, symbolic, valid, assume, Valid};
use valid_derive::Validate;

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
    let r = measure();
    info!("r: {}", r);
    let r = measure();
    info!("r: {}", r);
    let r = measure();
    info!("r: {}", r);
    let r = measure();
    info!("r: {}", r);
    let r = symex_test(4);
    info!("r: {}", r);
    loop {}
}

#[derive(Validate, PartialEq, Eq)]
enum TestEnum {
    One,
    Two,
    Three,
    Four,
    Five,
}

#[inline(never)]
#[no_mangle]
fn measure() -> u16 {
    let mut input: TestEnum = TestEnum::Two;
    symbolic(&mut input);
    if_test3(&input);
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
fn symex_test(n: u32) -> u16 {
    let mut input: TestEnum = TestEnum::Five;
    symbolic(&mut input);
    //valid(&input);
    //input.is_valid();
    if_test3(&input);
    handle_test_enum(input)
}

#[inline(never)]
#[no_mangle]
fn handle_test_enum(n: TestEnum) -> u16 {
    match n {
        TestEnum::One => 1,
        TestEnum::Two => simple_if(2) as u16,
        TestEnum::Three => 9,
        TestEnum::Four => 5,
        TestEnum::Five => simple_if(5 as u8) as u16,
    }
}

#[inline(never)]
#[no_mangle]
fn if_test3(n: &TestEnum) -> bool {
    let mut ret = true;
    if let TestEnum::One = n {
        symex_lib::black_box(&mut ret)
    } else if let TestEnum::Two = n {
        symex_lib::black_box(&mut ret)
    } else if let TestEnum::Three = n {
        symex_lib::black_box(&mut ret)
    } else if let TestEnum::Four = n {
        symex_lib::black_box(&mut ret)
    } else if let TestEnum::Five = n {
        symex_lib::black_box(&mut ret)
    } else {
        symex_lib::black_box(&mut ret);
        symex_lib::suppress_path();
        ret = false
    }
    ret
}

#[inline(never)]
#[no_mangle]
fn if_test2(n: &TestEnum) -> u8 {
    if let TestEnum::One = n {
        1
    } else if let TestEnum::Two = n {
        2
    } else if let TestEnum::Three = n {
        3
    } else if let TestEnum::Four = n {
        4
    } else if let TestEnum::Five = n {
        simple_if(5)
    } else {
        symex_lib::suppress_path();
        0
    }
}

#[inline(never)]
#[no_mangle]
fn if_test(n: TestEnum) -> u16 {
    if n == TestEnum::One {
        13
    } else if n == TestEnum::Two {
        12
    } else if n == TestEnum::Three {
        22
    } else if n == TestEnum::Four {
        simple_if(42) as u16 
    } else if n == TestEnum::Five {
        simple_if(1) as u16
    } else {
        core::panic!("hello")
    }
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
