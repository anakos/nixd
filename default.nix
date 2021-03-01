{ pkgs ? import ./nixpkgs.nix { } }:

with pkgs; rustPlatform.buildRustPackage rec {
  pname = "nixd";
  version = "0.1.1";

  doCheck = false;

  src = ./.;

  cargoSha256 = "1arnjbhkj4m3vfq4qxri4a9qfjkfgpwdrhz0n695fz8yssdw2gms";

  meta = with lib; {
    description = "Initializes a nix-shell based development environment (niv + lorri + direnv) in the current working directory";
    homepage = "https://github.com/anakos/nixd";
    license = licenses.unlicense;
    # maintainers = [ maintainers.tailhook ];
  };
}
