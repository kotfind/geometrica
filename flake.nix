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

        inherit (builtins) isPath removeAttrs map concatMap attrValues concatLists isBool;
        inherit (craneLib) buildDepsOnly buildPackage crateNameFromCargoToml devShell cargoDoc cargoTest;
        inherit (craneLib.fileset) commonCargoSources;
        inherit (flake-utils.lib) mkApp;
        inherit (lib) getExe' getLib optionalAttrs;
        inherit (lib.attrsets) unionOfDisjoint recursiveUpdate mapAttrsToList isDerivation;
        inherit (lib.fileset) toSource unions;
        inherit (lib.lists) all foldl;
        inherit (lib.path) append;
        inherit (lib.strings) makeLibraryPath concatStringsSep;
        inherit (pkgs) writeShellScriptBin;

        buildCrate = {
          cratePath,
          isBinary,
          localDeps ? [],
          testRuntimeDeps ? [],
          ...
        } @ origArgs:
          assert isPath cratePath;
          assert isBool isBinary;
          assert all isDerivation testRuntimeDeps;
          assert all isPath localDeps; let
            extraArgs = removeAttrs origArgs [
              "cratePath"
              "localDeps"
              "isBinary"
              "testRuntimeDeps"
            ];

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
                cargoExtraArgs = "-p ${crateName} --all-features";
              };

            deps = buildDepsOnly commonArgs;

            checks = cargoTest (commonArgs
              // {
                cargoArtifacts = deps;

                nativeBuildInputs =
                  (commonArgs.nativeBuildInputs or [])
                  ++ testRuntimeDeps;
              });

            pkg = buildPackage (commonArgs // {cargoArtifacts = deps;});
            app = mkApp {drv = pkg;};

            docPkg = cargoDoc (commonArgs // {cargoArtifacts = deps;});

            docApp = mkApp {
              drv = let
                mimeopen = getExe' pkgs.perl540Packages.FileMimeInfo "mimeopen";
                docIndex = "${docPkg}/share/doc/${crateName}/index.html";
              in
                writeShellScriptBin "${crateName}-doc-open" ''
                  ${mimeopen} ${docIndex}
                '';
            };
          in
            recursiveUpdate
            {
              checks."${crateName}" = checks;

              packages."${crateName}-doc" = docPkg;
              apps."${crateName}-doc" = docApp;
            }
            (optionalAttrs isBinary {
              packages.${crateName} = pkg;
              apps.${crateName} = app;
            });

        crates = rec {
          server = {
            cratePath = ./crates/server;
            isBinary = true;
            localDeps = [
              ./crates/executor
              ./crates/types
              ./crates/parser
            ];

            buildInputs = with pkgs; [openssl];
            nativeBuildInputs = with pkgs; [pkg-config];
          };

          client = {
            cratePath = ./crates/client;
            isBinary = false;
            localDeps = [
              ./crates/parser
              ./crates/types
            ];

            buildInputs = with pkgs; [openssl];
            nativeBuildInputs = with pkgs; [pkg-config];
            testRuntimeDeps = [(buildCrate server).packages.server];
          };

          cli = {
            cratePath = ./crates/cli;
            isBinary = true;
            localDeps = [
              ./crates/client
              ./crates/parser
              ./crates/types
            ];

            buildInputs = with pkgs; [openssl];
            nativeBuildInputs = with pkgs; [pkg-config];
            testRuntimeDeps = [(buildCrate server).packages.server];
          };

          gui = {
            cratePath = ./crates/gui;
            isBinary = true;
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

              # RFD Dependencies
              # Source: https://github.com/PolyMeilex/rfd/issues/124#issuecomment-1901738167
              gtk3
              glib
              gdk-pixbuf
              cairo
              pango
              atk
              gsettings-desktop-schemas
              makeWrapper

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

            # RFD Dependencies
            # Source: https://github.com/PolyMeilex/rfd/issues/124#issuecomment-1901738167
            preFixup = ''
              # Run for final package only, don't run for gui-deps
              if [ -f "$out/bin/server" ]; then
                wrapProgram "$out/bin/server" \
                  --prefix XDG_DATA_DIRS : "${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}" \
                  --prefix XDG_DATA_DIRS : "${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}"
              fi
            '';

            testRuntimeDeps = [(buildCrate server).packages.server];
          };

          executor = {
            cratePath = ./crates/executor;
            isBinary = false;
            localDeps = [
              ./crates/parser
              ./crates/types
            ];

            buildInputs = with pkgs; [openssl];
            nativeBuildInputs = with pkgs; [pkg-config];
          };

          parser = {
            cratePath = ./crates/parser;
            isBinary = false;
            localDeps = [./crates/types];
          };

          types = {
            cratePath = ./crates/types;
            isBinary = false;
            buildInputs = with pkgs; [openssl];
            nativeBuildInputs = with pkgs; [pkg-config];
          };
        };

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

          concatedInputs = concatLists (attrValues inputs);
        in
          devShell {
            packages =
              concatedInputs
              ++ (with pkgs; [
                rust-analyzer
                cargo-tarpaulin
              ]);

            LD_LIBRARY_PATH = makeLibraryPath concatedInputs;

            shellHook = ''
              # RFD Dependencies
              # Source: https://github.com/PolyMeilex/rfd/issues/124#issuecomment-1901738167
              export XDG_DATA_DIRS="${
                lib.debug.traceVal (concatStringsSep ":" [
                  "${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}"
                  "${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}"
                  "$XDG_DATA_DIRS"
                ])
              }"
            '';
          };
      in
        {
          devShells.default = shell;
        }
        // (
          foldl
          recursiveUpdate
          {}
          (
            mapAttrsToList
            (_: c: buildCrate c)
            crates
          )
        )
    );
}
