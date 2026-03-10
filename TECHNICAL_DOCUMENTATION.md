## Corode: Technische Code-Dokumentation (Kernel)

### Dateistruktur

Die Kernel-Implementierung basiert auf den folgenden Dateien. Low-Level-Boot-Code (`trap.S`) und die Speicher-Layout-Definition (`linker.ld`) existieren, werden hier aber nicht gezeigt.

```
.
├── src/
│   ├── main.rs           # Kernel-Einstiegspunkt und Hauptschleife
│   ├── pmp.rs            # PMP Hardware-Abstraktion
│   ├── memory/
│   │   └── mod.rs        # "Zuse" Vektor-Speicher-Allokator
│   ├── harlekin.rs       # Trap-Handler
│   ├── trickster.rs      # Ringbuffer-Logger für Traps
│   ├── block.rs          # (Nicht im Fokus dieser Doku)
│   ├── cond.rs           # (Nicht im Fokus dieser Doku)
│   ├── state.rs          # (Nicht im Fokus dieser Doku)
│   └── uart.rs           # (Nicht im Fokus dieser Doku)
├── trap.S                # Low-level trap entry point
└── linker.ld             # Linker-Skript
```

### 1. `src/pmp.rs`: Physische Speicherprotektion

Dies ist die direkte Abstraktion über die RISC-V PMP-Kontrollregister (`pmpcfg`, `pmpaddr`). Die Implementierung nutzt `asm!`-Blöcke, um die CSRs direkt zu manipulieren und eine physische Speicher-Firewall zu konfigurieren.

*   `init()`: Versetzt das System beim Start in einen "alles gesperrt"-Zustand, indem alle PMP-Konfigurationen gelöscht werden.
*   `set_pmp_napot()`: Konfiguriert eine PMP-Region unter Verwendung des "Naturally Aligned Power-of-Two"-Modus.
*   `disable_region()`: Deaktiviert eine spezifische PMP-Region und macht den zugehörigen Speicherbereich unzugänglich.

```rust
// src/pmp.rs

use core::arch::asm;

// Definiert die PMP-Konfigurationsflags
pub const READ: u8 = 1 << 0;
pub const WRITE: u8 = 1 << 1;
pub const EXEC: u8 = 1 << 2;

// Definiert die Adress-Matching-Modi
pub const OFF: u8 = 0; // Null region (disabled)
pub const TOR: u8 = 1; // Top of range
pub const NA4: u8 = 2; // Naturally aligned four-byte region
pub const NAPOT: u8 = 3; // Naturally aligned power-of-two region

/// Schreibt in ein PMP-Konfigurationsregister (pmpcfg).
#[inline]
fn write_pmp_cfg(reg_idx: usize, val: u64) {
    unsafe {
        match reg_idx {
            0 => asm!("csrw pmpcfg0, {}", in(reg) val),
            2 => asm!("csrw pmpcfg2, {}", in(reg) val),
            _ => (),
        }
    }
}

/// Schreibt in ein PMP-Adressregister (pmpaddr).
#[inline]
fn write_pmp_addr(reg_idx: usize, val: u64) {
    unsafe {
        match reg_idx {
            0 => asm!("csrw pmpaddr0, {}", in(reg) val),
            1 => asm!("csrw pmpaddr1, {}", in(reg) val),
            2 => asm!("csrw pmpaddr2, {}", in(reg) val),
            3 => asm!("csrw pmpaddr3, {}", in(reg) val),
            4 => asm!("csrw pmpaddr4, {}", in(reg) val),
            5 => asm!("csrw pmpaddr5, {}", in(reg) val),
            6 => asm!("csrw pmpaddr6, {}", in(reg) val),
            7 => asm!("csrw pmpaddr7, {}", in(reg) val),
            8 => asm!("csrw pmpaddr8, {}", in(reg) val),
            9 => asm!("csrw pmpaddr9, {}", in(reg) val),
            10 => asm!("csrw pmpaddr10, {}", in(reg) val),
            11 => asm!("csrw pmpaddr11, {}", in(reg) val),
            12 => asm!("csrw pmpaddr12, {}", in(reg) val),
            13 => asm!("csrw pmpaddr13, {}", in(reg) val),
            14 => asm!("csrw pmpaddr14, {}", in(reg) val),
            15 => asm!("csrw pmpaddr15, {}", in(reg) val),
            _ => (),
        }
    }
}

/// Konfiguriert eine einzelne PMP-Region mit NAPOT.
pub fn set_pmp_napot(region: usize, base_addr: u64, size: u64, permissions: u8) {
    if region > 15 || size == 0 || !size.is_power_of_two() {
        return;
    }

    let napot_addr = base_addr | (size.wrapping_sub(1) >> 1);
    write_pmp_addr(region, napot_addr >> 2);

    let mut current_cfg: u64;
    let cfg_reg_idx = if region < 8 { 0 } else { 2 };
    let cfg_byte_shift = (region % 8) * 8;

    unsafe {
        match cfg_reg_idx {
            0 => asm!("csrr {}, pmpcfg0", out(reg) current_cfg),
            2 => asm!("csrr {}, pmpcfg2", out(reg) current_cfg),
            _ => return,
        }
    }
    
    let cfg_mask = !((0xFF as u64) << cfg_byte_shift);
    let new_cfg_byte = ((permissions | (NAPOT << 3)) as u64) << cfg_byte_shift;
    let new_cfg = (current_cfg & cfg_mask) | new_cfg_byte;

    write_pmp_cfg(cfg_reg_idx, new_cfg);
}

/// Deaktiviert eine einzelne PMP-Region.
pub fn disable_region(region: usize) {
    if region > 15 {
        return;
    }

    let mut current_cfg: u64;
    let cfg_reg_idx = if region < 8 { 0 } else { 2 };
    let cfg_byte_shift = (region % 8) * 8;

    unsafe {
        match cfg_reg_idx {
            0 => asm!("csrr {}, pmpcfg0", out(reg) current_cfg),
            2 => asm!("csrr {}, pmpcfg2", out(reg) current_cfg),
            _ => return,
        }
    }
    
    let cfg_mask = !((0xFF as u64) << cfg_byte_shift);
    let new_cfg = current_cfg & cfg_mask;

    write_pmp_cfg(cfg_reg_idx, new_cfg);
    write_pmp_addr(region, 0);

    unsafe {
        asm!("sfence.vma");
    }
}


/// Initialisiert alle PMP-Regionen in einem sicheren, gesperrten Zustand.
pub fn init() {
    write_pmp_cfg(0, 0);
    write_pmp_cfg(2, 0);
    for i in 0..16 {
        write_pmp_addr(i, 0);
    }
    unsafe {
        asm!("sfence.vma");
    }
}
```

