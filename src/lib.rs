#![no_std]
#![feature(lang_items, asm)]

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! { loop {} }

#[allow(non_snake_case)]
#[no_mangle]
pub extern fn SystemInit() {}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern fn __wrap_main() {
    main()
}

const PERIOD_MS: u32 = 1000;
const ON_MS: u32 = 50;

#[no_mangle]
pub unsafe extern fn main() -> ! {
    let row_2 = Pin::new(14);
    let col_3 = Pin::new(6);
    row_2.set_high();
    loop {
        col_3.set_low();
        wait_approx_ms(ON_MS);
        col_3.set_high();
        wait_approx_ms(PERIOD_MS - ON_MS);
    }
}

struct Pin {
    mask: u32,
}

impl Pin {
    fn new(number: usize) -> Self {
        unsafe {
            (*GPIO_BASE).PIN_CNF[number] = GPIO_PIN_CNF_DIR_Output;
        }
        Pin {
            mask: 1 << number,
        }
    }

    fn set_high(&self) {
        unsafe {
            (*GPIO_BASE).OUTSET = self.mask
        }
    }

    fn set_low(&self) {
        unsafe {
            (*GPIO_BASE).OUTCLR = self.mask
        }
    }
}


#[allow(non_upper_case_globals)]
const GPIO_PIN_CNF_DIR_Output: u32 = 1;

const GPIO_BASE: *mut NRF_GPIO_Type = 0x50000000 as *mut _;

#[allow(non_snake_case)]
#[repr(C)]
struct NRF_GPIO_Type {
    RESERVED_0: [u32; 321],
    OUT: u32,                               /* Write GPIO port. */
    OUTSET: u32,                            /* Set individual bits in GPIO port. */
    OUTCLR: u32,                            /* Clear individual bits in GPIO port. */
    IN: u32,                                /* Read GPIO port. */
    DIR: u32,                               /* Direction of GPIO pins. */
    DIRSET: u32,                            /* DIR set register. */
    DIRCLR: u32,                            /* DIR clear register. */
    RESERVED_1: [u32; 120],
    PIN_CNF: [u32; 32],                       /* Configuration of GPIO pins. */
}

fn wait_approx_ms(ms: u32) {
    for i in 0..ms {
        for j in 0..1500 {
            black_box((i, j));
        }
    }
}

fn black_box<T>(dummy: T) -> T {
    // we need to "use" the argument in some way LLVM can't
    // introspect.
    unsafe { asm!("" : : "r"(&dummy)) }
    dummy
}
