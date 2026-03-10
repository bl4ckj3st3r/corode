
//! trickster.rs - Ein sicherer Ringbuffer-Logger in Rust

use crate::uart;
use core::fmt::{self, Write};

// Ein einfacher Ringbuffer für Log-Einträge
const MAX_LOG_ENTRIES: usize = 16;

#[derive(Clone, Copy)]
pub struct LogEntry {
    pub message: [u8; 64],
    pub length: usize,
}

impl LogEntry {
    pub fn new(msg: &str) -> Self {
        let mut message = [0u8; 64];
        let bytes = msg.as_bytes();
        let length = if bytes.len() > 64 { 64 } else { bytes.len() };
        message[..length].copy_from_slice(&bytes[..length]);
        Self { message, length }
    }

    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.message[..self.length]).unwrap_or("Invalid UTF-8")
    }
}


pub struct TricksterLogger {
    entries: [Option<LogEntry>; MAX_LOG_ENTRIES],
    head: usize, // Zeigt auf den nächsten freien Slot
}

impl TricksterLogger {
    pub const fn new() -> Self {
        Self {
            entries: [None; MAX_LOG_ENTRIES],
            head: 0,
        }
    }

    // Diese Funktion sollte nur vom Trap-Handler aufgerufen werden.
    pub fn log(&mut self, message: &str) {
        let entry = LogEntry::new(message);
        self.entries[self.head] = Some(entry);
        self.head = (self.head + 1) % MAX_LOG_ENTRIES;
    }

    // Bietet Read-Only-Zugriff auf die Log-Einträge
    pub fn read_entries<F>(&self, mut f: F) where F: FnMut(&LogEntry) {
        for i in 0..MAX_LOG_ENTRIES {
            let index = (self.head + i) % MAX_LOG_ENTRIES;
            if let Some(ref entry) = self.entries[index] {
                f(entry);
            }
        }
    }
}

// Globale Instanz des Loggers
// In einem echten Bare-Metal-Szenario würde man hier typischerweise einen Mutex
// oder eine andere Art von Sperre verwenden, um den Zugriff zu synchronisieren.
// Für die Einfachheit verwenden wir hier `unsafe`, um globale statische Variablen
// zu verändern.
static mut GLOBAL_LOGGER: TricksterLogger = TricksterLogger::new();

/// Schreibt eine Log-Nachricht in den globalen Logger.
/// Dies ist die primäre Funktion, die vom Trap-Handler verwendet wird.
/// ## Safety
/// Diese Funktion ist `unsafe`, da sie auf eine globale, veränderliche statische
/// Variable zugreift. Der Aufrufer muss sicherstellen, dass keine Race Conditions
/// auftreten. Im Kontext von `corode-core` wird angenommen, dass nur der
/// Trap-Handler schreibt, was das Risiko minimiert.
pub unsafe fn log_trap(message: &str) {
    GLOBAL_LOGGER.log(message);
}

/// Prints all log entries to the UART console.
/// ## Safety
/// This function is `unsafe` because it reads from a global, mutable static
/// variable. The caller must ensure that no other thread is writing to the
/// logger at the same time.
pub unsafe fn print_logs() {
    uart::uart_puts("\n--- Trickster Log ---\n");
    GLOBAL_LOGGER.read_entries(|entry| {
        uart::uart_puts(entry.as_str());
        uart::uart_puts("\n");
    });
    uart::uart_puts("--- End Log ---\n");
}

