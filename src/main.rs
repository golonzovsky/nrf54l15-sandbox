#![no_std]
#![no_main]

use core::{panic::PanicInfo, ptr::write_volatile};
use cortex_m::asm::nop;
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};

#[panic_handler]
fn panic(_i: &PanicInfo) -> ! {
    loop {}
}

#[entry]
fn main() -> ! {
    //rtt_init_print!();
    //rprintln!("hello world");

    // set p1.14 as out
    const GPIO_PIN1CNF14_ADDR: *mut u32 = 0x400D_82B8 as *mut u32;
    const DIR_OUTPUT_POS: u32 = 0;
    const PINCNF_DRIVE_LED: u32 = 1 << DIR_OUTPUT_POS;
    unsafe {
        write_volatile(GPIO_PIN1CNF14_ADDR, PINCNF_DRIVE_LED);
    }

    // write to p1.14
    const GPIO_PINCNF_P1_BASE_ADDR: *mut u32 = 0x400D_8200 as *mut u32;
    const GPIO_OUT_ADDR: *mut u32 = GPIO_PINCNF_P1_BASE_ADDR;
    const GPIO1_OUT_LED3_POS: u32 = 14;
    let mut is_on = false;
    loop {
        //rprintln!("hello world");
        unsafe {
            write_volatile(GPIO_PIN1CNF14_ADDR, (is_on as u32) << GPIO1_OUT_LED3_POS);
        }
        for _ in 0..100_000 {
            nop();
        }
        is_on = !is_on
    }
}
