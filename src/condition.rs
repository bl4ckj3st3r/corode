#![allow(unused)]

pub fn anzahl_aktiv() -> usize {
    // TODO: implementieren
    3 // dummy
}

pub fn liste_anzeigen(ausgabe: impl Fn(&str)) {
    ausgabe("  condition 1: aktiv (cage 0)\n");
    ausgabe("  condition 2: schlafend\n");
    ausgabe("  condition 3: training\n");
}

pub fn einfrieren(id: u64) {
    // TODO: implementieren
    crate::uart::puts("[log] condition eingefroren\n");
}

pub fn auftauen(id: u64) {
    // TODO: implementieren
    crate::uart::puts("[log] condition aufgetaut\n");
}

pub fn starten(name: &str) {
    // TODO: implementieren
    crate::uart::puts("[log] condition gestartet\n");
}

pub fn stoppen(id: u64) {
    // TODO: implementieren
    crate::uart::puts("[log] condition gestoppt\n");
}
