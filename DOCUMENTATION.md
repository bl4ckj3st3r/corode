# Corode Dokumentation

## Übersicht

Corode ist ein experimenteller Bare-Metal-Kernel für RISC-V, geschrieben in Rust. Das Projekt legt den Fokus auf eine minimale Codebasis und eine nachvollziehbare Architektur, die ohne externe Abhängigkeiten auskommt.

## Build & Test

Das Projekt verwendet `nix-shell`, um eine reproduzierbare und stabile Build-Umgebung zu gewährleisten.

1.  **Umgebung betreten:**
    ```bash
    nix-shell
    ```

2.  **Kompilieren:**
    ```bash
    cargo build --bin corode-core --target riscv64gc-unknown-none-elf
    ```

3.  **In QEMU testen:**
    ```bash
    qemu-system-riscv64 -machine virt -nographic -kernel target/riscv64gc-unknown-none-elf/debug/corode-core
    ```

## Troubleshooting

Bei der Entwicklung traten einige spezifische Build-Fehler auf, die hier dokumentiert sind.

### 1. Linker-Fehler (`GLIBC_... not found`)

*   **Problem:** Beim Kompilieren traten Fehler auf, die auf inkompatible Versionen der `GLIBC`-Systembibliothek hindeuteten. Zum Beispiel: `error adding symbols: DSO missing from command line` oder `/nix/store/.../bin/ld: ...: version 'GLIBC_2.34' not found (required by ...)`.

*   **Ursache:** Die Entwicklungsumgebung mischte Werkzeuge (wie `clang`, `ld`) aus dem System mit denen, die von Nix temporär bereitgestellt wurden. Dies führte zu Versionskonflikten bei essenziellen Bibliotheken.

*   **Lösung:** Die Einführung einer `shell.nix`-Datei. Diese Datei definiert eine **vollständig isolierte** Build-Umgebung, in der alle Werkzeuge (`rustc`, `cargo`, `qemu`, `binutils`, `gcc`) aus einer konsistenten Quelle stammen und garantiert zueinander passen. Anstatt Werkzeuge in die Umgebung zu laden, betreten wir eine saubere, dafür vorgesehene Umgebung.

### 2. Linker-Fehler (`duplicate symbol: _start`)

*   **Problem:** Der Build schlug mit der Meldung fehl, dass das `_start`-Symbol mehrfach definiert sei.

*   **Ursache:** Sowohl die Rust-Einstiegsfunktion in `src/main.rs` (`#[no_mangle] pub extern "C" fn _start()`) als auch die Assembler-Datei `src/trap.S` definierten ein `_start`-Label. Der Linker wusste nicht, welcher Einstiegspunkt der richtige ist.

*   **Lösung:** Entfernung des gesamten `_start`-Blocks aus der Assembler-Datei `src/trap.S`. Der einzige und korrekte Einstiegspunkt wird nun in `src/main.rs` definiert, wo auch der Stack-Pointer initialisiert wird.
