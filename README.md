nixd
-----

This utility initializes a new, nix-shell based development environment (nix + niv + lorri + direnv) in the current working directory.

It effectively automates the steps defined in [this excellent blog post](https://christine.website/blog/how-i-start-nix-2020-03-08).

While my very own personal nix workflow is still very much a work in progress, this workflow is pretty close to what I do everytime I start a new, local project.

# Prerequisites

The following programs are assumed to be installed and accessible on the user path:

- [niv](https://github.com/nmattia/niv/#install)
- [lorri](https://github.com/target/lorri#setup-on-nixos-or-with-home-manager-on-linux)
- [direnv](https://direnv.net/#basic-installation)

I am running NixOS locally and have the lorri dameon running as a service in the backgrounnd.  

## Installation

In order to install nixd, checkout this prohect and run nix-env -i nixd -f ./default.nix

## Building

## Running

In the directory you want to `nix`, simply run `nixd`.
