{
  cfg ? { },
}:
let
  sources  =
    import ./sources.nix;
  pkgs     =
    import sources.nixpkgs cfg;
in
  pkgs
