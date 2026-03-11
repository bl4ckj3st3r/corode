use core::arch::asm;

const PMP_REGIONEN: usize = 16;

// Sperrt eine Speicherregion für den M-Mode
const LOCK: u8 = 1 << 7;
// Adress-Matching: TOR (Top of Range)
const A_TOR: u8 = 1 << 3;
// Berechtigungen
const X: u8 = 1 << 0;
const W: u8 = 1 << 1;
const R: u8 = 1 << 2;

/// Konfiguriert die Physical Memory Protection (PMP)
pub fn init() {
    // Deaktiviere alle PMP-Regionen, um Konflikte zu vermeiden
    unsafe {
        asm!("csrw pmpaddr0, zero");
        asm!("csrw pmpcfg0, zero");
    }

    // Region 0: [0x0000_0000, 0x2000_0000) - RAM, R/W/X
    set_pmp_region(0, 0x2000_0000, R | W | X, A_TOR, false);

    // Region 1: [0x8000_0000, 0x8000_1000) - UART, R/W
    set_pmp_region(1, 0x8000_1000, R | W, A_TOR, false);
    
    // Region 2: Der Vault [0x80001000, 0x80002000) - M-Mode R, U-Mode No Access
    set_pmp_region(2, 0x80002000, R, A_TOR, true); 
}

/// Konfiguriert eine einzelne PMP-Region
fn set_pmp_region(index: usize, addr: usize, perms: u8, a_field: u8, locked: bool) {
    if index >= PMP_REGIONEN { return; }
    
    let pmpaddr = addr >> 2;
    let mut pmpcfg = perms | a_field;
    if locked { pmpcfg |= LOCK; }

    unsafe {
        match index {
            0 => asm!("csrw pmpaddr0, {}", in(reg) pmpaddr),
            1 => asm!("csrw pmpaddr1, {}", in(reg) pmpaddr),
            2 => asm!("csrw pmpaddr2, {}", in(reg) pmpaddr),
            // ... weitere für alle 16 Regionen
            _ => {}
        }

        // pmpcfg-Register werden pro 4/8 Regionen gepackt (je nach Architektur)
        // Hier vereinfacht für cfg0
        if index < 4 { // Für RV32, 8-bit cfg pro Region
            let shift = index * 8;
            let mask = !(0xFF << shift);
            let mut current_cfg0: usize;
            asm!("csrr {}, pmpcfg0", out(reg) current_cfg0);
            current_cfg0 = (current_cfg0 & mask) | ((pmpcfg as usize) << shift);
            asm!("csrw pmpcfg0, {}", in(reg) current_cfg0);
        }
    }
}

pub fn liste_anzeigen(ausgabe: impl Fn(&str)) {
    ausgabe("  region 0: 0x00000000-0x1fffffff (rwx)\n");
    ausgabe("  region 1: 0x80000000-0x80000fff (rw-)\n");
    ausgabe("  region 2: 0x80001000-0x80001fff (r--)\n");
    ausgabe("  region 3: nicht konfiguriert\n");
    ausgabe("  region 4: nicht konfiguriert\n");
    ausgabe("  region 5: nicht konfiguriert\n");
    ausgabe("  region 6: nicht konfiguriert\n");
    ausgabe("  region 7: nicht konfiguriert\n");
    ausgabe("  region 8: nicht konfiguriert\n");
    ausgabe("  region 9: nicht konfiguriert\n");
    ausgabe("  region 10: nicht konfiguriert\n");
    ausgabe("  region 11: nicht konfiguriert\n");
    ausgabe("  region 12: nicht konfiguriert\n");
    ausgabe("  region 13: nicht konfiguriert\n");
    ausgabe("  region 14: nicht konfiguriert\n");
    ausgabe("  region 15: nicht konfiguriert\n");
}
