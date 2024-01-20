#![feature(prelude_import)]
//! Blinks the LED on a Pico board
#![no_std]
#![no_main]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
use core::arch::asm;
use bsp::entry;
use cortex_m::peripheral::{syst::SystClkSource, SYST};
use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::v2::OutputPin;
use panic_probe as _;
use rp_pico as bsp;
use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac, sio::Sio, watchdog::Watchdog,
};
use symex_lib::{end_cyclecount, start_cyclecount, symbolic, valid, assume, Valid};
use valid_derive::Validate;
#[doc(hidden)]
#[export_name = "main"]
pub unsafe extern "C" fn __cortex_m_rt_main_trampoline() {
    __cortex_m_rt_main()
}
fn __cortex_m_rt_main() -> ! {
    unsafe {
        const SIO_BASE: u32 = 0xd0000000;
        const SPINLOCK0_PTR: *mut u32 = (SIO_BASE + 0x100) as *mut u32;
        const SPINLOCK_COUNT: usize = 32;
        for i in 0..SPINLOCK_COUNT {
            SPINLOCK0_PTR.wrapping_add(i).write_volatile(1);
        }
    }
    match () {
        () => {
            if {
                const CHECK: bool = {
                    const fn check() -> bool {
                        let module_path = "enum_match".as_bytes();
                        if if 10usize > module_path.len() {
                            false
                        } else {
                            module_path[0usize] == 101u8 && module_path[1usize] == 110u8
                                && module_path[2usize] == 117u8
                                && module_path[3usize] == 109u8
                                && module_path[4usize] == 95u8
                                && module_path[5usize] == 109u8
                                && module_path[6usize] == 97u8
                                && module_path[7usize] == 116u8
                                && module_path[8usize] == 99u8
                                && module_path[9usize] == 104u8
                                && if 10usize == module_path.len() {
                                    true
                                } else {
                                    module_path[10usize] == b':'
                                }
                        } {
                            return true;
                        }
                        false
                    }
                    check()
                };
                CHECK
            } {
                unsafe { defmt::export::acquire() };
                defmt::export::header(
                    &{
                        defmt::export::make_istr({
                            #[link_section = ".defmt.{\"package\":\"rp2040-rtic\",\"tag\":\"defmt_info\",\"data\":\"Ex1 start\",\"disambiguator\":\"13684650239054855457\",\"crate_name\":\"enum_match\"}"]
                            #[export_name = "{\"package\":\"rp2040-rtic\",\"tag\":\"defmt_info\",\"data\":\"Ex1 start\",\"disambiguator\":\"13684650239054855457\",\"crate_name\":\"enum_match\"}"]
                            static DEFMT_LOG_STATEMENT: u8 = 0;
                            &DEFMT_LOG_STATEMENT as *const u8 as u16
                        })
                    },
                );
                unsafe { defmt::export::release() }
            }
        }
    };
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);
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
    let r = measure(TestEnum::One);
    match (&(r)) {
        (arg0) => {
            if {
                const CHECK: bool = {
                    const fn check() -> bool {
                        let module_path = "enum_match".as_bytes();
                        if if 10usize > module_path.len() {
                            false
                        } else {
                            module_path[0usize] == 101u8 && module_path[1usize] == 110u8
                                && module_path[2usize] == 117u8
                                && module_path[3usize] == 109u8
                                && module_path[4usize] == 95u8
                                && module_path[5usize] == 109u8
                                && module_path[6usize] == 97u8
                                && module_path[7usize] == 116u8
                                && module_path[8usize] == 99u8
                                && module_path[9usize] == 104u8
                                && if 10usize == module_path.len() {
                                    true
                                } else {
                                    module_path[10usize] == b':'
                                }
                        } {
                            return true;
                        }
                        false
                    }
                    check()
                };
                CHECK
            } {
                unsafe { defmt::export::acquire() };
                defmt::export::header(
                    &{
                        defmt::export::make_istr({
                            #[link_section = ".defmt.{\"package\":\"rp2040-rtic\",\"tag\":\"defmt_info\",\"data\":\"r: {}\",\"disambiguator\":\"5046490553134408291\",\"crate_name\":\"enum_match\"}"]
                            #[export_name = "{\"package\":\"rp2040-rtic\",\"tag\":\"defmt_info\",\"data\":\"r: {}\",\"disambiguator\":\"5046490553134408291\",\"crate_name\":\"enum_match\"}"]
                            static DEFMT_LOG_STATEMENT: u8 = 0;
                            &DEFMT_LOG_STATEMENT as *const u8 as u16
                        })
                    },
                );
                defmt::export::fmt(arg0);
                unsafe { defmt::export::release() }
            }
        }
    };
    let r = measure(TestEnum::Two);
    match (&(r)) {
        (arg0) => {
            if {
                const CHECK: bool = {
                    const fn check() -> bool {
                        let module_path = "enum_match".as_bytes();
                        if if 10usize > module_path.len() {
                            false
                        } else {
                            module_path[0usize] == 101u8 && module_path[1usize] == 110u8
                                && module_path[2usize] == 117u8
                                && module_path[3usize] == 109u8
                                && module_path[4usize] == 95u8
                                && module_path[5usize] == 109u8
                                && module_path[6usize] == 97u8
                                && module_path[7usize] == 116u8
                                && module_path[8usize] == 99u8
                                && module_path[9usize] == 104u8
                                && if 10usize == module_path.len() {
                                    true
                                } else {
                                    module_path[10usize] == b':'
                                }
                        } {
                            return true;
                        }
                        false
                    }
                    check()
                };
                CHECK
            } {
                unsafe { defmt::export::acquire() };
                defmt::export::header(
                    &{
                        defmt::export::make_istr({
                            #[link_section = ".defmt.{\"package\":\"rp2040-rtic\",\"tag\":\"defmt_info\",\"data\":\"r: {}\",\"disambiguator\":\"14672199346832941299\",\"crate_name\":\"enum_match\"}"]
                            #[export_name = "{\"package\":\"rp2040-rtic\",\"tag\":\"defmt_info\",\"data\":\"r: {}\",\"disambiguator\":\"14672199346832941299\",\"crate_name\":\"enum_match\"}"]
                            static DEFMT_LOG_STATEMENT: u8 = 0;
                            &DEFMT_LOG_STATEMENT as *const u8 as u16
                        })
                    },
                );
                defmt::export::fmt(arg0);
                unsafe { defmt::export::release() }
            }
        }
    };
    let r = measure(TestEnum::Three);
    match (&(r)) {
        (arg0) => {
            if {
                const CHECK: bool = {
                    const fn check() -> bool {
                        let module_path = "enum_match".as_bytes();
                        if if 10usize > module_path.len() {
                            false
                        } else {
                            module_path[0usize] == 101u8 && module_path[1usize] == 110u8
                                && module_path[2usize] == 117u8
                                && module_path[3usize] == 109u8
                                && module_path[4usize] == 95u8
                                && module_path[5usize] == 109u8
                                && module_path[6usize] == 97u8
                                && module_path[7usize] == 116u8
                                && module_path[8usize] == 99u8
                                && module_path[9usize] == 104u8
                                && if 10usize == module_path.len() {
                                    true
                                } else {
                                    module_path[10usize] == b':'
                                }
                        } {
                            return true;
                        }
                        false
                    }
                    check()
                };
                CHECK
            } {
                unsafe { defmt::export::acquire() };
                defmt::export::header(
                    &{
                        defmt::export::make_istr({
                            #[link_section = ".defmt.{\"package\":\"rp2040-rtic\",\"tag\":\"defmt_info\",\"data\":\"r: {}\",\"disambiguator\":\"5815022772197952069\",\"crate_name\":\"enum_match\"}"]
                            #[export_name = "{\"package\":\"rp2040-rtic\",\"tag\":\"defmt_info\",\"data\":\"r: {}\",\"disambiguator\":\"5815022772197952069\",\"crate_name\":\"enum_match\"}"]
                            static DEFMT_LOG_STATEMENT: u8 = 0;
                            &DEFMT_LOG_STATEMENT as *const u8 as u16
                        })
                    },
                );
                defmt::export::fmt(arg0);
                unsafe { defmt::export::release() }
            }
        }
    };
    let r = measure(TestEnum::Four);
    match (&(r)) {
        (arg0) => {
            if {
                const CHECK: bool = {
                    const fn check() -> bool {
                        let module_path = "enum_match".as_bytes();
                        if if 10usize > module_path.len() {
                            false
                        } else {
                            module_path[0usize] == 101u8 && module_path[1usize] == 110u8
                                && module_path[2usize] == 117u8
                                && module_path[3usize] == 109u8
                                && module_path[4usize] == 95u8
                                && module_path[5usize] == 109u8
                                && module_path[6usize] == 97u8
                                && module_path[7usize] == 116u8
                                && module_path[8usize] == 99u8
                                && module_path[9usize] == 104u8
                                && if 10usize == module_path.len() {
                                    true
                                } else {
                                    module_path[10usize] == b':'
                                }
                        } {
                            return true;
                        }
                        false
                    }
                    check()
                };
                CHECK
            } {
                unsafe { defmt::export::acquire() };
                defmt::export::header(
                    &{
                        defmt::export::make_istr({
                            #[link_section = ".defmt.{\"package\":\"rp2040-rtic\",\"tag\":\"defmt_info\",\"data\":\"r: {}\",\"disambiguator\":\"7700749238242250749\",\"crate_name\":\"enum_match\"}"]
                            #[export_name = "{\"package\":\"rp2040-rtic\",\"tag\":\"defmt_info\",\"data\":\"r: {}\",\"disambiguator\":\"7700749238242250749\",\"crate_name\":\"enum_match\"}"]
                            static DEFMT_LOG_STATEMENT: u8 = 0;
                            &DEFMT_LOG_STATEMENT as *const u8 as u16
                        })
                    },
                );
                defmt::export::fmt(arg0);
                unsafe { defmt::export::release() }
            }
        }
    };
    let r = measure(TestEnum::Five);
    match (&(r)) {
        (arg0) => {
            if {
                const CHECK: bool = {
                    const fn check() -> bool {
                        let module_path = "enum_match".as_bytes();
                        if if 10usize > module_path.len() {
                            false
                        } else {
                            module_path[0usize] == 101u8 && module_path[1usize] == 110u8
                                && module_path[2usize] == 117u8
                                && module_path[3usize] == 109u8
                                && module_path[4usize] == 95u8
                                && module_path[5usize] == 109u8
                                && module_path[6usize] == 97u8
                                && module_path[7usize] == 116u8
                                && module_path[8usize] == 99u8
                                && module_path[9usize] == 104u8
                                && if 10usize == module_path.len() {
                                    true
                                } else {
                                    module_path[10usize] == b':'
                                }
                        } {
                            return true;
                        }
                        false
                    }
                    check()
                };
                CHECK
            } {
                unsafe { defmt::export::acquire() };
                defmt::export::header(
                    &{
                        defmt::export::make_istr({
                            #[link_section = ".defmt.{\"package\":\"rp2040-rtic\",\"tag\":\"defmt_info\",\"data\":\"r: {}\",\"disambiguator\":\"4853838974843172275\",\"crate_name\":\"enum_match\"}"]
                            #[export_name = "{\"package\":\"rp2040-rtic\",\"tag\":\"defmt_info\",\"data\":\"r: {}\",\"disambiguator\":\"4853838974843172275\",\"crate_name\":\"enum_match\"}"]
                            static DEFMT_LOG_STATEMENT: u8 = 0;
                            &DEFMT_LOG_STATEMENT as *const u8 as u16
                        })
                    },
                );
                defmt::export::fmt(arg0);
                unsafe { defmt::export::release() }
            }
        }
    };
    let r = symex_test(4);
    match (&(r)) {
        (arg0) => {
            if {
                const CHECK: bool = {
                    const fn check() -> bool {
                        let module_path = "enum_match".as_bytes();
                        if if 10usize > module_path.len() {
                            false
                        } else {
                            module_path[0usize] == 101u8 && module_path[1usize] == 110u8
                                && module_path[2usize] == 117u8
                                && module_path[3usize] == 109u8
                                && module_path[4usize] == 95u8
                                && module_path[5usize] == 109u8
                                && module_path[6usize] == 97u8
                                && module_path[7usize] == 116u8
                                && module_path[8usize] == 99u8
                                && module_path[9usize] == 104u8
                                && if 10usize == module_path.len() {
                                    true
                                } else {
                                    module_path[10usize] == b':'
                                }
                        } {
                            return true;
                        }
                        false
                    }
                    check()
                };
                CHECK
            } {
                unsafe { defmt::export::acquire() };
                defmt::export::header(
                    &{
                        defmt::export::make_istr({
                            #[link_section = ".defmt.{\"package\":\"rp2040-rtic\",\"tag\":\"defmt_info\",\"data\":\"r: {}\",\"disambiguator\":\"17020605592461201783\",\"crate_name\":\"enum_match\"}"]
                            #[export_name = "{\"package\":\"rp2040-rtic\",\"tag\":\"defmt_info\",\"data\":\"r: {}\",\"disambiguator\":\"17020605592461201783\",\"crate_name\":\"enum_match\"}"]
                            static DEFMT_LOG_STATEMENT: u8 = 0;
                            &DEFMT_LOG_STATEMENT as *const u8 as u16
                        })
                    },
                );
                defmt::export::fmt(arg0);
                unsafe { defmt::export::release() }
            }
        }
    };
    loop {}
}
enum TestEnum {
    One,
    Two,
    Three,
    Four,
    Five,
}
impl symex_lib::Valid for TestEnum {
    fn is_valid(&self) -> bool {
        let mut ret = true;
        if let TestEnum::One = self {
            symex_lib::black_box(&mut ret)
        } else if let TestEnum::Two = self {
            symex_lib::black_box(&mut ret)
        } else if let TestEnum::Three = self {
            symex_lib::black_box(&mut ret)
        } else if let TestEnum::Four = self {
            symex_lib::black_box(&mut ret)
        } else if let TestEnum::Five = self {
            symex_lib::black_box(&mut ret)
        } else {
            symex_lib::black_box(&mut ret);
            symex_lib::suppress_path();
            ret = false;
        }
        ret
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for TestEnum {}
#[automatically_derived]
impl ::core::cmp::PartialEq for TestEnum {
    #[inline]
    fn eq(&self, other: &TestEnum) -> bool {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        __self_tag == __arg1_tag
    }
}
#[automatically_derived]
impl ::core::marker::StructuralEq for TestEnum {}
#[automatically_derived]
impl ::core::cmp::Eq for TestEnum {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[inline(never)]
#[no_mangle]
fn measure(v: TestEnum) -> u16 {
    start_cyclecount();
    unsafe {}
    let r = handle_test_enum(v);
    unsafe {}
    end_cyclecount();
    r
}
#[inline(never)]
#[no_mangle]
fn symex_test(n: u32) -> u16 {
    let mut input: TestEnum = TestEnum::Five;
    symbolic(&mut input);
    input.is_valid();
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
        ret = false;
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
        {
            ::core::panicking::panic_fmt(format_args!("hello"));
        }
    }
}
#[inline(never)]
#[no_mangle]
fn simple_if(n: u8) -> u8 {
    if n == 3 { 1 } else if n == 6 { 5 } else { 2 }
}
