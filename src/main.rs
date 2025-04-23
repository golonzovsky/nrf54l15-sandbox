#![no_std]
#![no_main]

use core::{panic::PanicInfo, ptr::write_volatile};
use cortex_m::asm::nop;
use cortex_m_rt::entry;
use nrf_pac as pac;

#[panic_handler]
fn panic(_i: &PanicInfo) -> ! {
    loop {}
}

#[entry]
fn main() -> ! {
    pac::P1_S
        .pin_cnf(14)
        .write(|w| w.set_dir(pac::gpio::vals::Dir::OUTPUT));

    let mut is_on = true;
    loop {
        pac::P1_S.out().write(|w| w.set_pin(14, is_on));
        for _ in 0..100_000 {
            nop();
        }
        is_on = !is_on;
    }
}