### 2. `src/memory/mod.rs`: "Zuse" Vektor-Speicher-Allokator

`Zuse` ersetzt einen traditionellen Heap-Allokator. Er verwaltet einen festen Pool von 15 Speicherblöcken ("Cages"), die jeweils 64KB groß sind.

*   `alloc()`: Sucht nach einem freien `Cage`. Findet er einen, markiert er ihn als belegt, aktiviert via `pmp.rs` die zugehörige PMP-Region (1-15) mit vollen Zugriffsrechten und gibt die Basisadresse des `Cages` zurück. Das angefragte `Layout` wird ignoriert; es wird immer ein ganzer `Cage` alloziert.
*   `dealloc()`: Findet den `Cage`, der zur übergebenen Adresse gehört, deaktiviert dessen PMP-Region via `pmp::disable_region` (macht den Speicher unzugänglich) und markiert den `Cage` als wieder frei.

```rust
// src/memory/mod.rs

use crate::pmp;
use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

// --- ZUSE: Deterministisches Vektor-Speicher-Management ---

// Anzahl der verfügbaren Speicher-Käfige. Wir reservieren 15, da PMP-Region 0 für den Kernel ist.
const NUM_CAGES: usize = 15;

// Größe jedes Käfigs: 64 KB. Muss eine Potenz von 2 sein.
const CAGE_SIZE: usize = 64 * 1024;

// Die Basisadresse, ab der die Käfige beginnen.
const CAGES_BASE_ADDR: u64 = 0x90000000;

// Der globale Allokator wird unser ZuseAllocator sein.
#[global_allocator]
static ALLOCATOR: ZuseAllocator = ZuseAllocator::new();

/// Initialisiert den globalen Zuse-Allokator.
/// Muss einmalig beim Systemstart aufgerufen werden.
pub fn init() {
    // Diese `unsafe` Operation ist notwendig, um die `init`-Methode auf dem
    // statischen Allokator aufzurufen. Sie ist sicher, weil wir sie hier,
    // am Anfang des Kernel-Hauptprogramms, einmalig und ohne konkurrierende
    // Zugriffe ausführen.
    unsafe {
        let allocator = &mut *(&ALLOCATOR as *const ZuseAllocator as *mut ZuseAllocator);
        allocator.init();
    }
}

// Repräsentiert einen einzelnen Speicher-Käfig.
struct Cage {
    base_addr: u64, // Startadresse des Käfigs
    pmp_region: usize,  // Zugeordnete PMP-Region (1-15)
    is_allocated: bool, // Ist der Käfig belegt?
}

// Der "Zuse" Block-Allokator
pub struct ZuseAllocator {
    cages: [Cage; NUM_CAGES],
}

impl ZuseAllocator {
    pub const fn new() -> Self {
        // Erzeugt die Liste der Käfige zur Compile-Zeit.
        let mut cages = [Cage::new_const(0,0); NUM_CAGES];
        let mut i = 0;
        while i < NUM_CAGES {
            cages[i] = Cage::new_const(
                CAGES_BASE_ADDR + (i * CAGE_SIZE) as u64,
                i + 1 // PMP-Regionen 1 bis 15
            );
            i += 1;
        }
        Self { cages }
    }

    pub fn init(&mut self) {
        // Initialisiert den Zustand der Käfige beim Systemstart.
        for i in 0..NUM_CAGES {
            self.cages[i].is_allocated = false;
            // WICHTIG: Sicherstellen, dass alle PMP-Regionen anfangs gesperrt sind.
            pmp::disable_region(self.cages[i].pmp_region);
        }
    }
}

impl Cage {
    const fn new_const(base_addr: u64, pmp_region: usize) -> Self {
        Self {
            base_addr,
            pmp_region,
            is_allocated: false,
        }
    }
}

unsafe impl GlobalAlloc for ZuseAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        let self_mut = &mut *(self as *const Self as *mut Self);

        for i in 0..NUM_CAGES {
            if !self_mut.cages[i].is_allocated {
                self_mut.cages[i].is_allocated = true;
                pmp::set_pmp_napot(
                    self_mut.cages[i].pmp_region,
                    self_mut.cages[i].base_addr,
                    CAGE_SIZE as u64,
                    pmp::READ | pmp::WRITE | pmp::EXEC,
                );
                return self_mut.cages[i].base_addr as *mut u8;
            }
        }
        null_mut()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        let addr = ptr as u64;
        let self_mut = &mut *(self as *const Self as *mut Self);

        for i in 0..NUM_CAGES {
            if self_mut.cages[i].base_addr == addr {
                pmp::disable_region(self_mut.cages[i].pmp_region);
                self_mut.cages[i].is_allocated = false;
                return;
            }
        }
    }
}
```

