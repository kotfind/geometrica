{ pkgs, ... }:
let
    # Source: https://github.com/iced-rs/iced/blob/master/DEPENDENCIES.md
    icedDeps = with pkgs; [
        expat
        fontconfig
        freetype
        freetype.dev
        libGL
        pkg-config
        xorg.libX11
        xorg.libXcursor
        xorg.libXi
        xorg.libXrandr
        wayland
        libxkbcommon
        openssl
    ];
in
pkgs.mkShell rec {
    name = "geometrica";

    nativeBuildInputs = with pkgs; [
        cargo
        rustc
        rustfmt
        clippy

        gcc
        pkg-config
    ] ++ icedDeps;

    LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath nativeBuildInputs;
}
