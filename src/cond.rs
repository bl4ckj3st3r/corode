//! CondMask – Systemzustand als Bitmaske

// Eine CondMask ist eine 64-Bit-Bitmaske, die den Zustand des Systems darstellt.
// Jedes Bit repräsentiert eine "Condition", d.h. ein Ereignis, das eingetreten ist.
pub type CondMask = u64;

// -- Beispiel-Conditions --

/// Die PMP-Einheit wurde erfolgreich initialisiert.
#[allow(dead_code)]
pub const PMP_OK: CondMask = 1 << 0;

/// Ein Timer-Tick ist aufgetreten.
#[allow(dead_code)]
pub const TIMER_TICK: CondMask = 1 << 1;

// Hier können und werden viele weitere Conditions definiert, z.B. für
// abgeschlossene DMA-Transfers, Netzwerk-Events, etc.
