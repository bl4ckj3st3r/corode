//! Trickster – der freundliche Trap-Handler
use crate::TRICKSTER_MESSAGE;

#[no_mangle]
pub extern "C" fn trickster_handler() {
    unsafe {
        for &b in TRICKSTER_MESSAGE {
            core::ptr::write_volatile(0x10000000 as *mut u8, b);
        }
    }
    loop {}
}
