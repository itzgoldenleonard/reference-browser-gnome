{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    # Currently you need to already have a rust toolchain installed on your computer
    # I might update it in the future to use the rust components from the unstable channel
    # the ones from 23.05 are too old for gtk4
    nativeBuildInputs = with pkgs.buildPackages; [ clang gtk4 pkg-config libadwaita ];
}
