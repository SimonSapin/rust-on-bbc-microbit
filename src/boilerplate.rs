#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! { loop {} }

#[allow(non_snake_case)]
#[no_mangle]
pub extern fn SystemInit() {}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern fn __wrap_main() {
    ::main()
}
