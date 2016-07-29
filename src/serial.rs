use core::fmt;
use core::ptr::read_volatile;
use pins;

pub struct Serial;

impl Serial {
    pub fn init() {
        unsafe {
            pins::RX.input_pullup();
            pins::TX.output_pullup();
            (*UART0).BAUDRATE = BAUDRATE_9600_BAUD;
            (*UART0).ENABLE = UART_ENABLE;
            (*UART0).TASKS_STARTTX = 1;
            (*UART0).TASKS_STARTRX = 1;
            // Dummy write needed or TXDRDY trails write rather than leads write.
            // Pins are disconnected so nothing is physically transmitted on the wire.
            (*UART0).TXD = 0;
            (*UART0).PSELRXD = u32::from(pins::RX.0);
            (*UART0).PSELTXD = u32::from(pins::TX.0);
        }
    }

    pub fn writable() -> bool {
        unsafe {
            read_volatile(&(*UART0).EVENTS_TXDRDY) == 1
        }
    }

    pub fn readable() -> bool {
        unsafe {
            read_volatile(&(*UART0).EVENTS_RXDRDY) == 1
        }
    }

    pub fn write_byte(byte: u8) {
        while !Self::writable() {}
        unsafe {
            (*UART0).EVENTS_TXDRDY = 0;
            (*UART0).TXD = u32::from(byte);
        }
    }

    #[allow(dead_code)]
    pub fn read_byte() -> u8 {
        while !Self::readable() {}
        unsafe {
            (*UART0).EVENTS_RXDRDY = 0;
            (*UART0).RXD as u8
        }
    }
}

impl fmt::Write for Serial {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            Serial::write_byte(b)
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        use core::fmt::Write;
        write!($crate::serial::Serial, $($arg)*).unwrap()
    }
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        use core::fmt::Write;
        writeln!($crate::serial::Serial, $($arg)*).unwrap()
    }
}

const BAUDRATE_9600_BAUD: u32 = 0x00275000;
const UART_ENABLE: u32 = 4;

const UART0: *mut NRF_UART_Type = 0x40002000 as *mut _;

#[allow(non_snake_case)]
#[repr(C)]
struct NRF_UART_Type {
    TASKS_STARTRX: u32,  /* Start UART receiver. */
    TASKS_STOPRX: u32,  /* Stop UART receiver. */
    TASKS_STARTTX: u32,  /* Start UART transmitter. */
    TASKS_STOPTX: u32,  /* Stop UART transmitter. */
    RESERVED_0: [u32; 3],
    TASKS_SUSPEND: u32,  /* Suspend UART. */
    RESERVED_1: [u32; 56],
    EVENTS_CTS: u32,  /* CTS activated. */
    EVENTS_NCTS: u32,  /* CTS deactivated. */
    EVENTS_RXDRDY: u32,  /* Data received in RXD. */
    RESERVED_2: [u32; 4],
    EVENTS_TXDRDY: u32,  /* Data sent from TXD. */
    RESERVED_3: u32,
    EVENTS_ERROR: u32,  /* Error detected. */
    RESERVED_4: [u32; 7],
    EVENTS_RXTO: u32,  /* Receiver timeout. */
    RESERVED_5: [u32; 46],
    SHORTS: u32,  /* Shortcuts for UART. */
    RESERVED_6: [u32; 64],
    INTENSET: u32,  /* Interrupt enable set register. */
    INTENCLR: u32,  /* Interrupt enable clear register. */
    RESERVED_7: [u32; 93],
    ERRORSRC: u32,  /* Error source. Write error field to 1 to clear error. */
    RESERVED_8: [u32; 31],
    ENABLE: u32,  /* Enable UART and acquire IOs. */
    RESERVED_9: u32,
    PSELRTS: u32,  /* Pin select for RTS. */
    PSELTXD: u32,  /* Pin select for TXD. */
    PSELCTS: u32,  /* Pin select for CTS. */
    PSELRXD: u32,  /* Pin select for RXD. */
    RXD: u32,  /* RXD register. On read action the buffer pointer is displaced.
                          Once read the character is consumed. If read when no character
                          available, the UART will stop working. */
    TXD: u32,  /* TXD register. */
    RESERVED_10: u32,
    BAUDRATE: u32,  /* UART Baudrate. */
    RESERVED_11: [u32; 17],
    CONFIG: u32,  /* Configuration of parity and hardware flow control register. */
    RESERVED_12: [u32; 675],
    POWER: u32,  /* Peripheral power control. */
}
