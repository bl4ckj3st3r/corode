//! uart-treiber mit polling für eingabe

use core::ptr;

const UART_BASE: usize = 0x10000000;
const UART_THR: *mut u8 = UART_BASE as *mut u8;
const UART_RHR: *mut u8 = UART_BASE as *mut u8;
const UART_LSR: *mut u8 = (UART_BASE + 5) as *mut u8;

const LSR_DR: u8 = 1 << 0;  // data ready
const LSR_THRE: u8 = 1 << 5; // transmitter empty

pub fn init() {
    // nichts zu tun für qemu
}

pub fn puts(s: &str) {
    for c in s.bytes() {
        schreibe_byte(c);
    }
}

pub fn schreibe_byte(c: u8) {
    unsafe {
        while (ptr::read_volatile(UART_LSR) & LSR_THRE) == 0 {}
        ptr::write_volatile(UART_THR, c);
    }
}

// polling: aktiv prüfen, ob taste gedrückt wurde
pub fn lese_taste_polling() -> Option<u8> {
    unsafe {
        if (ptr::read_volatile(UART_LSR) & LSR_DR) != 0 {
            Some(ptr::read_volatile(UART_RHR))
        } else {
            None
        }
    }
}
