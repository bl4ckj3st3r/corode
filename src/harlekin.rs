//! Harlekin – Zentraler Trap-Handler in purem Rust
//! Fängt alle Traps ab, loggt, friert Conditions ein

use crate::trickster;

/// Trap-Frame (vom Assembler übergeben)
/// Diese Struktur muss exakt dem auf dem Stack abgelegten Kontext
/// in `trap.S` entsprechen.
#[repr(C)]
pub struct TrapFrame {
    pub ra: usize,
    pub sp: usize,
    pub gp: usize,
    pub tp: usize,
    pub t0: usize,
    pub t1: usize,
    pub t2: usize,
    pub s0: usize,
    pub s1: usize,
    pub a0: usize,
    pub a1: usize,
    pub a2: usize,
    pub a3: usize,
    pub a4: usize,
    pub a5: usize,
    pub a6: usize,
    pub a7: usize,
    pub s2: usize,
    pub s3: usize,
    pub s4: usize,
    pub s5: usize,
    pub s6: usize,
    pub s7: usize,
    pub s8: usize,
    pub s9: usize,
    pub s10: usize,
    pub s11: usize,
    pub t3: usize,
    pub t4: usize,
    pub t5: usize,
    pub t6: usize,
    pub mcause: usize,
    pub mepc: usize,
    pub mtval: usize,
    pub mstatus: usize, // mstatus muss am Ende sein, um es einfach wiederherzustellen
}

/// Haupt-Trap-Handler (wird von trap_entry in trap.S aufgerufen)
#[no_mangle]
pub extern "C" fn rust_trap_handler(frame: &mut TrapFrame) {
    // mcause auslesen (oberstes Bit = Interrupt, untere Bits = Cause)
    let is_interrupt = (frame.mcause & (1 << (core::mem::size_of::<usize>() * 8 - 1))) != 0;
    let cause_code = frame.mcause & 0x7FF;

    if is_interrupt {
        // Interrupts werden hier noch nicht behandelt.
        // Später könnte hier der Timer-Interrupt behandelt werden.
    } else {
        // Es war eine Exception
        match cause_code {
            // PMP-Fehler (Load/Store/Instruction Access Fault)
            1 | 5 | 7 => {
                handle_pmp_fault(frame);
            }
            // Illegal Instruction
            2 => {
                handle_illegal_instruction(frame);
            }
            // Alles andere
            _ => {
                handle_unknown_trap(frame);
            }
        }
    }
}

/// Behandelt einen PMP-Fehler.
fn handle_pmp_fault(frame: &TrapFrame) {
    // mtval enthält die Adresse, die den Fehler verursacht hat.
    let fault_addr = frame.mtval;
    
    // TODO: Die Condition-ID muss aus dem Kontext des laufenden Codes ermittelt werden.
    // Vorerst verwenden wir einen Platzhalter.
    let condition_id = 0; 
    
    // Logge den Fehler mit dem Trickster-Logger.
    trickster::log_trap(
        condition_id,
        frame.mcause as u32,
        frame.mepc,
        fault_addr,
    );
    
    // Friert die fehlerverursachende Condition ein.
    freeze_condition(condition_id);
    
    // Der PC wird in `trap.S` *nicht* erhöht. Der fehlerhafte Code wird
    // in einer Endlosschleife gefangen, was das Debugging erleichtert.
}

/// Behandelt eine illegale Instruktion.
fn handle_illegal_instruction(frame: &TrapFrame) {
    trickster::log_trap(
        0, // Condition ID unbekannt
        frame.mcause as u32,
        frame.mepc,
        frame.mtval, // mtval kann die fehlerhafte Instruktion selbst enthalten
    );
    freeze_condition(0);
}

/// Behandelt unbekannte Traps.
fn handle_unknown_trap(frame: &TrapFrame) {
    trickster::log_trap(
        0, // Condition ID unbekannt
        frame.mcause as u32,
        frame.mepc,
        frame.mtval,
    );
    freeze_condition(0);
}

/// Friert eine Condition ein, um weitere Ausführung zu verhindern.
fn freeze_condition(condition_id: u64) {
    // TODO: Hier muss die Logik implementiert werden, um die Condition
    // in der `CoreState`-Maschine als "frozen" zu markieren.
    
    // Wir loggen das Freeze-Ereignis selbst.
    trickster::log_trap(
        condition_id,
        0xFFFF, // Spezieller Code für "Condition Freeze"
        0,      // PC nicht relevant
        0,      // Adresse nicht relevant
    );
}
