#![no_std]
#![no_main]

// Module deklarieren
mod pmp;
mod uart;
mod trickster;
mod zuse;
mod sidekernel;
mod condition;
mod terminal;

use core::arch::asm;
use crate::terminal::system_terminal::SystemTerminal;

#[link_section = ".vault"]
static TRICKSTER_MESSAGE: &[u8] = b"\x1B[2J\x1B[H\n\rTRICKSTER SAGT NEIN, BRO!\n\r";

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    // 1. Init
    uart::init();
    pmp::init();
    asm!("csrw mtvec, {}", in(reg) trickster::trickster_handler as usize);

    // 2. System-Terminal starten
    let mut terminal = SystemTerminal::neue();
    terminal.ausführen();

    // Diese Schleife wird durch terminal.ausführen() ersetzt
    // loop {
    //     unsafe { core::arch::asm!("wfi"); }
    // }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    uart::puts("\n\n*** KERNEL PANIC ***\n");
    if let Some(location) = info.location() {
        uart::puts("Panic in ");
        uart::puts(location.file());
        uart::puts(":");
        // TODO: integer to string conversion for line number
        // uart::puts(location.line()); 
        uart::puts("\n");
        // uart::puts(info.message().unwrap()); // needs a formatter
    } else {
        uart::puts("Panic, aber keine genaue Location.\n");
    }
    loop {}
}
