{ pkgs ? import <nixpkgs> {} }:

{
  packages = with pkgs; [
    rustup
    cargo
    rustc
    qemu
  ];
}
