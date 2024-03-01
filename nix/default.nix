{ pkgs ? import <nixpkgs> { }, lib }:

pkgs.rustPlatform.buildRustPackage rec {
  pname = "skout_bot";
  version = "0.1.0";
  cargoLock.lockFile = ../Cargo.lock;
  src = pkgs.lib.cleanSource ../.;
  buildInputs = with pkgs; [ pkg-config openssl ];
  nativeBuildInputs = with pkgs; [ pkg-config openssl ];
#  doCheck = false;
}
