'''
// src/memory/mod.rs

pub mod vector_alloc;

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
''