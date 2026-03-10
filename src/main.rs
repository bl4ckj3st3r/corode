#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

mod block;
mod cond;
mod pmp;
mod state;
mod trap;
mod uart;
mod trickster;
mod harlekin;
mod memory;

use core::panic::PanicInfo;
use crate::uart::{uart_puts, uart_getc, set_pink_mode};
use crate::state::CoreState;
use crate::cond::id;
use alloc::string::String;

// Statischer Puffer für die Terminal-Eingabe
const CMD_BUFFER_SIZE: usize = 128;
static mut CMD_BUFFER: [u8; CMD_BUFFER_SIZE] = [0; CMD_BUFFER_SIZE];
static mut CMD_LEN: usize = 0;

static mut PANIC_COUNT: u32 = 0;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let harlekin_gesichter = [
        "O_o", // 1. Panik
        "o_O", // 2. Panik
        "X_x", // 3. Panik
        "x_X", // 4. Panik
        "T_T", // 5. Panik und danach
    ];

    unsafe {
        let gesicht_index = if PANIC_COUNT < harlekin_gesichter.len() as u32 {
            PANIC_COUNT as usize
        } else {
            harlekin_gesichter.len() - 1
        };

        uart_puts("\n\n         ╭─────────────────────╮\n         │  🤡                 │\n         │   _/\\_/            │\n         │    (");
        uart_puts(harlekin_gesichter[gesicht_index]);
        uart_puts(")            │\n         │   > ^ <             │\n         │  /       \\          │\n         │ │ HARLEKIN │        │\n         │ │  SAGT    │        │\n         │ │  NEIN,   │        │\n         │ │   BRO!   │        │\n         │  \\_______/         │\n         ╰─────────────────────╯\n\n");

        PANIC_COUNT += 1;
    }

    if let Some(location) = info.location() {
        uart_puts("    Location: ");
        uart_puts(location.file());
        uart_puts("\n");
    }
    loop {}
}

#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}


fn handle_terminal_input() {
    if let Some(c) = uart_getc() {
        unsafe {
            match c {
                13 => { // Enter (carriage return)
                    uart_puts("\n");
                    if CMD_LEN > 0 {
                        let cmd = core::str::from_utf8_unchecked(&CMD_BUFFER[0..CMD_LEN]);
                        
                        // --- EASTER EGG TISCH ---
                        if cmd == "orakel" {
                            uart_puts("Das Orakel schweigt. Die Bedingungen sind noch nicht reif.\n");
                        } else if cmd == "4711" {
                            uart_puts("Eine Bedingung, die noch nicht gestellt wurde.\n");
                        } else if cmd == "xyzzy" { // Klassiker
                            uart_puts("Nichts passiert hier.\n
                        } else if cmd == "oma" {
                            uart_puts("Oma ist stolz auf dich.\n");
                        } else if cmd == "philosoph" {
                            uart_puts("Wer mit Ungeheuern kaempft, mag zusehn, dass er nicht dabei zum Ungeheuer wird.\n");
                        } else if cmd == "hacker" {
                            uart_puts("The Pink Hacker Boy says: Race conditions are dead - change my mind.\n");
                        } else if cmd == "pink-jogger-mode-on" {
                            set_pink_mode(true);
                            uart_puts("Pink Jogger Mode activated.\n");
                        } else if cmd == "pink-jogger-mode-off" {
                            set_pink_mode(false);
                            uart_puts("Pink Jogger Mode deactivated.\n");
                        } else if cmd == "test_trap" {
                            uart_puts("Testing trap handler...\n");
                            // Simulate a Load access fault (cause = 5) at PC 0x12345678
                            harlekin::handle_trap(5, 0x12345678);
                            trickster::print_logs();
                        } else {
                             // Normales Echo für unbekannte Befehle
                            uart_puts("Echo: ");
                            uart_puts(cmd);
                            uart_puts("\n");
                        }

                        CMD_LEN = 0; // Puffer zurücksetzen
                    }
                    uart_puts("> ");
                },
                127 | 8 => { // Backspace / Delete
                    if CMD_LEN > 0 {
                        CMD_LEN -= 1;
                        uart_puts("\x08 \x08"); // Rückwärts, überschreiben, rückwärts
                    }
                },
                _ => { // Normales Zeichen
                    if CMD_LEN < CMD_BUFFER_SIZE - 1 {
                        let c_as_slice = &[c];
                        let c_as_str = core::str::from_utf8_unchecked(c_as_slice);
                        CMD_BUFFER[CMD_LEN] = c;
                        CMD_LEN += 1;
                        // Echo des Zeichens zurück an den Benutzer
                        uart::uart_puts(c_as_str);
                    }
                }
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    uart_puts("\n[corode] kmain enter\n");

    // PMP-Setup
    pmp::set_pmp_region_napot(0, 0, 1024 * 1024 * 1024); // 1GB
    uart_puts("[corode] pmp initialized\n");

    let mut state = CoreState::new();
    state.set_cond(id::PMP_OK);
    
    // EASTER EGG: Eierschalensollbruchstellenverursacher
    uart_puts("\nEierschalensollbruchstellenverursacher sagt Hallo!\n");

    uart_puts("Willkommen bei Corode. Das Orakel erwartet deine Eingabe.\n");
    uart_puts("> "); // Prompt für das Terminal
    
    let mut tick_counter = 0;

    loop {
        handle_terminal_input();

        // Verlangsamen des Heartbeats, damit das Terminal benutzbar bleibt
        tick_counter += 1;
        if tick_counter > 500000 {
            state.set_cond(id::TIMER_TICK);
            tick_counter = 0;
        }

        state.drain_pending_conds();
        block::run_blocks(block::BLOCKS, &mut state);
    }
}
