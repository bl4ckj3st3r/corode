//! Direkter Zugriff auf pmpcfg und pmpaddr CSRs

use core::arch::asm;

/// PMP-Konfigurationsflags
#[repr(u8)]
#[allow(dead_code)]
pub enum PmpFlags {
    OFF = 0,           // Deaktiviert
    READ = 1 << 0,     // Lesen erlaubt
    WRITE = 1 << 1,    // Schreiben erlaubt
    EXECUTE = 1 << 2,  // Ausführen erlaubt
}

/// PMP-Region (NAPOT-konfiguriert)
#[derive(Debug, Clone, Copy)]
pub struct PmpRegion {
    pub index: u8,
    pub address: usize,
    pub size: usize,
    pub flags: u8,
}

/// PMP initialisieren – ALLES sperren
pub fn init() {
    // Alle 16 PMP-Regionen deaktivieren, indem wir die Konfiguration auf 0 setzen.
    // Die Adressregister müssen nicht zwingend gelöscht werden, wenn die Konfiguration OFF ist.
    unsafe {
        // RISC-V hat pmpcfg0, pmpcfg2, pmpcfg4, ...
        // Wir löschen die ersten 4 Konfigurationsregister, die 16 Regionen abdecken.
        asm!("csrw pmpcfg0, zero");
        asm!("csrw pmpcfg2, zero");
        // Auf 64-bit Systemen werden pmpcfg1, 3 etc. für die oberen 32 bits der Adresse verwendet.
        // Für RV64 sind pmpcfg0 und pmpcfg2 ausreichend für 16 Regionen.
    }
}

/// PMP-Region im NAPOT-Modus setzen
pub fn set_region_napot(region: &PmpRegion) -> Result<(), &'static str> {
    if region.index > 15 {
        return Err("PMP index out of range");
    }
    
    // Größe muss 2er-Potenz sein und mindestens 4 Bytes (nicht 8)
    if !region.size.is_power_of_two() || region.size < 4 {
        return Err("Size must be power of two >= 4");
    }
    
    // NAPOT-Adresse berechnen: base | (size/2 - 1)
    // Die Adresse muss für NAPOT rechts um 2 geshiftet werden.
    let napot_addr = (region.address >> 2) | (region.size - 1) >> 3;
    
    unsafe {
        // pmpaddr Register setzen
        match region.index {
            0 => asm!("csrw pmpaddr0, {}", in(reg) napot_addr),
            1 => asm!("csrw pmpaddr1, {}", in(reg) napot_addr),
            2 => asm!("csrw pmpaddr2, {}", in(reg) napot_addr),
            3 => asm!("csrw pmpaddr3, {}", in(reg) napot_addr),
            4 => asm!("csrw pmpaddr4, {}", in(reg) napot_addr),
            5 => asm!("csrw pmpaddr5, {}", in(reg) napot_addr),
            6 => asm!("csrw pmpaddr6, {}", in(reg) napot_addr),
            7 => asm!("csrw pmpaddr7, {}", in(reg) napot_addr),
            8 => asm!("csrw pmpaddr8, {}", in(reg) napot_addr),
            9 => asm!("csrw pmpaddr9, {}", in(reg) napot_addr),
            10 => asm!("csrw pmpaddr10, {}", in(reg) napot_addr),
            11 => asm!("csrw pmpaddr11, {}", in(reg) napot_addr),
            12 => asm!("csrw pmpaddr12, {}", in(reg) napot_addr),
            13 => asm!("csrw pmpaddr13, {}", in(reg) napot_addr),
            14 => asm!("csrw pmpaddr14, {}", in(reg) napot_addr),
            15 => asm!("csrw pmpaddr15, {}", in(reg) napot_addr),
            _ => unreachable!(),
        }
        
        // pmpcfg Register setzen (NAPOT + Flags)
        let cfg_value = (3 << 3) | region.flags; // Bit 3,4 = NAPOT
        set_pmp_cfg(region.index, cfg_value);

        // Ensure PMP changes are synchronized
        asm!("sfence.vma");
    }
    
    Ok(())
}

/// pmpcfg Register für einen Index setzen
fn set_pmp_cfg(index: u8, value: u8) {
    unsafe {
        let cfg_reg_num = index / 8; // pmpcfg0, pmpcfg2, etc.
        let cfg_shift = (index % 8) * 8;

        match cfg_reg_num {
            0 => { // pmpcfg0
                let mut cfg: usize;
                asm!("csrr {}, pmpcfg0", out(reg) cfg);
                cfg = (cfg & !(0xFF << cfg_shift)) | ((value as usize) << cfg_shift);
                asm!("csrw pmpcfg0, {}", in(reg) cfg);
            }
            1 => { // pmpcfg2
                let mut cfg: usize;
                asm!("csrr {}, pmpcfg2", out(reg) cfg);
                cfg = (cfg & !(0xFF << cfg_shift)) | ((value as usize) << cfg_shift);
                asm!("csrw pmpcfg2, {}", in(reg) cfg);
            }
            _ => unreachable!(), // Wir unterstützen nur 16 Regionen
        }
    }
}

#[allow(dead_code)]
pub fn clear_region(index: u8) {
    if index > 15 {
        return;
    }
    set_pmp_cfg(index, 0);
}
