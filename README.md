t# corode-core

**Isolation statt Abstraktion. Determinismus statt Interrupt. Harmonie statt Krieg.**

[![Lizenz: GPL v3](https://img.shields.io/badge/Lizenz-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

---

`corode-core` ist kein weiteres Betriebssystem. Es ist der Versuch, die seit 80 Jahren etablierte Von-Neumann-Architektur zu Ende zu denken und ihre fundamentalen Probleme zu lösen. Anstatt Prozesse um knappe Ressourcen kämpfen zu lassen, etabliert `corode-core` eine beweisbare Ordnung, die vor der Ausführung mathematisch sicherstellt, dass keine Konflikte entstehen.

**Race Conditions sind hier nicht nur selten. Sie sind unmöglich.**

## Das Kernprinzip: Das Condition Paradigma

Alles im System ist eine **Condition**: ein isolierter, vollständig aufgelöster, deterministisch ausführbarer Zustand. Eine Condition existiert nur, wenn ihre Existenz formal bewiesen werden kann. Sie läuft in einem eigenen, durch Hardware (RISC-V PMP) geschützten Speicherbereich und verschwindet, wenn ihre Aufgabe erfüllt ist.

- **Kein Kampf:** Ressourcen werden nicht zur Laufzeit verhandelt, sondern zur Compile-Zeit im **Manifest** festgeschrieben.
- **Kein Chaos:** Es gibt keinen Scheduler, der Entscheidungen trifft. Das System fließt nach den im Manifest bewiesenen Regeln.
- **Kein Raten:** Der Z3 SMT Solver beweist die Korrektheit jeder Condition, bevor sie entsteht.

## Aktueller Status: Genesis

Das Projekt befindet sich im `Genesis`-Stadium. Der erste, lauffähige Code existiert in `main.rs` auf bare-metal RISC-V. Dieser "Genesis-Commit" beweist die grundlegende Machbarkeit des Konzepts:

- **Kompilierung:** `no_std` und `no_main` sind umgesetzt.
- **Hardware-Isolation:** Physische Speicherprotektion (PMP) ist konzeptionell verankert.
- **Stabilität:** Ein erster Trap-Handler ("Harlekin") fängt bereits Zugriffsverletzungen ab und verhindert den Systemabsturz.

## Die Roadmap

Die vollständige Vision ist in der [ROADMAP.md](ROADMAP.md) detailliert. Die nächsten Meilensteine sind:

1.  **Phase 1: Beweisbare Stabilität:** Vollständige Implementierung der PMP-isolierten Speicherarchitektur und des fälschungssicheren "Trickster"-Loggers.
2.  **Phase 2: Infrastruktur:** Aufbau der `Z3³`-Speicherlogistik und des `SLS`-Speichersystems für maximale CPU-Sättigung.
3.  **Phase 3: Intelligenz:** Entwicklung des "Quarantäne Neural Network" zur automatischen Fehleranalyse und des "Weightless Agent Training" für autonome Systemoptimierung.

## Die Philosophie: Hardware-Sozialdemokratie

Jede Condition hat das Recht auf Existenz, solange ihre Anforderungen die Systemstabilität nicht gefährden. Sie erhält ihren festen Platz, darf nicht mehr beanspruchen, ist aber sicher, dass sie ihren Raum bekommt. Das ist keine technische, sondern eine philosophische Entscheidung gegen das "Survival of the Fittest"-Prinzip klassischer Betriebssysteme.

Dieses Projekt ist Open Source (GPLv3) und wird von der **Trickster IT SE** vorangetrieben.

---

# Technische Dokumentation

## Deutsch

Dieses Dokument beschreibt die Kernkomponenten des Bare-Metal-Betriebssystemprojekts "corode-core", das ausschließlich in Rust für die RISC-V-Architektur (riscv64gc-unknown-none-elf) entwickelt wurde. Der Fokus liegt auf einem minimalen, reinen Rust-Kernel ohne externe Abhängigkeiten wie C-Code oder Standardbibliotheken.

### 1. PMP (pmp.rs) - Physical Memory Protection in reinem Rust

Die physische Speicherprotektion (PMP) ist ein entscheidender Sicherheitsmechanismus in RISC-V-Systemen. In `corode-core` wird die PMP vollständig in Rust implementiert, um Speicherbereiche zu isolieren und die Systemstabilität zu gewährleisten.

**Kernkonzepte**

- **PMP-Regionen als Rust-Strukturen**: Jede PMP-Region wird durch eine Rust-Struktur repräsentiert, die ihre Basisadresse, Größe und Berechtigungen (Lesen, Schreiben, Ausführen) kapselt.

- **Hardware-Register-Mapping**: Die PMP-Hardware-Register (pmpcfg und pmpaddr) werden direkt in Rust abgebildet, was eine typsichere und intuitive Konfiguration ermöglicht.

- **`set_region()`-Funktion**: Eine dedizierte Rust-Funktion `set_pmp_region_napot()` konfiguriert eine PMP-Region für eine bestimmte `Condition`. Diese Funktion stellt sicher, dass jede `Condition` in einer isolierten Speicherregion ausgeführt wird, um unbefugte Speicherzugriffe zu verhindern.

### 2. VEKTOR-SPEICHER (memory/vector_alloc.rs) - Dynamische Speicherverwaltung in Rust

`corode-core` benötigt einen dynamischen Speicher-Allokator, um zur Laufzeit Speicher für verschiedene Datenstrukturen zuzuweisen. Der Vektor-Speicher ist ein reiner Rust-Allokator, der eng mit der PMP-Implementierung integriert ist.

**Design**

- **`allocate()` / `deallocate()`**: Der Allokator stellt einfache `allocate()`- und `deallocate()`-Funktionen zur Verfügung, um Speicherblöcke anzufordern und freizugeben.

- **Integration mit PMP**: Jede erfolgreiche Speicherallokation führt zur Erstellung einer dedizierten PMP-Region. Dies stellt sicher, dass der zugewiesene Speicher isoliert ist und nur von der Komponente zugegriffen werden kann, die ihn angefordert hat.

### 3. TRICKSTER-LOGGER (trickster.rs) - Ein sicherer Ringbuffer-Logger in Rust

Der `Trickster-Logger` ist ein reiner Rust-Logger, der für die Aufzeichnung von Systemereignissen, insbesondere von Trap-Informationen, konzipiert ist. Sein Design ist auf Sicherheit und Effizienz ausgelegt.

**Architektur**

- **`LogEntry`-Struktur**: Jede Log-Nachricht wird in einer `LogEntry`-Struktur gespeichert, die einen Zeitstempel, den Log-Level und die eigentliche Nachricht enthält.

- **Ringbuffer**: Der Logger verwendet einen Ringbuffer, um Log-Einträge zu speichern. Dies ist eine effiziente Methode, um eine feste Anzahl von Log-Nachrichten im Speicher zu halten, ohne dass es zu Speicherüberläufen kommt.

- **Schreibzugriff nur durch den Trap-Handler**: Nur der Trap-Handler (`Harlekin`) hat Schreibzugriff auf den Logger. Dies stellt sicher, dass Log-Einträge nur bei außergewöhnlichen Ereignissen wie PMP-Fehlern erstellt werden.

### 4. HARLEKIN (harlekin.rs) - Der Rust-Trap-Handler

`Harlekin` ist der zentrale Trap-Handler des Systems, der in reinem Rust implementiert ist. Seine Hauptaufgabe ist es, bei Traps (Ausnahmen) die Kontrolle zu übernehmen, die Ursache zu analysieren und entsprechende Maßnahmen zu ergreifen, um die Systemstabilität zu gewährleisten.

**Funktionsweise**

- **Auswertung der Trap-Ursache**: `Harlekin` liest das `mcause`-Register, um die Ursache des Traps zu ermitteln.

- **Behandlung von PMP-Fehlern**: Wenn ein PMP-Fehler (Speicherzugriffsverletzung) auftritt, ergreift `Harlekin` die folgenden Maßnahmen:
    1. **Loggen des Fehlers**: Der Fehler wird über den `Trickster-Logger` protokolliert, um eine spätere Analyse zu ermöglichen.
    2. **Freezen der `Condition`**: Die `Condition`, die den Fehler verursacht hat, wird "eingefroren", d.h. sie wird angehalten und an der weiteren Ausführung gehindert.

---

## English

This document describes the core components of the bare-metal operating system project "corode-core," developed exclusively in Rust for the RISC-V architecture (riscv64gc-unknown-none-elf). The focus is on a minimal, pure Rust kernel with no external dependencies such as C code or standard libraries.

### 1. PMP (pmp.rs) - Physical Memory Protection in Pure Rust

The Physical Memory Protection (PMP) is a crucial security mechanism in RISC-V systems. In `corode-core`, PMP is implemented entirely in Rust to isolate memory regions and ensure system stability.

**Core Concepts**

- **PMP Regions as Rust Structs**: Each PMP region is represented by a Rust struct that encapsulates its base address, size, and permissions (read, write, execute).

- **Hardware Register Mapping**: The PMP hardware registers (pmpcfg and pmpaddr) are directly mapped in Rust, enabling type-safe and intuitive configuration.

- **`set_region()` Function**: A dedicated Rust function, `set_pmp_region_napot()`, configures a PMP region for a specific `Condition`. This function ensures that each `Condition` runs in an isolated memory region to prevent unauthorized memory access.

### 2. VECTOR MEMORY (memory/vector_alloc.rs) - Dynamic Memory Management in Rust

`corode-core` requires a dynamic memory allocator to allocate memory for various data structures at runtime. The vector memory is a pure Rust allocator that is tightly integrated with the PMP implementation.

**Design**

- **`allocate()` / `deallocate()`**: The allocator provides simple `allocate()` and `deallocate()` functions to request and release memory blocks.

- **Integration with PMP**: Every successful memory allocation results in the creation of a dedicated PMP region. This ensures that the allocated memory is isolated and can only be accessed by the component that requested it.

### 3. TRICKSTER-LOGGER (trickster.rs) - A Secure Ring Buffer Logger in Rust

The `Trickster-Logger` is a pure Rust logger designed for recording system events, especially trap information. Its design focuses on security and efficiency.

**Architecture**

- **`LogEntry` Struct**: Each log message is stored in a `LogEntry` struct, which contains a timestamp, log level, and the actual message.

- **Ring Buffer**: The logger uses a ring buffer to store log entries. This is an efficient method for keeping a fixed number of log messages in memory without causing memory overflows.

- **Write Access Only by the Trap Handler**: Only the trap handler (`Harlekin`) has write access to the logger. This ensures that log entries are created only in exceptional circumstances, such as PMP errors.

### 4. HARLEKIN (harlekin.rs) - The Rust Trap Handler

`Harlekin` is the system's central trap handler, implemented in pure Rust. Its main task is to take control during traps (exceptions), analyze the cause, and take appropriate measures to ensure system stability.

**Functionality**

- **Evaluation of the Trap Cause**: `Harlekin` reads the `mcause` register to determine the cause of the trap.

- **Handling of PMP Errors**: If a PMP error (memory access violation) occurs, `Harlekin` takes the following actions:
    1. **Logging the Error**: The error is logged via the `Trickster-Logger` to allow for later analysis.
    2. **Freezing the `Condition`**: The `Condition` that caused the error is "frozen," meaning it is halted and prevented from further execution.