### 3. `src/trickster.rs`: Ringbuffer-Logger

Der `Trickster` ist ein einfacher, statischer Ringbuffer für Diagnose-Logs. Er ist für Situationen gedacht, in denen normale Ausgaben (wie UART) nicht sicher oder möglich sind, insbesondere innerhalb eines Trap-Handlers.

*   `LogEntry`: Enthält die Trap-Ursache (`mcause`), die verursachende Adresse (`mtval`) und einen Zeitstempel.
*   `TRICKSTER_LOG`: Ein statisches, veränderliches Array, das als Ringbuffer dient. Der Zugriff darauf ist per Definition unsicher und sollte nur in atomaren Kontexten (wie einem Trap) erfolgen.
*   `log_trap()`: Die Funktion, die vom Trap-Handler aufgerufen wird, um einen Fehler zu protokollieren.

```rust
// src/trickster.rs

use crate::uart::uart_puts;

const LOG_CAPACITY: usize = 16;

#[derive(Clone, Copy)]
pub struct LogEntry {
    mcause: u64,
    mtval: u64,
    timestamp: u64, // Simpler Zähler, keine echte Zeit
}

impl LogEntry {
    pub fn new(mcause: u64, mtval: u64) -> Self {
        Self {
            mcause,
            mtval,
            timestamp: 0, // In einer echten Implementierung würde hier eine Zeitquelle stehen
        }
    }
}

// Der globale, statische Ringbuffer für Trap-Logs.
// ACHTUNG: Der Zugriff ist unsicher und muss sorgfältig synchronisiert werden.
// In unserem Fall wird er nur innerhalb des atomaren Trap-Handlers verwendet.
static mut TRICKSTER_LOG: [Option<LogEntry>; LOG_CAPACITY] = [None; LOG_CAPACITY];
static mut LOG_INDEX: usize = 0;

/// Protokolliert einen Trap-Eintrag im globalen Ringbuffer.
pub fn log_trap(entry: LogEntry) {
    unsafe {
        TRICKSTER_LOG[LOG_INDEX] = Some(entry);
        LOG_INDEX = (LOG_INDEX + 1) % LOG_CAPACITY;
    }
}

/// Gibt alle aktuellen Log-Einträge über UART aus.
pub fn print_logs() {
    uart_puts("--- Trickster Log ---\
");
    unsafe {
        for i in 0..LOG_CAPACITY {
            let index = (LOG_INDEX + i) % LOG_CAPACITY; // Startet beim ältesten Eintrag
            if let Some(entry) = &TRICKSTER_LOG[index] {
                // Hier würde man die Log-Daten formatiert ausgeben
                uart_puts("Log Entry: mcause=");
                // ... (Ausgabe-Code für u64)
                uart_puts("\
");
            }
        }
    }
    uart_puts("---------------------\
");
}
```

