{ pkgs, ... }:
pkgs.mkShell {
    name = "geometrica";

    buildInputs = with pkgs; [
        cargo
        rustc
        gcc
        pkg-config
        openssl
        clippy
        rustfmt
    ];

    nativeBuildInputs = with pkgs; [
        pkg-config
    ];

    LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [ pkgs.openssl ];
}
