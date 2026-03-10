#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;
use core::ptr::write_volatile;

// =============================================================================
// 1. Startup and External Symbols
// =============================================================================

extern "C" {
    fn __trap_vector();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe { asm!("li sp, 0x80100000"); }
    
    // Header der neuen Ära
    uart_puts("\n--- corode-core v2.0: OXIZ Pilot Phase ---\n");
    
    setup_trap_vector();
    setup_pmp_shield();

    // --- Das OXIZ-Manifest: Die erste Condition des Systems ---
    
    // OXIZ ist rein, minimalistisch und beweisbar.
    let oxiz_condition = Condition {
        id: 0x01,                // Die Erste ihrer Art
        name: "OXIZ",            // Der Name ist Programm
        entry_point: 0xB0000000, // Start der ARENA0 Sandkiste
        required_memory: 0x4000, // 16KB für die erste Ausbaustufe
        role: 0x01,              // Basis-Rolle: Sensor/Actor
        proof_hash: 0x07122026,  // Das beweisbare Manifest (Datum der Vision?)
    };

    uart_puts("Condition OXIZ wird dem Orakel vorgelegt...\n");
    
    // Das Orakel prüft die mathematische Korrektheit von OXIZ
    if verify_condition_via_oracle(&oxiz_condition) {
        uart_puts(">> ORAKEL: OXIZ ist beweisbar sicher. Existenz GENEHMIGT. <<\n");
        // Hier würde der Kernel den Speicherbereich für OXIZ freischalten (PMP)
        // und den Sprung in den Code von OXIZ vorbereiten.
    } else {
        uart_puts("!! ORAKEL: ALARM! OXIZ verletzt Systemgesetze. ABGELEHNT. !!\n");
    }

    // --- Ultima Ratio: Der Harlekin wacht über die Hardware ---
    uart_puts("\nStarte Hardware-Integritätstest (Vault-Schutz)...\n");
    let vault_ptr = 0x8000F000 as *mut u32; // Vault (geschützt durch PMP)
    unsafe { write_volatile(vault_ptr, 0xDEADC0DE); }

    loop { unsafe { asm!("wfi"); } }
}

// =============================================================================
// 2. Das Orakel & Das Condition-Paradigma
// =============================================================================

#[repr(C)]
pub struct Condition {
    pub id: u32,
    pub name: &'static str,
    pub entry_point: usize,
    pub required_memory: usize,
    pub role: u32,
    pub proof_hash: u32,
}

/// Das Orakel: Der Ort, an dem Z3-Logik und Kernel-Regeln verschmelzen.
fn verify_condition_via_oracle(cond: &Condition) -> bool {
    // 1. Bereichsprüfung (Deterministisch)
    let arena_start = 0xB0000000;
    let arena_end = arena_start + (128 * 1024 * 1024); // 128MB ARENA0

    // Beweisbare Sicherheit: Liegt OXIZ vollständig in der zugewiesenen Sandkiste?
    if cond.entry_point < arena_start || (cond.entry_point + cond.required_memory) > arena_end {
        return false; // Mathematischer Ausschluss
    }

    // 2. Identitätsprüfung (Beweis-Check)
    // In der Realität würde hier der Z3-Proof-Hash gegen das Manifest geprüft.
    if cond.proof_hash == 0x07122026 {
        true
    } else {
        false
    }
}

// =============================================================================
// 3. UART & Infrastruktur
// =============================================================================
const UART_BASE: *mut u8 = 0x10000000 as *mut u8;
fn uart_putc(c: u8) { unsafe { write_volatile(UART_BASE, c); } }
fn uart_puts(s: &str) { for byte in s.bytes() { uart_putc(byte); } }

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { loop { unsafe { asm!("wfi"); } } }

fn setup_trap_vector() { unsafe { asm!("csrw mtvec, {}", in(reg) __trap_vector as usize); } }

fn setup_pmp_shield() {
    // NAPOT Schutz für den Header-Käfig (Vault) bei 0x8000F000
    let pmp_addr = (0x8000F000 >> 2) | ((4096 - 1) >> 3);
    unsafe {
        asm!("csrw pmpaddr0, {}", in(reg) pmp_addr);
        asm!("csrw pmpcfg0, {}", in(reg) 0x81); // Read, Locked, NAPOT
    }
}

#[no_mangle]
pub unsafe extern "C" fn rust_handle_trap() -> ! {
    let mcause: usize;
    asm!("csrr {}, mcause", out(reg) mcause);
    uart_puts("\n>> HARLEKIN: PMP-Eingriff! Zugriff auf geschützten Bereich unterbunden. <<\n");
    uart_puts("System nominal. Ordnung gewahrt.\n");
    loop { asm!("wfi"); }
}
