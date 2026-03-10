#![no_std]
#![no_main]

mod pmp;
mod memory;
mod trickster;
mod harlekin;
mod uart;
mod block;
mod state;
mod cond;
mod trap;

use core::panic::PanicInfo;
use core::arch::asm;

// Die Adresse des Trap-Handlers aus `trap.S`. 
extern "C" {
    fn trap_entry();
}

/// Kernel-Einstieg (ersetzt kmain)
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // 1. Stack-Pointer initialisieren (ESSENTIELL!)
    unsafe {
        asm!("li sp, 0x88000000");
    }

    // 2. UART initialisieren und Boot-Nachrichten ausgeben
    uart::init();
    uart::uart_puts("\x1b[32mCorodeZ3 Online!\x1b[0m\n");      // Grün
    uart::uart_puts("\x1b[35mP1Nk H4CK3R B01 2\x1b[0m\n");     // Magenta

    // 3. PMP initialisieren
    pmp::init();
    let kernel_region = pmp::PmpRegion {
        index: 0,
        address: 0x80000000,
        size: 128 * 1024 * 1024,
        flags: pmp::PmpFlags::READ as u8 | pmp::PmpFlags::WRITE as u8 | pmp::PmpFlags::EXECUTE as u8,
    };
    pmp::set_region_napot(&kernel_region).unwrap();

    // 4. Trap-Handler aktivieren
    unsafe {
        asm!("csrw mtvec, {}", in(reg) trap_entry as *const () as usize);
    }

    uart::uart_puts("[corode] Corode läuft – Hauptschleife\n");

    // Hauptschleife
    loop {
        unsafe {
            asm!("wfi"); // Wait for Interrupt
        }
    }
}

/// Panic-Handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Versuch, eine letzte Nachricht zu senden.
    uart::uart_puts("\n\n*** KERNEL PANIC ***\n");
    // Eine einfache Schleife, da eine formatierte Ausgabe zu riskant sein könnte.
    loop {}
}
