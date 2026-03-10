//! 15 feste Cages à 64KB, jede mit eigener PMP-Region

use crate::pmp::{self, PmpRegion, PmpFlags};
use core::cell::UnsafeCell;

/// Ein Cage: 64KB fester, isolierter Speicherblock
#[derive(Debug, Clone, Copy)]
pub struct Cage {
    #[allow(dead_code)]
    pub id: u8,
    pub base_addr: usize, // Renamed from base to be more descriptive
    pub size: usize,
    pub in_use: bool,
}

impl Cage {
    pub const fn new(id: u8, base_addr: usize) -> Self {
        Self {
            id,
            base_addr,
            size: 65536, // 64KB fix
            in_use: false,
        }
    }
}

/// Zuse-Allokator – interior mutability für &self-Methoden
pub struct ZuseAllocator {
    cages: UnsafeCell<[Cage; 15]>,
    free_mask: UnsafeCell<u16>,  // Bit 0-14 = Cage 0-14 frei?
}

// Um ZUSE als globale statische Variable zu verwenden, muss Sync implementiert sein.
// Da unser Kernel single-threaded ist, können wir dies sicher tun.
// UnsafeCell ist nicht Sync, daher müssen wir diese manuelle Implementierung vornehmen.
unsafe impl Sync for ZuseAllocator {}

impl ZuseAllocator {
    pub const fn new() -> Self {
        const CAGES_BASE_ADDR: usize = 0x90000000; // Startadresse für Cages
        const CAGE_SIZE: usize = 65536;

        // Die Initialisierung von const-Arrays ist in Rust etwas umständlich.
        // Wir müssen es manuell tun, anstatt eine Schleife zu verwenden.
        let cages = [
            Cage::new(0, CAGES_BASE_ADDR + 0 * CAGE_SIZE),
            Cage::new(1, CAGES_BASE_ADDR + 1 * CAGE_SIZE),
            Cage::new(2, CAGES_BASE_ADDR + 2 * CAGE_SIZE),
            Cage::new(3, CAGES_BASE_ADDR + 3 * CAGE_SIZE),
            Cage::new(4, CAGES_BASE_ADDR + 4 * CAGE_SIZE),
            Cage::new(5, CAGES_BASE_ADDR + 5 * CAGE_SIZE),
            Cage::new(6, CAGES_BASE_ADDR + 6 * CAGE_SIZE),
            Cage::new(7, CAGES_BASE_ADDR + 7 * CAGE_SIZE),
            Cage::new(8, CAGES_BASE_ADDR + 8 * CAGE_SIZE),
            Cage::new(9, CAGES_BASE_ADDR + 9 * CAGE_SIZE),
            Cage::new(10, CAGES_BASE_ADDR + 10 * CAGE_SIZE),
            Cage::new(11, CAGES_BASE_ADDR + 11 * CAGE_SIZE),
            Cage::new(12, CAGES_BASE_ADDR + 12 * CAGE_SIZE),
            Cage::new(13, CAGES_BASE_ADDR + 13 * CAGE_SIZE),
            Cage::new(14, CAGES_BASE_ADDR + 14 * CAGE_SIZE),
        ];
        
        Self {
            cages: UnsafeCell::new(cages),
            free_mask: UnsafeCell::new(0x7FFF), // Alle 15 Cages frei (Bits 0-14)
        }
    }
    
    /// Allokiere einen freien Cage
    pub fn allocate_cage(&self) -> Option<Cage> {
        let free_mask_ptr = self.free_mask.get();
        let current_mask = unsafe { *free_mask_ptr };

        if current_mask == 0 {
            return None; // Kein freier Cage
        }
        
        // Finde das niedrigste gesetzte Bit (den ersten freien Cage)
        let id = current_mask.trailing_zeros() as u8;
        if id >= 15 {
            return None; // Sollte nicht passieren, wenn current_mask != 0
        }
        
        let cages_ptr = self.cages.get();

        unsafe {
            // Cage als belegt markieren
            (*cages_ptr)[id as usize].in_use = true;
            *free_mask_ptr &= !(1 << id);
        
            // PMP-Region für diesen Cage konfigurieren
            let cage = (*cages_ptr)[id as usize];
            let pmp_region = PmpRegion {
                index: id + 1, // PMP-Region 0 ist für den Kernel reserviert
                address: cage.base_addr,
                size: cage.size,
                flags: PmpFlags::READ as u8 | PmpFlags::WRITE as u8,
            };
            
            // PMP in Hardware setzen (Fehler wird hier ignoriert)
            let _ = pmp::set_region_napot(&pmp_region);
            
            Some(cage)
        }
    }
    
    /// Cage freigeben
    #[allow(dead_code)]
    pub fn deallocate_cage(&self, id: u8) -> bool {
        if id >= 15 {
            return false;
        }

        let cages_ptr = self.cages.get();
        let free_mask_ptr = self.free_mask.get();

        unsafe {
            if !(*cages_ptr)[id as usize].in_use {
                return false; // War schon frei
            }

            // PMP-Region deaktivieren
            // Wichtig: Index ist id + 1
            pmp::clear_region(id + 1);

            // Cage freigeben
            (*cages_ptr)[id as usize].in_use = false;
            *free_mask_ptr |= 1 << id;
        }

        true
    }
    
    /// Status aller Cages abfragen
    #[allow(dead_code)]
    pub fn status(&self) -> [(u8, bool); 15] {
        let mut result = [(0, false); 15];
        let cages = unsafe { &*self.cages.get() };
        for i in 0..15 {
            result[i] = (i as u8, cages[i].in_use);
        }
        result
    }
}

// Globale, statische Instanz des Zuse-Allokators.
// Diese ist der einzige Weg, um auf den Speicher zuzugreifen.
pub static ZUSE: ZuseAllocator = ZuseAllocator::new();
