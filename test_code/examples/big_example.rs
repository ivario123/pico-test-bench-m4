//! Blinks the LED on a Pico board

#![no_std]
#![no_main]

use core::arch::asm;

use bsp::{entry, hal::pwm::{Channel, ChannelId, Pwm0}, pac::{IO_BANK0, PADS_BANK0, PWM}};
use cortex_m::peripheral::{syst::SystClkSource, SYST};
use defmt::*;
use defmt_rtt as _;
use embedded_hal::{digital::v2::OutputPin, PwmPin};
use panic_probe as _;

// Provide an alias for our BSP so we can switch targets quickly.
use rp_pico as bsp;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
    pwm,
};
use symex_lib::{assume, end_cyclecount, start_cyclecount, symbolic};

type PwmChannel = Channel<pwm::Slice<Pwm0, pwm::FreeRunning>, pwm::B>;

#[entry]
fn main() -> ! {
    info!("Ex1 start");
    let mut pac: pac::Peripherals = pac::Peripherals::take().unwrap();
    let core: cortex_m::Peripherals = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);

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

    //measure_hw();
    //small_timing_test();
    //smaller_timing_test();
    //measure_symex();
    let r = measure(pac.PADS_BANK0, pac.PWM, pac.IO_BANK0, pac.SIO, pac.RESETS, core);
    info!("r: {}", r);
    loop {}
}

#[inline(never)]
#[no_mangle]
fn measure(PADS_BANK0: pac::PADS_BANK0, PWM: pac::PWM, IO_BANK0: pac::IO_BANK0, SIO: pac::SIO, mut RESETS: pac::RESETS, core: cortex_m::Peripherals) -> u16 {

    let sio = Sio::new(SIO);

    // Set the pins up according to their function on this particular board
    let pins = rp_pico::Pins::new(
        IO_BANK0,
        PADS_BANK0,
        sio.gpio_bank0,
        &mut RESETS,
    );

    // Init PWMs
    let mut pwm_slices = pwm::Slices::new(PWM, &mut RESETS);

    // Configure PWM0
    let pwm = &mut pwm_slices.pwm0;
    //pwm.set_ph_correct();
    pwm.set_div_int(20u8); // 50 hz
    pwm.enable();

    // Output channel B on PWM0 to the GPIO1 pin
    let channel = &mut pwm.channel_b;
    channel.output_to(pins.gpio1);

    let systic_reload_time: u32 = 0x00ffffff;
    let mut systic = core.SYST;
    systic.set_clock_source(SystClkSource::Core);
    systic.set_reload(systic_reload_time);
    systic.enable_counter();

    let mut old_data = Frame { data: [0; 24], index: 24, last: 4 };
    symbolic(&mut old_data.data);
    symbolic(&mut old_data.index);
    symbolic(&mut old_data.last);
    assume(old_data.index <= 24);
    let mut new_data = 4;
    symbolic(&mut new_data);
    start_cyclecount();
    unsafe {
        asm!("bkpt 1");
    }
    let r = handle_inputs(&mut old_data, new_data);
    unsafe {
        asm!("bkpt 1");
    }
    end_cyclecount();
    r
}

#[inline(never)]
#[no_mangle]
fn handle_inputs(
    old_data: &mut Frame,
    new_data: u8,
) -> u16 {
    let channels = match old_data.push_and_try_parse(new_data) {
        Ok(c) => c,
        Err(_) => return 0,
    };

    let pwm_value = calculate_pwm_value(channels[0]);

    pwm_value
}

#[inline(never)]
fn calculate_pwm_value(channel_value: u16) -> u16 {
    let channel_value = channel_value & 0x07ff; // ensure 11 bits
    let minus = (channel_value as i32) - (u16::MAX as i32);
    minus.abs() as u16
}

pub struct Frame{
    data: [u8; 24],
    index: usize,
    last: u8,
}

/// Sbus channels.
pub type Channels = [u16; 17];


impl Frame {

    /// Creates a new Frame that is used as a buffer to parse out the channels.
    pub const fn new() -> Frame {
        Frame{
            data: [0; 24],
            index: 0,
            last: 0x0F,
        }
    }

