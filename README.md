# Corode: Das deterministische RISC-V Betriebssystem

Corode ist ein experimentelles, auf Determinismus ausgelegtes Betriebssystem für RISC-V Architekturen, geschrieben in Rust. Es verfolgt das Ziel, eine vollständig vorhersagbare und sichere Umgebung für kritische Anwendungen zu schaffen, indem es radikal mit traditionellen, undeterministischen Kernel-Konzepten bricht.

## Kernprinzipien

*   **Determinismus:** Jeder Systemzustand ist eine direkte, berechenbare Folge des vorherigen Zustands und der eingetretenen `Conditions`. Es gibt keinen Platz für Zufall oder unvorhersehbare Seiteneffekte.
*   **Physische Speicherprotektion:** Sicherheit wird nicht durch komplexe Software-Abstraktionen, sondern durch die direkte Konfiguration der RISC-V Physical Memory Protection (PMP) Unit erzwungen. Der Speicher ist physisch in geschützte Zonen aufgeteilt.
*   **Vektor-Speicher "Zuse":** Anstelle eines traditionellen Heaps verwendet Corode einen deterministischen Block-Allokator namens "Zuse". Speicher wird nicht in variablen Größen angefordert, sondern in Form von festen, PMP-gesicherten "Käfigen" (`Cages`) zugeteilt. Dies eliminiert Speicherfragmentierung und sorgt für vorhersagbare Allokationszeiten.

## Die Revolution der Speicherverwaltung

Die jüngste Transformation hat Corode von einem halb-simulierten System zu einem echten, deterministischen Kernel gemacht:

1.  **Eliminierung des Heaps:** Der `global_allocator` wurde durch den `ZuseAllocator` ersetzt. Anfragen an den Allokator liefern nun einen von 15 verfügbaren, 64KB großen und PMP-geschützten Speicher-`Cages`.
2.  **Hardware-nahe PMP-Kontrolle:** Das `pmp.rs`-Modul manipuliert nicht länger nur simulierte Datenstrukturen. Es schreibt nun direkt in die `pmpcfg` und `pmpaddr` Control and Status Registers (CSRs) der CPU und konfiguriert so die physische Speicher-Firewall.
3.  **Sicherer Grundzustand:** Beim Booten versetzt `pmp::init()` das System in einen Zustand totaler Sperrung. Erst danach werden dem Kernel explizit seine notwendigen Privilegien für den Hauptspeicher erteilt. Alle anderen Speicherbereiche bleiben unzugänglich, bis sie durch den Zuse-Allokator explizit einem `Cage` zugewiesen werden.

Corode ist damit ein Beweis, dass Betriebssysteme nicht zwangsläufig komplexe, undeterministische Kolosse sein müssen. Es ist ein Schritt hin zu einer Welt, in der Software die Präzision und Vorhersagbarkeit der Hardware widerspiegelt.
