#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PinNumber(pub u8);

impl PinNumber {
    fn mask(&self) -> u32 {
        1 << self.0
    }

    pub fn input_pullup(&self) {
        unsafe {
            (*GPIO_BASE).PIN_CNF[self.0 as usize] = CONFIG_INPUT | CONFIG_PULLUP;
        }
    }

    pub fn output_pullup(&self) {
        unsafe {
            (*GPIO_BASE).PIN_CNF[self.0 as usize] = CONFIG_OUTPUT | CONFIG_PULLUP;
        }
    }
}

pub struct Pin {
    mask: u32,
}

impl Pin {
    pub fn output(number: PinNumber) -> Self {
        unsafe {
            (*GPIO_BASE).PIN_CNF[number.0 as usize] = CONFIG_OUTPUT;
        }
        Pin {
            mask: number.mask(),
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


#[allow(non_upper_case_globals)] const CONFIG_INPUT: u32 = 0 << 0;
#[allow(non_upper_case_globals)] const CONFIG_OUTPUT: u32 = 1 << 0;
#[allow(non_upper_case_globals)] const CONFIG_PULLUP: u32 = 3 << 2;

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
