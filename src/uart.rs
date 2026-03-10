//! UART-Treiber für QEMU-Ausgaben
use core::ptr;

const UART_BASE: usize = 0x10000000;
const UART_THR: *mut u8 = UART_BASE as *mut u8;
const UART_LSR: *mut u8 = (UART_BASE + 5) as *mut u8;
const LSR_THRE: u8 = 1 << 5;

pub const ROSA: &str = "\x1B[35m";
pub const GRUEN: &str = "\x1B[32m";
pub const RESET: &str = "\x1B[0m";

pub struct UartWriter;

impl UartWriter {
    pub fn putc(&self, c: u8) {
        unsafe {
            while (ptr::read_volatile(UART_LSR) & LSR_THRE) == 0 {}
            ptr::write_volatile(UART_THR, c);
        }
    }
    
    pub fn puts(&self, s: &str) {
        for c in s.bytes() {
            self.putc(c);
            if c == b'\n' {
                self.putc(b'\r');
            }
        }
    }
}

use core::cell::UnsafeCell;

struct UartWrapper {
    writer: UnsafeCell<Option<UartWriter>>,
}

unsafe impl Sync for UartWrapper {}

static UART_WRITER: UartWrapper = UartWrapper {
    writer: UnsafeCell::new(None),
};

pub fn init() {
    unsafe {
        *UART_WRITER.writer.get() = Some(UartWriter);
    }
}

pub fn puts(s: &str) {
    unsafe {
        if let Some(writer) = &*UART_WRITER.writer.get() {
            writer.puts(s);
        }
    }
}

#[macro_export]
macro_rules! rosa {
    ($($arg:tt)*) => {
        $crate::uart::puts(concat!($crate::uart::ROSA, $($arg)*, $crate::uart::RESET, "\n"));
    };
}

#[macro_export]
macro_rules! gruen {
    ($($arg:tt)*) => {
        $crate::uart::puts(concat!($crate::uart::GRUEN, $($arg)*, $crate::uart::RESET, "\n"));
    };
}
