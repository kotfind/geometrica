{
    pkgs ? import <nixpkgs> {},
}:
pkgs.mkShell {
    name = "geometrica";

    buildInputs = with pkgs; [
        cargo
        rustc
        gcc
        pkg-config
        openssl
    ];

    nativeBuildInputs = with pkgs; [
        pkg-config
    ];
}
