//! Systemzustand (`CoreState`)

use crate::cond::CondMask;

/// `CoreState` kapselt den gesamten Zustand eines Prozessorkerns.
/// In unserem Fall ist es der globale Zustand des einzigen Kerns.
pub struct CoreState {
    /// Die aktuelle Bitmaske der aktiven Conditions.
    pub conds: CondMask,
    /// Ein Zähler für die Anzahl der "Epochen", d.h. der Durchläufe
    /// durch die `run_blocks`-Schleife. Nützlich für Scheduling und Timeouts.
    pub epoch: u64,
}

impl CoreState {
    /// Erstellt einen neuen, initialen `CoreState`.
    #[allow(dead_code)]
    pub const fn new() -> Self {
        Self {
            conds: 0,
            epoch: 0,
        }
    }
    
    /// Übernimmt anstehende Conditions aus Interrupt-Handlern oder anderen
    /// asynchronen Quellen in die Haupt-CondMask.
    #[allow(dead_code)]
    pub fn drain_pending(&mut self) {
        // TODO: Hier würde man atomar anstehende Ereignisse aus einer
        // globalen, von Interrupts beschreibbaren Variable übernehmen.
        self.epoch += 1;
    }
}
