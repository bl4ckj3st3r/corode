// src/trap.rs
// Bindet die Assembler-Trap-Handler ein
use core::arch::global_asm;

global_asm!(include_str!("trap.S"));
