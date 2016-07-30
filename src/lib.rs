#![no_std]
#![feature(lang_items, asm)]

#[macro_use] mod serial;
pub mod boilerplate;  // Public to export symbols to the linker
mod gpio;
mod busy_loop;
mod pins;

const PERIOD_MS: u32 = 1000;
const ON_MS: u32 = 50;

#[no_mangle]
pub unsafe extern fn main() -> ! {
    serial::Serial::init();
    let row_2 = gpio::Pin::output(pins::ROW_2);
    let col_3 = gpio::Pin::output(pins::COL_3);
    row_2.set_high();
    let mut uptime: u32 = 0;
    loop {
        println!("Uptime: {}", uptime);
        uptime += 1;
        col_3.set_low();
        busy_loop::wait_approx_ms(ON_MS);
        col_3.set_high();
        busy_loop::wait_approx_ms(PERIOD_MS - ON_MS);
    }
}
