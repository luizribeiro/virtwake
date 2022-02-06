{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    # dependencies
    cargo
    libvirt
    rustc

    # tools for development
    rust-analyzer
    rustfmt
    valgrind
  ];
}
