//! Blocks – Code, der auf bestimmte Conditions reagiert

use crate::cond::CondMask;
use crate::state::CoreState;

/// Ein `Block` ist eine Funktion, die ausgeführt wird, wenn eine bestimmte
/// Kombination von Conditions (`mask`) erfüllt ist.
pub struct Block {
    #[allow(dead_code)]
    pub name: &'static str,
    pub mask: CondMask,
    pub func: fn(&mut CoreState),
}

/// Eine globale Liste aller Blöcke im System.
/// In einem echten System würde diese Liste dynamisch verwaltet.
pub const BLOCKS: &[Block] = &[
    // -- Beispiel-Block --
    // Block {
    //     name: "Handle Timer Tick",
    //     mask: crate::cond::TIMER_TICK,
    //     func: |state| {
    //         // Logik, die bei einem Timer-Tick ausgeführt wird.
    //         state.conds &= !crate::cond::TIMER_TICK; // Condition als "verbraucht" markieren
    //     }
    // },
];

/// Führt alle Blöcke aus, deren `mask` im aktuellen Zustand `state.conds` erfüllt ist.
#[allow(dead_code)]
pub fn run_blocks(state: &mut CoreState) {
    for block in BLOCKS {
        // Prüfe, ob alle Bits der Maske in den Conditions gesetzt sind.
        if (state.conds & block.mask) == block.mask {
            (block.func)(state);
        }
    }
}
