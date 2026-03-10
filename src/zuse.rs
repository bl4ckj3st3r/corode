//! Zuse-Allokator – benannt nach Konrad Zuse, dem Erfinder des ersten Computers.
//!
//! Dieser Allokator verwaltet einen festen Satz von Speicherblöcken ("Cages"),
//! die für isolierte Sidekernel-Komponenten verwendet werden können.

const NUM_CAGES: usize = 15;
const CAGE_SIZE: usize = 65536; // 64KB

#[derive(Debug, Clone, Copy)]
pub struct Cage {
    pub id: u8,
    pub base: usize,
    pub in_use: bool,
}

pub struct ZuseAllocator {
    cages: [Cage; NUM_CAGES],
    free_mask: u16,
}

impl ZuseAllocator {
    /// Initialisiert den Allokator mit 15 freien Cages.
    pub fn new() -> Self {
        let mut cages = [Cage { id: 0, base: 0, in_use: false }; NUM_CAGES];
        for i in 0..NUM_CAGES {
            cages[i] = Cage {
                id: i as u8,
                base: 0x80200000 + (i * CAGE_SIZE),
                in_use: false,
            };
        }

        Self {
            cages,
            free_mask: 0x7FFF, // 15 Cages frei (111111111111111b)
        }
    }

    /// Findet und belegt die erste freie Cage.
    pub fn allocate(&mut self) -> Option<&mut Cage> {
        if self.free_mask == 0 {
            return None; // Kein freier Cage
        }

        let id = self.free_mask.trailing_zeros() as usize;
        if id >= NUM_CAGES {
            return None; // Sollte nicht passieren
        }

        self.free_mask &= !(1 << id);
        self.cages[id].in_use = true;
        Some(&mut self.cages[id])
    }

    /// Gibt eine Cage wieder frei.
    pub fn deallocate(&mut self, id: u8) {
        if id < NUM_CAGES as u8 {
            self.free_mask |= 1 << id;
            self.cages[id as usize].in_use = false;
        }
    }

    /// Gibt den aktuellen Status aller Cages zurück.
    pub fn status(&self) -> &[Cage; NUM_CAGES] {
        &self.cages
    }
}
