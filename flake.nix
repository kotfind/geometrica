{
  description = "Geometrica";

  inputs = {
    nixpkgs.url = "nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    nixpkgs,
    flake-utils,
    fenix,
    crane,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {inherit system;};
        lib = pkgs.lib;

        fenixProfile = fenix.packages.${system}.stable;
        craneLib = (crane.mkLib pkgs).overrideToolchain (_:
          fenixProfile.withComponents [
            "rustc"
            "cargo"
            "rustfmt"
            "clippy"
          ]);

        inherit (builtins) isPath removeAttrs map mapAttrs attrValues;
        inherit (craneLib) buildDepsOnly buildPackage crateNameFromCargoToml devShell;
        inherit (craneLib.fileset) commonCargoSources;
        inherit (flake-utils.lib) mkApp;
        inherit (lib.fileset) toSource unions;
        inherit (lib.lists) all foldl;
        inherit (lib.path) append;
        inherit (lib.strings) makeLibraryPath;

        buildCrate = {
          cratePath,
          localDeps ? [],
          ...
        } @ origArgs:
          assert isPath cratePath;
          assert all isPath localDeps; let
            extraArgs = removeAttrs origArgs ["cratePath" "localDeps"];

            src = toSource {
              root = ./.;
              fileset = unions ([
                  ./Cargo.lock
                  ./Cargo.toml
                ]
                ++ (
                  map
                  commonCargoSources
                  (localDeps ++ [cratePath])
                ));
            };

            cargoToml = append cratePath "Cargo.toml";
            crateName = (crateNameFromCargoToml {inherit cargoToml;}).pname;

            commonArgs =
              extraArgs
              // {
                inherit src cargoToml;
                strictDeps = true;
                cargoExtraArgs = "-p ${crateName}";
              };

            deps = buildDepsOnly commonArgs;

            pkg = buildPackage (commonArgs // {cargoArtifacts = deps;});
          in
            pkg;

        crates = {
          server = {
            cratePath = ./crates/server;

            localDeps = [
              ./crates/executor
              ./crates/types
              ./crates/parser
            ];

            buildInputs = with pkgs; [openssl];

            nativeBuildInputs = with pkgs; [pkg-config];
          };

          cli = {
            cratePath = ./crates/cli;

            localDeps = [
              ./crates/client
              ./crates/parser
              ./crates/types
            ];

            buildInputs = with pkgs; [openssl];

            nativeBuildInputs = with pkgs; [pkg-config];
          };

          gui = {
            cratePath = ./crates/gui;

            localDeps = [
              ./crates/client
              ./crates/parser
              ./crates/types
            ];

            buildInputs = with pkgs; [
              openssl

              # Iced Dependencies
              # source: https://github.com/iced-rs/iced/blob/master/DEPENDENCIES.md
              expat
              fontconfig
              freetype
              freetype.dev
              libGL
              xorg.libX11
              xorg.libXcursor
              xorg.libXi
              xorg.libXrandr
              wayland
              libxkbcommon

              # Iced Dependencies
              # that are not listed above
              xorg.libxcb
              vulkan-loader
              libgcc
            ];

            nativeBuildInputs = with pkgs; [
              pkg-config
              autoPatchelfHook
            ];

            runtimeDependencies = with pkgs; [
              wayland
              libglvnd
              libxkbcommon
            ];

            # postFixup = ''
            #   patchelf --add-rpath ${
            #     makeLibraryPath (with pkgs; [
            #       vulkan-loader
            #       libxkbcommon
            #     ])
            #   } $out/bin/gui
            # '';

            # postFixup = ''
            #   patchelf $out/bin/sniffnet --add-rpath ${
            #     makeLibraryPath
            #     (with pkgs; [
            #       vulkan-loader
            #       xorg.libX11
            #       libxkbcommon
            #     ])
            #   }
            # '';
          };
        };

        packages = mapAttrs (_: crate: buildCrate crate) crates;
        apps = mapAttrs (_: pkg: mkApp {drv = pkg;}) packages;

        shell = devShell rec {
          buildInputs =
            foldl
            (acc: crate: acc ++ (crate.buildInputs or []))
            []
            (attrValues crates);

          nativeBuildInputs =
            foldl
            (acc: crate: acc ++ (crate.nativeBuildInputs or []))
            []
            (attrValues crates);

          LD_LIBRARY_PATH = makeLibraryPath buildInputs;
        };
      in {
        inherit packages apps;
        devShells.default = shell;
      }
    );
}
