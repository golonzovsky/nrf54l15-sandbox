#![no_std]
#![no_main]

use core::panic::PanicInfo;
use cortex_m::asm::nop;
use cortex_m_rt::entry;
use embassy_nrf::gpio::{Level, Output, OutputDrive};

#[panic_handler]
fn panic(_i: &PanicInfo) -> ! {
    loop {}
}

#[entry]
fn main() -> ! {
    let p = unsafe { embassy_nrf::Peripherals::steal() };
    let mut led4 = Output::new(p.P1_14, Level::Low, OutputDrive::Standard);

    let mut is_on = true;
    loop {
        led4.set_level(Level::from(is_on));
        for _ in 0..50_000 {
            nop();
        }
        is_on = !is_on;
    }
}