    /// The function that takes in the most recent message in data and tries to parse the frame.
    /// If the frame is complete and can be parsed it returns Ok with the channels otherwise 
    /// it returns Err which indicates that no complete frame was found at this point.
    #[inline(never)]
    pub fn push_and_try_parse(&mut self, data: u8) -> Result<Channels, ()> {
        const SBUS_HEADER: u8 = 0x0F;
        const SBUS_FOOTER: u8 = 0x00;
        const SBUS_2FOOTER: u8 = 0x04;
        const SBUS_2MASK: u8 = 0x0F;
        const SBUS_SIZE: usize = 24;

        if self.index == 0 {
            // Se if the current data is a header and therefore start of frame.
            if (data == SBUS_HEADER) && ((self.last == SBUS_FOOTER) || ((self.last & SBUS_2MASK) == SBUS_2FOOTER)) {
                // Start of frame found start filling up data.
                self.data[self.index] = data;
                self.index += 1;
            } else {
                self.index = 0;
            }
        } else {
            if self.index < SBUS_SIZE {
                // End not reached push data.
                self.data[self.index] = data;
                self.index += 1;
            } else if self.index == SBUS_SIZE {
                // End reached check if data is footer.
                if (data == SBUS_FOOTER) || ((data & SBUS_2MASK) == SBUS_2FOOTER) {
                    // Is footer parse the frame and return the channels and reset to search for next frame.
                    self.index = 0;
                    return Ok(self.to_channels());
                } else {
                    // Footer not found so something went wrong reset to try again with next frame.
                    self.index = 0;
                    return Err(()); // added random error for now change to proper error later
                }
            }
        }
        self.last = data;
        return Err(());
    }

    #[inline(never)]
    fn to_channels(&self) -> Channels {
        let mut channels : [u16; 17] = [0; 17];

        channels[0] = (((self.data[1]) as u16 | ((self.data[2] as u16) << 8)) & 0x07FF).into();
        channels[1] = (((((self.data[2] as u16) >> 3) | ((self.data[3] as u16) << 5)) as u16) & 0x07FF).into();
        channels[2] = (((((self.data[3] as u16) >> 6) | ((self.data[4] as u16) << 2) | ((self.data[5] as u16) << 10)) as u16) & 0x07FF).into();
        channels[3] = (((((self.data[5] as u16) >> 1) | ((self.data[6] as u16) << 7)) as u16) & 0x07FF).into();
        channels[4] = (((((self.data[6] as u16) >> 4) | ((self.data[7] as u16) << 4)) as u16) & 0x07FF).into();
        channels[5] = (((((self.data[7] as u16) >> 7) | ((self.data[8] as u16) << 1) | ((self.data[9] as u16) << 9)) as u16) & 0x07FF).into();
        channels[6] = (((((self.data[9] as u16) >> 2) | ((self.data[10] as u16) << 6)) as u16) & 0x07FF).into();
        channels[7] = (((((self.data[10] as u16) >> 5) | ((self.data[11] as u16) << 3)) as u16) & 0x07FF).into();
        channels[8] = ((((self.data[12] as u16) | ((self.data[13] as u16) << 8)) as u16) & 0x07FF).into();
        channels[9] = (((((self.data[13] as u16) >> 3) | ((self.data[14] as u16) << 5)) as u16) & 0x07FF).into();
        channels[10] = (((((self.data[14] as u16) >> 6) | ((self.data[15] << 2) as u16) | ((self.data[16] as u16) << 10)) as u16) & 0x07FF).into();
        channels[11] = (((((self.data[16] as u16) >> 1) | ((self.data[17] as u16) << 7)) as u16) & 0x07FF).into();
        channels[12] = (((((self.data[17] as u16) >> 4) | ((self.data[18] as u16) << 4)) as u16) & 0x07FF).into();
        channels[13] = (((((self.data[18] as u16) >> 7) | ((self.data[19] as u16) << 1) | ((self.data[20] as u16) << 9)) as u16) & 0x07FF) .into();
        channels[14] = ((((self.data[20] as u16) >> 2) | ((self.data[21] as u16) << 6) as u16) & 0x07FF).into();
        channels[15] = (((((self.data[21] as u16) >> 5) | ((self.data[22] as u16) << 3)) as u16) & 0x07FF).into();
        channels[16] = self.data[23] as u16;
        
        return channels;
    }
}