### 4. `src/harlekin.rs`: Rust Trap-Handler

`Harlekin` ist der High-Level Trap-Handler, der von der Low-Level-Assembler-Routine in `trap.S` aufgerufen wird.

*   `handle_trap()`: Liest die Trap-Ursache (`mcause`) und die fehlerverursachende Adresse (`mtval`). Bei PMP-Zugriffsfehlern (Codes 5, 7) wird der Fehler mithilfe des `Trickster`-Loggers protokolliert. In einem erweiterten System würde hier die verursachende `Condition` (Task/Prozess) identifiziert und eingefroren.

```rust
// src/harlekin.rs

use crate::trickster;
use crate::uart::uart_puts;

/// Der High-Level Trap Handler.
/// Wird von der Low-Level Assembler-Routine aufgerufen.
#[no_mangle]
pub extern "C" fn handle_trap(mcause: u64, mtval: u64) {
    // Exception Codes für RISC-V
    let is_load_access_fault = mcause == 5;
    let is_store_access_fault = mcause == 7;

    if is_load_access_fault || is_store_access_fault {
        // Ein PMP-Fehler ist aufgetreten.
        // 1. Protokolliere den Fehler mit dem Trickster.
        let log_entry = trickster::LogEntry::new(mcause, mtval);
        trickster::log_trap(log_entry);

        // 2. In einem vollständigen System:
        //    - Identifiziere die "Condition" (Task/Prozess), die den Fehler verursacht hat.
        //    - Setze den Zustand dieser Condition auf "Frozen" oder "Terminated".
        //    - Löse einen Scheduling-Vorgang aus.
        uart_puts("[Harlekin] PMP Fault logged. Condition would be frozen.\
");
        
    } else {
        // Andere Fehlerarten behandeln
        uart_puts("[Harlekin] Unhandled Trap! mcause: ");
        // ... (Ausgabe für mcause)
    }

    // Hier würde die Kontrolle an den Scheduler zurückgegeben.
    // In diesem simplen System kehren wir einfach zurück.
}
```

### 5. `src/main.rs`: Kernel-Einstiegspunkt

`kmain` ist der Startpunkt des Rust-Codes. Die Initialisierungssequenz ist entscheidend für die Etablierung der Systemarchitektur:

1.  `pmp::init()`: Versiegelt das gesamte System.
2.  `pmp::set_pmp_napot(...)`: Gewährt dem Kernel selbst eine 1GB große PMP-Region (Region 0) zum Operieren.
3.  `memory::init()`: Initialisiert den `ZuseAllocator`, der seinerseits alle PMP-Regionen (1-15) für die `Cages` deaktiviert und in einen sauberen, freien Zustand versetzt.

```rust
// Auszug aus src/main.rs

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    // Bringe die PMP in einen deterministischen, sicheren Zustand.
    pmp::init();
    uart_puts("\n[corode] PMP initialized. All regions locked.\n");

    // Definiere die Lebensgrundlage des Kernels: 1GB RAM mit vollem Zugriff.
    pmp::set_pmp_napot(
        0, 
        0x80000000, 
        1024 * 1024 * 1024, 
        pmp::READ | pmp::WRITE | pmp::EXEC
    );
    uart_puts("[corode] PMP Region 0 (Kernel-Space) configured.\n");

    // Initialisiere die deterministische Vektor-Speicher-Architektur "Zuse".
    memory::init();
    uart_puts("[corode] Zuse vector memory initialized.\n");

    let mut state = CoreState::new();
    state.set_cond(id::PMP_OK);
    
    uart_puts("Willkommen bei Corode. Das Orakel erwartet deine Eingabe.\n");
    uart_puts("> ");
    
    let mut tick_counter = 0;

    loop {
        handle_terminal_input();

        tick_counter += 1;
        if tick_counter > 500000 {
            state.set_cond(id::TIMER_TICK);
            tick_counter = 0;
        }

        state.drain_pending_conds();
        block::run_blocks(block::BLOCKS, &mut state);
    }
}
```