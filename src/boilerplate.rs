#[lang = "eh_personality"]
extern fn eh_personality() {
}

#[lang = "panic_fmt"]
extern fn panic_fmt(details: ::core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    println!("Panic at {}:{}, {}", file, line, details);
    let row_2 = ::gpio::Pin::output(::pins::ROW_2);
    let col_3 = ::gpio::Pin::output(::pins::COL_3);
    row_2.set_high();
    loop {
        col_3.set_low();
        ::busy_loop::wait_approx_ms(5);
        col_3.set_high();
        ::busy_loop::wait_approx_ms(200);
    }
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern fn SystemInit() {}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern fn __wrap_main() {
    ::main()
}
