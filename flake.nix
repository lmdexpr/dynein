{
  description = "Rust environment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix/monthly";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { fenix, flake-utils, nixpkgs, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        toolchain = fenix.packages.${system}.fromToolchainFile {
          file = ./rust-toolchain;
          sha256 = "sha256-7QfkHty6hSrgNM0fspycYkRcB82eEqYa4CoAJ9qA3tU=";
        };
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        formatter = pkgs.nixpkgs-fmt;
        devShells.default = pkgs.stdenv.mkDerivation {
          name = "rust environment";
          nativeBuildInputs = [
            toolchain
          ];
          buildInputs = with pkgs; [
            rust-analyzer
          ];
        };
      }
    );
}
