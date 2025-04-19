{
  inputs = {
    nixpkgs.url = "nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;

        # for corefonts
        config.allowUnfree = true;
      };
      lib = pkgs.lib;

      inherit (pkgs) mkShell;
      inherit (lib.strings) concatMapStringsSep;

      fonts = with pkgs; [
        corefonts
        dejavu_fonts
      ];

      shell = mkShell {
        name = "geometrica-doc-shell";

        buildInputs = with pkgs;
          [
            typst
            zathura
          ]
          ++ fonts;

        TYPST_FONT_PATHS =
          concatMapStringsSep
          ":"
          (font_pkg: "${font_pkg}/share/fonts/truetype/")
          fonts;
      };
    in {
      devShell = shell;
    });
}
