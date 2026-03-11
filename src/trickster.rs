//! Trickster – der freundliche Trap-Handler und Logger

use crate::uart;
use crate::TRICKSTER_MESSAGE;

#[no_mangle]
pub extern "C" fn trickster_handler() {
    uart::puts("\n\n*** TRAP GEFANGEN ***\n");
    uart::puts(core::str::from_utf8(TRICKSTER_MESSAGE).unwrap_or("Fehlerhafte UTF-8-Nachricht im Vault\n"));
    loop {}
}

pub fn logs_anzeigen(ausgabe: impl Fn(&str)) {
    ausgabe("  [00:01] condition 1: normal beendet\n");
    ausgabe("  [00:02] condition 2: training abgeschlossen\n");
    ausgabe("  [00:03] condition 3: eingefroren (pmp-verletzung)\n");
}

pub fn log_training(zyklus: u64) {
    // Dies wird jetzt direkt im Terminal gehandhabt, aber die Funktion bleibt als API-Platzhalter.
    // In einer echten Implementierung würde dies mit dem Sidekernel interagieren.
}

pub fn log(msg: &str) {
    // Allzweck-Logger, der zum UART schreibt (könnte später an einen Ringpuffer gehen)
    uart::puts("[log] ");
    uart::puts(msg);
    uart::puts("\n");
}
