'''
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
            7. => asm!("csrw pmpaddr7, {}", in(reg) val),
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
''