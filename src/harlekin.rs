
//! harlekin.rs - Der Rust-Trap-Handler

use crate::trickster;

/// # Safety
///
/// This function is marked as `unsafe` because it directly interacts with the `trickster::log_trap` function, 
/// which operates on a mutable global static variable. The caller must ensure that this function is called 
/// in a context where data races are not a concern. In the `corode-core` design, this function is intended 
/// to be called exclusively by the trap-handling mechanism, which naturally serializes its execution, 
/// thus minimizing the risk of concurrent access issues.
pub unsafe fn handle_trap(cause: usize, pc: usize) {
    let cause_str = match cause {
        0 => "Instruction address misaligned",
        1 => "Instruction access fault",
        2 => "Illegal instruction",
        3 => "Breakpoint",
        4 => "Load address misaligned",
        5 => "Load access fault",
        6 => "Store/AMO address misaligned",
        7 => "Store/AMO access fault",
        8 => "Environment call from U-mode",
        9 => "Environment call from S-mode",
        11 => "Environment call from M-mode",
        12 => "Instruction page fault",
        13 => "Load page fault",
        15 => "Store/AMO page fault",
        _ => "Unknown trap cause",
    };

    // Format the log message
    let mut message_buf = [0u8; 64];
    let message = core::format_args!("PMP Fault @ {:#x}: {}", pc, cause_str);
    
    // This is a simplified way to write the formatted message to the buffer.
    // A more robust solution would handle potential overflows.
    let mut writer = &mut message_buf[..];
    let _ = core::fmt::write(&mut writer, message);
    let len = 64 - writer.len();
    let final_message = core::str::from_utf8(&message_buf[..len]).unwrap_or("Log format error");

    // Log the trap using the Trickster-Logger
    trickster::log_trap(final_message);

    // In a real scenario, we would freeze the condition that caused the trap.
    // For now, we just log the event.
}
