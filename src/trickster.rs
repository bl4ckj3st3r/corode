//! Trickster - Der gerissene Logger
//! Protokolliert Traps und andere Ereignisse in einem globalen Ringpuffer.

use core::cell::UnsafeCell;
use core::fmt::Write;
use core::sync::atomic::{AtomicUsize, Ordering};
use crate::uart;

/// Maximale Anzahl von Log-Einträgen im Ringpuffer.
const MAX_LOG_ENTRIES: usize = 256;

/// Ein einzelner Log-Eintrag.
/// `repr(C, packed(1))` sorgt für ein dichtes Layout im Speicher, ohne Padding.
/// Das ist wichtig für die rohe Datenextraktion, erfordert aber sorgfältigen Zugriff.
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct LogEntry {
    pub timestamp: u64,       // Zeitstempel (z.B. aus `rdcycle`)
    pub condition_id: u64,    // ID der Condition, die den Trap ausgelöst hat
    pub trap_cause: u32,      // Ursachencode des Traps (aus `mcause`)
    pub _padding: u32,        // Füllbytes, um auf 64-Bit-Grenze zu bleiben
    pub program_counter: usize, // Wo der Trap aufgetreten ist (aus `mepc`)
    pub accessed_address: usize, // Zugehörige Adresse (aus `mtval`)
}

impl LogEntry {
    /// Erstellt einen leeren Log-Eintrag in einem `const`-Kontext.
    pub const fn new() -> Self {
        Self {
            timestamp: 0,
            condition_id: 0,
            trap_cause: 0,
            _padding: 0,
            program_counter: 0,
            accessed_address: 0,
        }
    }
}

/// Ein konstanter, leerer Log-Eintrag, der für die Initialisierung verwendet wird.
const EMPTY_LOG_ENTRY: LogEntry = LogEntry::new();

/// Die globale Logger-Struktur.
/// Enthält einen Ringpuffer für Log-Einträge.
#[repr(C)]
struct TricksterLogger {
    buffer: UnsafeCell<[LogEntry; MAX_LOG_ENTRIES]>,
    write_index: UnsafeCell<AtomicUsize>,
}

#[no_mangle]
static GLOBAL_LOGGER: TricksterLogger = TricksterLogger {
    buffer: UnsafeCell::new([EMPTY_LOG_ENTRY; MAX_LOG_ENTRIES]),
    write_index: UnsafeCell::new(AtomicUsize::new(0)),
};

unsafe impl Sync for TricksterLogger {}

/// Protokolliert einen Trap.
pub fn log_trap(condition_id: u64, cause: u32, pc: usize, addr: usize) {
    let logger_ptr = &GLOBAL_LOGGER as *const TricksterLogger;
    
    let write_index_ptr = unsafe { (*logger_ptr).write_index.get() };
    let index = unsafe { (*write_index_ptr).fetch_add(1, Ordering::SeqCst) % MAX_LOG_ENTRIES };

    let timestamp: u64;
    unsafe { core::arch::asm!("rdcycle {}", out(reg) timestamp); }

    let entry = LogEntry {
        timestamp,
        condition_id,
        trap_cause: cause,
        _padding: 0,
        program_counter: pc,
        accessed_address: addr,
    };

    let buffer_ptr = unsafe { (*logger_ptr).buffer.get() };
    unsafe {
        (*buffer_ptr).as_mut_ptr().add(index).write(entry);
    }
}

/// Gibt den gesamten Inhalt des Log-Puffers auf der UART aus.
pub fn dump_log() {
    uart::uart_puts("\n--- TRICKSTER LOG DUMP ---\n");
    let logger_ptr = &GLOBAL_LOGGER as *const TricksterLogger;
    let buffer_ptr = unsafe { (*logger_ptr).buffer.get() };
    let write_index_ptr = unsafe { (*logger_ptr).write_index.get() };
    let current_index = unsafe { (*write_index_ptr).load(Ordering::Relaxed) };

    let start_index = current_index % MAX_LOG_ENTRIES;

    for i in 0..MAX_LOG_ENTRIES {
        let index = (start_index + i) % MAX_LOG_ENTRIES;
        let entry_ptr = unsafe { (*buffer_ptr).as_ptr().add(index) };
        
        let entry = unsafe { core::ptr::read_volatile(entry_ptr) };

        if entry.timestamp == 0 {
            continue;
        }
        
        let mut buf = [0u8; 256];
        let mut writer = uart::BufferWriter::new(&mut buf);

        // Die Feldwerte müssen in separate lokale Variablen kopiert werden,
        // bevor sie an `write!` übergeben werden, um unaligned reference errors zu vermeiden.
        let timestamp = entry.timestamp;
        let condition_id = entry.condition_id;
        let trap_cause = entry.trap_cause;
        let program_counter = entry.program_counter;
        let accessed_address = entry.accessed_address;

        let _ = write!(
            writer,
            "[T:{: >10}] ID:{: >3} Cause:0x{:04X} PC:0x{:08X} Addr:0x{:08X}\n",
            timestamp,
            condition_id,
            trap_cause,
            program_counter,
            accessed_address
        );
        uart::uart_puts(writer.as_str());
    }
    uart::uart_puts("--- END OF LOG ---\n");
}
