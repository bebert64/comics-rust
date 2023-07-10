{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {
  buildInputs = with pkgs; [ cargo rustc sqlite openssl libpqxx postgresql ];
  nativeBuildInputs = with pkgs; [ pkg-config ];
}
