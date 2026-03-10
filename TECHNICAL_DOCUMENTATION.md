# Trickster Core – Technische Dokumentation

## Übersicht

Trickster Core ist ein experimenteller Bare-Metal-Kernel für die RISC-V-Architektur (RV64GC). Sein Hauptzweck ist die Demonstration einer hardware-isolierten Laufzeitumgebung, die durch die Physical Memory Protection (PMP)-Einheit von RISC-V geschützt wird.

Der Kernel fängt absichtlich eine Zugriffsverletzung ab, um seinen benutzerdefinierten Trap-Handler (`trickster_handler`) zu demonstrieren, der anstelle eines standardmäßigen Panic-Handlers die Kontrolle übernimmt.

## Komponenten

### `main.rs` – Der Kernel-Einstiegspunkt

- **`_start()`**: Die erste Funktion, die nach dem Bootloader ausgeführt wird. Sie ist `unsafe` und `extern "C"`, um die Rust-Namensveränderung zu verhindern.
- **Initialisierungssequenz**:
  1. **UART**: Initialisiert die serielle Schnittstelle für Textausgaben.
  2. **PMP**: Richtet die Physical Memory Protection ein, um den gesamten Speicher zu sperren.
  3. **Trap-Handler**: Setzt den `mtvec` (Machine Trap Vector) auf die Adresse von `trickster_handler`.
  4. **Zuse-Allokator**: Initialisiert den Speicherallokator für "Cages".
  5. **Sidekernel**: Startet den (derzeit leeren) Sidekernel-Layer.
  6. **Self-Attack**: Provoziert eine Zugriffsverletzung durch einen Schreibversuch auf eine geschützte Adresse, was den `trickster_handler` auslöst.

### `trickster.rs` – Der Trap-Handler

- **`trickster_handler()`**: Diese `#[naked]` Funktion wird direkt vom Prozessor bei einer Trap (wie einer Zugriffsverletzung) angesprungen.
- **Funktionsweise**: Gibt die im `.vault`-Speicherbereich hinterlegte Nachricht über die UART-Schnittstelle aus und geht dann in eine Endlosschleife.
- **`.vault`**: Ein spezieller Speicherbereich, der die Nachricht von Trickster enthält. Er ist durch die PMP-Einstellungen vor unbefugtem Zugriff geschützt.

### `pmp.rs` – Physical Memory Protection

- **`init()`**: Konfiguriert die 16 PMP-Regionen so, dass sie standardmäßig gesperrt sind.
- **`set_region()`**: Richtet eine einzelne PMP-Region mit einer Start-/Endadresse und Zugriffsrechten (Lesen, Schreiben, Ausführen) ein.
- **NAPOT**: Nutzt das "Naturally Aligned Power-of-Two" (NAPOT)-Format für eine effiziente Adressbereichsdefinition.

### `zuse.rs` – Der Cage-Allokator

- **`ZuseAllocator`**: Verwaltet einen Pool von 15 festen Speicherblöcken à 64 KB, die als "Cages" bezeichnet werden.
- **`Cage`**: Repräsentiert eine isolierte Speichereinheit für eine Sidekernel-Komponente.
- **Funktionsweise**: Verwendet eine Bitmaske (`free_mask`), um schnell freie Cages zu finden und zu belegen.

### `sidekernel.rs` – Der Supervisor-Layer

- **`Sidekernel`**: Ein Platzhalter für eine zukünftige Laufzeitumgebung, die isolierte Komponenten in den von `zuse.rs` bereitgestellten Cages ausführen würde.
- **Vision**: In einer vollständigen Implementierung würde der Sidekernel Systemaufrufe entgegennehmen und die Kommunikation zwischen den Cages verwalten.

### `condition.rs` – Bedingungsvariablen

- **`Condition`**: Ein Platzhalter für einen Synchronisationsmechanismus, der für die nebenläufige Ausführung von Tasks im Sidekernel unerlässlich wäre.

## Build- & Test-Prozess

- **Ziel-Architektur**: `riscv64gc-unknown-none-elf`
- **Toolchain**: Der Build erfordert die entsprechende Rust-Toolchain, die mit `rustup target add` installiert wird.
- **`run.sh`**: Ein Shell-Skript, das den `cargo build` Prozess und den anschließenden Test in QEMU automatisiert.
- **QEMU**: Der Kernel wird mit `qemu-system-riscv64` in einer `virt`-Maschine ohne grafische Oberfläche oder BIOS getestet.
