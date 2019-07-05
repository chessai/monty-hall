let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
in
  with nixpkgs;
  with nixpkgs.latest.rustChannels.nightly;
  mkShell {
    buildInputs = [
      cargo
      #clippy-preview
      rust
      rustfmt-preview
    ];
  }
