#![no_std]
#![feature(lang_items, asm)]

pub mod boilerplate;  // Public to export symbols to the linker
mod gpio;
mod busy_loop;

const PERIOD_MS: u32 = 1000;
const ON_MS: u32 = 50;

#[no_mangle]
pub unsafe extern fn main() -> ! {
    let row_2 = gpio::Pin::new(14);
    let col_3 = gpio::Pin::new(6);
    row_2.set_high();
    loop {
        col_3.set_low();
        busy_loop::wait_approx_ms(ON_MS);
        col_3.set_high();
        busy_loop::wait_approx_ms(PERIOD_MS - ON_MS);
    }
}
