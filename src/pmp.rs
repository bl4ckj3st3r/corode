//! Physical Memory Protection – Hardware-Isolation
use core::arch::asm;

pub const PMP_READ: u8 = 1 << 0;
pub const PMP_WRITE: u8 = 1 << 1;
pub const PMP_EXEC: u8 = 1 << 2;
pub const PMP_NAPOT: u8 = 1 << 3;
pub const PMP_LOCK: u8 = 1 << 7;

pub struct PmpRegion {
    pub index: u8,
    pub start: usize,
    pub end: usize,
    pub flags: u8,
}

pub fn init() {
    // Alle 16 PMP-Regionen sperren
    for i in 0..16 {
        set_region(i, 0, 0, 0);
    }
}

pub fn set_region(index: u8, start: usize, end: usize, flags: u8) {
    unsafe {
        if index < 16 {
            // NAPOT-Adresse berechnen
            let napot = if end > start {
                start | ((end - start) >> 1) - 1
            } else {
                0
            };
            
            // pmpaddr Register setzen
            match index {
                0 => asm!("csrw pmpaddr0, {}", in(reg) napot),
                1 => asm!("csrw pmpaddr1, {}", in(reg) napot),
                2 => asm!("csrw pmpaddr2, {}", in(reg) napot),
                3 => asm!("csrw pmpaddr3, {}", in(reg) napot),
                4 => asm!("csrw pmpaddr4, {}", in(reg) napot),
                5 => asm!("csrw pmpaddr5, {}", in(reg) napot),
                6 => asm!("csrw pmpaddr6, {}", in(reg) napot),
                7 => asm!("csrw pmpaddr7, {}", in(reg) napot),
                8 => asm!("csrw pmpaddr8, {}", in(reg) napot),
                9 => asm!("csrw pmpaddr9, {}", in(reg) napot),
                10 => asm!("csrw pmpaddr10, {}", in(reg) napot),
                11 => asm!("csrw pmpaddr11, {}", in(reg) napot),
                12 => asm!("csrw pmpaddr12, {}", in(reg) napot),
                13 => asm!("csrw pmpaddr13, {}", in(reg) napot),
                14 => asm!("csrw pmpaddr14, {}", in(reg) napot),
                15 => asm!("csrw pmpaddr15, {}", in(reg) napot),
                _ => {}
            }
            
            // pmpcfg Register setzen
            let cfg = PMP_NAPOT | flags;
            set_pmpcfg(index, cfg);
        }
    }
}

fn set_pmpcfg(index: u8, value: u8) {
    unsafe {
        let reg = index / 4;
        let shift = (index % 4) * 8;
        
        match reg {
            0 => {
                let mut cfg: u32;
                asm!("csrr {}, pmpcfg0", out(reg) cfg);
                cfg = (cfg & !(0xFF << shift)) | ((value as u32) << shift);
                asm!("csrw pmpcfg0, {}", in(reg) cfg);
            }
            1 => {
                let mut cfg: u32;
                asm!("csrr {}, pmpcfg2", out(reg) cfg);
                cfg = (cfg & !(0xFF << shift)) | ((value as u32) << shift);
                asm!("csrw pmpcfg2, {}", in(reg) cfg);
            }
            2 => {
                let mut cfg: u32;
                asm!("csrr {}, pmpcfg4", out(reg) cfg);
                cfg = (cfg & !(0xFF << shift)) | ((value as u32) << shift);
                asm!("csrw pmpcfg4, {}", in(reg) cfg);
            }
            3 => {
                let mut cfg: u32;
                asm!("csrr {}, pmpcfg6", out(reg) cfg);
                cfg = (cfg & !(0xFF << shift)) | ((value as u32) << shift);
                asm!("csrw pmpcfg6, {}", in(reg) cfg);
            }
            _ => {}
        }
    }
}
