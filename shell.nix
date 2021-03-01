let
  pkgs = import ./nixpkgs.nix { };

  rust =
    pkgs.rustChannels.stable.rust.override {
      extensions = [
        "clippy-preview"
        "rustfmt-preview"
      ];
    };

in
pkgs.mkShell rec {
  RUST_BACKTRACE = 1;
  RUST_SRC_PATH  = "${pkgs.rustChannels.stable.rust-src}/lib/rustlib/src/rust/src";
  RUST_DOCS      = "${pkgs.rustChannels.stable.rust-docs}/share/doc/rust/html/index.html";
  
  buildInputs = [
    # environment
    pkgs.binutils
    pkgs.gcc
    pkgs.gnumake
    pkgs.openssl
    pkgs.git
    pkgs.pkg-config

    # rust
    rust
    pkgs.rustChannels.stable.rust-src
    pkgs.rustChannels.stable.rust-docs
    # pkgs.rustChannels.stable.clippy-preview
    # pkgs.rustChannels.stable.rustfmt
    
    # keep this line if you use bash
    pkgs.bashInteractive
  ];
  
  shellHook = ''
    export CARGO_HOME="${toString ./.}/.cargo";
  '';   
}
