#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PinNumber(pub u8);

pub struct Pin {
    mask: u32,
}

impl Pin {
    pub fn new(number: PinNumber) -> Self {
        unsafe {
            (*GPIO_BASE).PIN_CNF[number.0 as usize] = GPIO_PIN_CNF_DIR_Output;
        }
        Pin {
            mask: 1 << number.0,
        }
    }

    pub fn set_high(&self) {
        unsafe {
            (*GPIO_BASE).OUTSET = self.mask
        }
    }

    pub fn set_low(&self) {
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
