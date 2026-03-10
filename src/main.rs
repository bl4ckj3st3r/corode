#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(asm)]

extern crate alloc;

mod block;
mod cond;
mod pmp;
mod state;
mod uart;
mod trickster;
mod harlekin;
mod memory;

use core::panic::PanicInfo;
use crate::uart::{uart_puts, uart_getc, set_pink_mode};
use crate::state::CoreState;
use crate::cond::id;

const CMD_BUFFER_SIZE: usize = 128;
static mut CMD_BUFFER: [u8; CMD_BUFFER_SIZE] = [0; CMD_BUFFER_SIZE];
static mut CMD_LEN: usize = 0;

static mut PANIC_COUNT: u32 = 0;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let harlekin_gesichter = ["O_o", "o_O", "X_x", "x_X", "T_T"];

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
                13 => { // Enter
                    uart_puts("\n");
                    if CMD_LEN > 0 {
                        let cmd = core::str::from_utf8_unchecked(&CMD_BUFFER[0..CMD_LEN]);
                        if cmd == "test_trap" {
                            uart_puts("Testing trap handler...\n");
                            harlekin::handle_trap(5, 0x12345678);
                            trickster::print_logs();
                        } else {
                            uart_puts("Echo: ");
                            uart_puts(cmd);
                            uart_puts("\n");
                        }
                        CMD_LEN = 0;
                    }
                    uart_puts("> ");
                },
                127 | 8 => { // Backspace
                    if CMD_LEN > 0 {
                        CMD_LEN -= 1;
                        uart_puts("\x08 \x08");
                    }
                },
                _ => {
                    if CMD_LEN < CMD_BUFFER_SIZE - 1 {
                        CMD_BUFFER[CMD_LEN] = c;
                        CMD_LEN += 1;
                        uart::uart_puts(core::str::from_utf8_unchecked(&[c]));
                    }
                }
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    // Bringe die PMP in einen deterministischen, sicheren Zustand.
    pmp::init();
    uart_puts("\n[corode] PMP initialized. All regions locked.\n");

    // Definiere die Lebensgrundlage des Kernels: 1GB RAM mit vollem Zugriff.
    pmp::set_pmp_napot(
        0, 
        0x80000000, 
        1024 * 1024 * 1024, 
        pmp::READ | pmp::WRITE | pmp::EXEC
    );
    uart_puts("[corode] PMP Region 0 (Kernel-Space) configured.\n");

    // Initialisiere die deterministische Vektor-Speicher-Architektur "Zuse".
    memory::init();
    uart_puts("[corode] Zuse vector memory initialized.\n");

    let mut state = CoreState::new();
    state.set_cond(id::PMP_OK);
    
    uart_puts("Willkommen bei Corode. Das Orakel erwartet deine Eingabe.\n");
    uart_puts("> ");
    
    let mut tick_counter = 0;

    loop {
        handle_terminal_input();

        tick_counter += 1;
        if tick_counter > 500000 {
            state.set_cond(id::TIMER_TICK);
            tick_counter = 0;
        }

        state.drain_pending_conds();
        block::run_blocks(block::BLOCKS, &mut state);
    }
}
