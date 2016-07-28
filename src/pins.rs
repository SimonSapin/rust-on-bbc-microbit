//! BBC micro:bit pins.
//!
//! Constant names: what a pin is connected to.
//! Constant value: pin number in the Nordic micro-controller’s memory mapping,
//! for use with `gpio::Pin::new`.

#![allow(dead_code)]

use gpio::PinNumber;

// On the edge connector
pub const P0: PinNumber = PinNumber(3);
pub const P1: PinNumber = PinNumber(2);
pub const P2: PinNumber = PinNumber(1);
pub const P3: PinNumber = PinNumber(4);  // COL_1 (ANALOG/DIGITAL)
pub const P4: PinNumber = PinNumber(5);  // COL_2 (ANALOG/DIGITAL)
pub const P5: PinNumber = PinNumber(17);  // BTN_A
pub const P6: PinNumber = PinNumber(12);  // COL_9
pub const P7: PinNumber = PinNumber(11);  // COL_8
pub const P8: PinNumber = PinNumber(18);
pub const P9: PinNumber = PinNumber(10);  // COL_7
pub const P10: PinNumber = PinNumber(6);  // COL_3 (ANALOG/DIGITAL)
pub const P11: PinNumber = PinNumber(26);  // BTN_B
pub const P12: PinNumber = PinNumber(20);
pub const P13: PinNumber = PinNumber(23);  // SCK
pub const P14: PinNumber = PinNumber(22);  // MISO
pub const P15: PinNumber = PinNumber(21);  // MOSI
pub const P16: PinNumber = PinNumber(16);
pub const P19: PinNumber = PinNumber(0);  // SCL
pub const P20: PinNumber = PinNumber(30);  // SDA

// Display
pub const COL_1: PinNumber = PinNumber(4);
pub const COL_2: PinNumber = PinNumber(5);
pub const COL_3: PinNumber = PinNumber(6);
pub const COL_4: PinNumber = PinNumber(7);
pub const COL_5: PinNumber = PinNumber(8);
pub const COL_6: PinNumber = PinNumber(9);
pub const COL_7: PinNumber = PinNumber(10);
pub const COL_8: PinNumber = PinNumber(11);
pub const COL_9: PinNumber = PinNumber(12);
pub const ROW_1: PinNumber = PinNumber(13);
pub const ROW_2: PinNumber = PinNumber(14);
pub const ROW_3: PinNumber = PinNumber(15);

pub const BUTTON_A: PinNumber = PinNumber(17);
pub const BUTTON_B: PinNumber = PinNumber(26);

pub const TARGET_NRESET: PinNumber = PinNumber(19);

pub const SPI_MOSI: PinNumber = PinNumber(21); // MASTER OUT SLAVE IN
pub const SPI_MISO: PinNumber = PinNumber(22); // MASTER IN SLAVE OUT
pub const SPI_SCK: PinNumber = PinNumber(23); // SERIAL CLOCK

pub const TX: PinNumber = PinNumber(24);
pub const RX: PinNumber = PinNumber(25);

// ACCELLEROMETER INTERRUPT PINS (MMA8653FC)
pub const ACCEL_INT2: PinNumber = PinNumber(27);
pub const ACCEL_INT1: PinNumber = PinNumber(28);

pub const MAG_INT1: PinNumber = PinNumber(29); // MAGENETOMETER INTERRUPT PIN (MAG3110)

pub const I2C_SDA0: PinNumber = PinNumber(30); // SDA (SERIAL DATA LINE)
pub const I2C_SCL0: PinNumber = PinNumber(0); //SCL (SERIAL CLOCK LINE)
