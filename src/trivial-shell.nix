let
  pkgs = import ./nix/nixpkgs.nix { };
in
pkgs.mkShell rec {
  buildInputs = [
    # keep this line if you use bash
    pkgs.bashInteractive
  ];
  
  shellHook = ''
  '';   
}
