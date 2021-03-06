#![no_std]
#![no_main]

extern crate actinius_icarus_bsp as bsp;
extern crate cortex_m_rt as rt;
extern crate nb;
extern crate panic_semihosting;

use bsp::{hal::Timer, prelude::*, Board};
use core::fmt::Write;
use nb::block;
use rt::entry;

#[entry]
fn main() -> ! {
    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0_NS);

    writeln!(board.cdc_uart, "Hello, world!").unwrap();

    let mut led_is_on = false;
    loop {
        if led_is_on {
            board.leds.red.disable();
        } else {
            board.leds.red.enable();
        }
        timer.start(1_000_000_u32);
        block!(timer.wait()).unwrap();
        led_is_on = !led_is_on;
    }
}
