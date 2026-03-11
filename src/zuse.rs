#![allow(unused)]

pub struct ZuseAllocator;

impl ZuseAllocator {
    pub fn new() -> Self { Self } 
}

pub fn anzahl_belegt() -> usize {
    // TODO: implementieren
    1 // dummy
}

pub fn anzahl_gesamt() -> usize {
    15
}

pub fn liste_anzeigen(ausgabe: impl Fn(&str)) {
    ausgabe("  cage 0: belegt (system-terminal)\n");
    ausgabe("  cage 1: frei\n");
    ausgabe("  cage 2: frei\n");
    ausgabe("  cage 3: frei\n");
    ausgabe("  cage 4: frei\n");
    ausgabe("  cage 5: frei\n");
    ausgabe("  cage 6: frei\n");
    ausgabe("  cage 7: frei\n");
    ausgabe("  cage 8: frei\n");
    ausgabe("  cage 9: frei\n");
    ausgabe("  cage 10: frei\n");
    ausgabe("  cage 11: frei\n");
    ausgabe("  cage 12: frei\n");
    ausgabe("  cage 13: frei\n");
    ausgabe("  cage 14: frei\n");

}
