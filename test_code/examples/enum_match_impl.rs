//! Tests enum matching, this method produces 8 paths which is correct as the
//! compiler flattens both the paths that return `1` in to one path. Chaning one
//! of the return values proves that this assumption is correct.

#![no_std]
#![no_main]

use core::arch::asm;

use cortex_m_rt::entry;
use defmt::*;
use defmt_rtt as _;
use nrf52840_hal as hal;
use nrf52840_hal::pac;
use panic_probe as _;
use symex_lib::{any, assume, end_cyclecount, start_cyclecount, symbolic, Any, Valid};

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

enum TestEnum2 {
    One,
    Two,
    Three(u16),
    Four(u16),
    Five(u16),
}

impl Valid for TestEnum2 {
    #[inline(never)]
    fn is_valid(&self) -> bool {
        let input = &unsafe {
            let raw_pointer = core::ptr::addr_of!(*self);
            core::ptr::read_volatile(raw_pointer as *const Self)
        };

        if let TestEnum2::One = input {
            true
        } else if let TestEnum2::Two = input {
            true
        } else if let TestEnum2::Three(t) = input {
            t.is_valid()
        } else if let TestEnum2::Four(t) = input {
            t.is_valid()
        } else if let TestEnum2::Five(t) = input {
            t.is_valid()
        } else {
            false
        }
    }
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
fn test_validate() -> u16 {
    let mut input: TestEnum2 = TestEnum2::One;
    symbolic(&mut input);
    //enum_2_if(&input);
    assume(input.is_valid());
    let r = handle_test_enum2(input);
    r
}

#[inline(never)]
#[no_mangle]
fn handle_test_enum2(n: TestEnum2) -> u16 {
    match n {
        TestEnum2::One => 1,
        TestEnum2::Two => simple_if(2),
        TestEnum2::Three(v) => v,
        TestEnum2::Four(i1) => i1 + i1,
        TestEnum2::Five(v) => simple_if(v),
    }
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
