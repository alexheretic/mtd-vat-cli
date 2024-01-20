# Use nix to get cargo2nix & rust toolchain on your path
# $ nix develop github:cargo2nix/cargo2nix#bootstrap

# In this directory with Cargo.toml & Cargo.lock files 
# $ cargo2nix

# You'll need that in version control
# $ git add Cargo.nix

# Build with nix
# $ nix build

# $ ./result-bin/bin/mtd-vat
# error: the following required arguments were not provided:
#  ...

# With thanks to https://github.com/cargo2nix/cargo2nix

{
  inputs = {
    cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.11.0";
    flake-utils.follows = "cargo2nix/flake-utils";
    nixpkgs.follows = "cargo2nix/nixpkgs";
  };

  outputs = inputs: with inputs;
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [cargo2nix.overlays.default];
        };

        rustPkgs = pkgs.rustBuilder.makePackageSet {
          rustVersion = "1.75.0";
          packageFun = import ./Cargo.nix;
        };

      in rec {
        packages = {
          # replace mtd-vat-cli with your package name
          mtd-vat-cli = (rustPkgs.workspace.mtd-vat-cli {});
          default = packages.mtd-vat-cli;
        };
      }
    );
}