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

        inherit (builtins) isPath removeAttrs map mapAttrs concatMap attrValues concatLists;
        inherit (craneLib) buildDepsOnly buildPackage crateNameFromCargoToml devShell;
        inherit (craneLib.fileset) commonCargoSources;
        inherit (flake-utils.lib) mkApp;
        inherit (lib.attrsets) unionOfDisjoint;
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

              # Other Dependencies
              openssl
              libgcc
            ];

            nativeBuildInputs = with pkgs; [
              pkg-config
              autoPatchelfHook
            ];

            # For `autoPatchelfHook`
            runtimeDependencies = with pkgs; [
              libxkbcommon
              vulkan-loader
            ];
          };
        };

        packages = mapAttrs (_: crate: buildCrate crate) crates;
        apps = mapAttrs (_: pkg: mkApp {drv = pkg;}) packages;

        collectCrateOptions = optionNames:
          foldl
          (acc: optionName:
            unionOfDisjoint acc {
              "${optionName}" =
                concatMap
                (c: c.${optionName} or [])
                (attrValues crates);
            })
          {}
          optionNames;

        shell = let
          inputs = collectCrateOptions [
            "buildInputs"
            "nativeBuildInputs"
            "runtimeDependencies"
          ];
        in
          devShell (inputs
            // {
              LD_LIBRARY_PATH = makeLibraryPath (concatLists (attrValues inputs));
            });
      in {
        inherit packages apps;
        devShells.default = shell;
      }
    );
}
