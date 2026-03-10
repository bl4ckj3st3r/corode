{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustup
    cargo
    rustc
    rustfmt
    clippy
    gcc
    binutils
    qemu
  ];

  shellHook = ''
    echo "🔥 CORODE ENTWICKLUNGSUMGEBUNG"
    echo "──────────────────────────────"
    
    # RISC-V Target
    rustup target add riscv64gc-unknown-none-elf 2>/dev/null || true
    
    # Cargo-Konfiguration prüfen
    mkdir -p .cargo
    
    echo "🚀 Jetzt bauen: cargo build --bin corode-core"
    echo "🖥️  QEMU-Test: qemu-system-riscv64 -machine virt -nographic -kernel target/riscv64gc-unknown-none-elf/debug/corode-core"
  '';
}
