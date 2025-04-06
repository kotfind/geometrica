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

      inherit (pkgs) mkShell;

      shell = mkShell {
        name = "geometrica-doc-shell";

        buildInputs = with pkgs; [
          typst
          zathura
          corefonts
        ];

        TYPST_FONT_PATHS = "${pkgs.corefonts}/share/fonts/truetype/";
      };
    in {
      devShell = shell;
    });
}
