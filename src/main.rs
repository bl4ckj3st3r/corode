#![no_std]
#![no_main]

mod pmp;
mod uart;
mod trickster;
mod zuse;
mod sidekernel;
mod condition;

use core::arch::asm;
use crate::sidekernel::Sidekernel;

// Tricksters Botschaft – im geschützten Vault
#[link_section = ".vault"]
static TRICKSTER_MESSAGE: &[u8] = b"\x1B[2J\x1B[H\n\r\
    *** TRICKSTER SAGT NEIN, BRO! ***\n\r\
    Zugriffsverletzung erkannt.\n\r\
    Der Vault ist versiegelt.\n\r\
    Komm besser wieder, wenn du gelernt hast.\n\r";

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    // 1. UART initialisieren (für Ausgaben)
    uart::init();
    uart::puts("Trickster Core bootet...\n");
    
    // 2. PMP-Schutzschild aktivieren
    pmp::init();
    uart::puts("PMP aktiv – alle Speicher gesperrt\n");
    
    // 3. Trap-Vektor auf Trickster setzen
    asm!("csrw mtvec, {}", in(reg) trickster::trickster_handler as usize);
    uart::puts("Trickster Trap-Handler aktiv\n");
    
    // 4. Zuse-Allokator initialisieren
    let _zuse = zuse::ZuseAllocator::new();
    uart::puts("Zuse-Allokator bereit (15 Cages à 64KB)\n");
    
    // 5. Sidekernel starten
    let _sidekernel = Sidekernel::new();
    uart::puts("Sidekernel-Layer aktiv\n");
    
    // 6. Self-Attack provozieren – Trickster freut sich
    uart::puts("Provoziere Zugriffsverletzung...\n");
    let vault_ptr = 0x80001000 as *mut u32;
    core::ptr::write_volatile(vault_ptr, 0xDEADBEEF);
    
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
