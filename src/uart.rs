//! Einfacher UART-Treiber für Debug-Ausgaben (QEMU virt)

use core::ptr;
use core::fmt::{self, Write};

const UART_BASE: usize = 0x10000000; // Basisadresse für QEMU virt-Maschine
const UART_THR: *mut u8 = (UART_BASE + 0) as *mut u8; // Transmitter Holding Register
const UART_LSR: *mut u8 = (UART_BASE + 5) as *mut u8; // Line Status Register
const LSR_THRE: u8 = 1 << 5; // Transmitter Holding Register Empty

/// Initialisiert die UART (für QEMU ist hier nichts zu tun).
pub fn init() {
    // Bei echter Hardware müsste hier die Baudrate etc. konfiguriert werden.
}

/// Schreibt ein einzelnes Byte an die UART.
pub fn uart_putc(c: u8) {
    unsafe {
        // Warte, bis das Senderegister leer ist.
        while (ptr::read_volatile(UART_LSR) & LSR_THRE) == 0 {}
        // Schreibe das Byte in das Senderegister.
        ptr::write_volatile(UART_THR, c);
    }
}

/// Schreibt einen String an die UART.
pub fn uart_puts(s: &str) {
    for byte in s.bytes() {
        // Füge nach jedem Newline ein Carriage Return hinzu, wie es Terminals erwarten.
        if byte == b'\n' {
            uart_putc(b'\r');
        }
        uart_putc(byte);
    }
}

// --- Hilfskonstrukt für das `write!`-Makro --- 

/// Ein Writer, der formatierte Argumente direkt in die UART schreibt.
#[allow(dead_code)]
pub struct UartWriter;

impl Write for UartWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        uart_puts(s);
        Ok(())
    }
}

/// Ein Writer, der in einen Puffer schreibt, nützlich für `panic`.
pub struct BufferWriter<'a> {
    buffer: &'a mut [u8],
    cursor: usize,
}

impl<'a> BufferWriter<'a> {
    pub fn new(buffer: &'a mut [u8]) -> Self {
        BufferWriter { buffer, cursor: 0 }
    }

    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.buffer[..self.cursor]).unwrap_or("")
    }
}

impl<'a> Write for BufferWriter<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let bytes = s.as_bytes();
        let remaining = &mut self.buffer[self.cursor..];
        let len = core::cmp::min(bytes.len(), remaining.len());
        
        remaining[..len].copy_from_slice(&bytes[..len]);
        self.cursor += len;
        
        Ok(())
    }
}
