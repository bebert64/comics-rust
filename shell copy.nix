{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {
  buildInputs = with pkgs; [ cargo rustc openssl libpqxx postgresql ];
  nativeBuildInputs = with pkgs; [ pkg-config ];
}
