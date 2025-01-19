{
  description = "Build a cargo project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane.url = "github:ipetkov/crane";

    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs@{ self, nixpkgs, crane, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        inherit (pkgs) lib;

        rustToolchainFor = p: p.rust-bin.stable.latest.minimal.override {
          extensions = [ "rust-docs" "clippy" "rustfmt" ];
          targets = [ "wasm32-unknown-unknown" ];
        };
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchainFor;

        rustNightlyToolchainFor = p: p.rust-bin.nightly."2025-01-02".minimal.override {
          extensions = [ "rust-docs" "clippy" "rust-src" "rustc-dev" "llvm-tools-preview" ];
          targets = [ "wasm32-unknown-unknown" ];
        };
        craneNightlyLib = (crane.mkLib pkgs).overrideToolchain rustNightlyToolchainFor;

        # When filtering sources, we want to allow assets other than .rs files
        unfilteredRoot = ./.; # The original, unfiltered source
        fileset = lib.fileset.unions [
            # Default files from crane (Rust and cargo files)
            (craneLib.fileset.commonCargoSources unfilteredRoot)
            (lib.fileset.fileFilter
              (file: lib.any file.hasExt [ "html" "scss" ])
              unfilteredRoot
            )
            ./tucant-extension/bootstrap.bundle.min.js
            ./tucant-extension/bootstrap.min.css
            ./tucant-yew/fixup.sh
        ];
        src = lib.fileset.toSource {
          root = unfilteredRoot;
          fileset = fileset;
        };

        # Arguments to be used by both the client and the server
        # When building a workspace with crane, it's a good idea
        # to set "pname" and "version".
        commonArgs = {
          inherit src;
          strictDeps = true;

          buildInputs = [
            # Add additional build inputs here
          ] ++ lib.optionals pkgs.stdenv.isDarwin [
            # Additional darwin specific inputs can be set here
            pkgs.libiconv
          ];
        };

        # Native packages

        nativeArgs = commonArgs // {
          pname = "trunk-workspace-native";
        };

        # Build *just* the cargo dependencies, so we can reuse
        # all of that work (e.g. via cachix) when running in CI
        cargoArtifacts = craneLib.buildDepsOnly nativeArgs;

        # Simple JSON API that can be queried by the client
        myServer = craneLib.buildPackage (nativeArgs // {
          inherit cargoArtifacts;
          # The server needs to know where the client's dist dir is to
          # serve it, so we pass it as an environment variable at build time
          CLIENT_DIST = myClient;
        });

        # Wasm packages

        # it's not possible to build the server on the
        # wasm32 target, so we only build the client.
        wasmArgs = commonArgs // {
          pname = "trunk-workspace-wasm";
          cargoExtraArgs = "--package=tucant-yew";
          CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
        };

        cargoArtifactsWasm = craneLib.buildDepsOnly (wasmArgs // {
          doCheck = false;
        });

        trunk = pkgs.trunk.overrideAttrs (oldAttrs: rec {
          version = "0.21.6";
          src = pkgs.fetchFromGitHub {
            owner = "mohe2015";
            repo = "trunk";
            rev = "fix-critical-navigation-bug-in-firefox-with-version-bump";
            hash = "sha256-HW0eoIQG7Ida4+/JY5goLpQI7750zAfg17kV/RJ3UJA=";
          };
          cargoDeps = oldAttrs.cargoDeps.overrideAttrs (pkgs.lib.const {
            name = "${oldAttrs.pname}-vendor.tar.gz";
            inherit src;
            outputHash = "sha256-DDP3/DJaZmckcgb9qtmH0FVviITBpB8WwmlVAkvepsY=";
          });
        });

        # Build the frontend of the application.
        # This derivation is a directory you can put on a webserver.
        myClient = craneLib.buildTrunkPackage.override { trunk = trunk; } (wasmArgs // {
          trunkExtraBuildArgs = "--features direct --public-url /dist";
          pname = "trunk-workspace-tucant-yew";
          cargoArtifacts = cargoArtifactsWasm;
          # Trunk expects the current directory to be the crate to compile
          preBuild = ''
            cd ./tucant-yew
          '';
          # After building, move the `dist` artifacts and restore the working directory
          postBuild = ''
            mv ./dist ..
            cd ..
          '';
          # The version of wasm-bindgen-cli here must match the one from Cargo.lock.
          wasm-bindgen-cli = pkgs.wasm-bindgen-cli.override {
            version = "0.2.99";
            hash = "sha256-1AN2E9t/lZhbXdVznhTcniy+7ZzlaEp/gwLEAucs6EA=";
            cargoHash = "sha256-DbwAh8RJtW38LJp+J9Ht8fAROK9OabaJ85D9C/Vkve4=";
            # When updating to a new version comment out the above two lines and
            # uncomment the bottom two lines. Then try to do a build, which will fail
            # but will print out the correct value for `hash`. Replace the value and then
            # repeat the process but this time the printed value will be for `cargoHash`
            # hash = lib.fakeHash;
            # cargoHash = lib.fakeHash;
          };
        });

        fileset-extension = lib.fileset.unions [
          ./tucant-extension/background.js
          ./tucant-extension/fix-session-id-in-url.js
          ./tucant-extension/content-script.js
          ./tucant-extension/content-script-redirect.js
          ./tucant-extension/bootstrap.bundle.min.js
          ./tucant-extension/bootstrap.min.css
          ./tucant-extension/icon.png
          ./tucant-extension/manifest.json
          ./tucant-extension/mobile.css
          ./tucant-extension/mobile.js
          ./tucant-extension/options.html
          ./tucant-extension/options.js
          ./tucant-extension/popup.html
          ./tucant-extension/popup.js
          ./tucant-extension/rules.json
          ./tucant-extension/screenshot.png
        ];

        extension-unpacked = pkgs.stdenv.mkDerivation {
          pname = "tucant-extension.zip";
          version = (lib.importJSON ./tucant-extension/manifest.json).version;

          src = lib.fileset.toSource {
              root = ./tucant-extension;
              fileset = fileset-extension;
          };

          installPhase = ''
            mkdir $out
            cp -r $src/. $out/
            cp -r ${myClient}/. $out/dist/
          '';
        };

        extension = pkgs.runCommand "tucant-extension.zip" {} ''
          cd ${extension-unpacked}
          ${pkgs.zip}/bin/zip -r $out *
          ${pkgs.strip-nondeterminism}/bin/strip-nondeterminism --type zip $out
        '';

        source-with-build-instructions = lib.fileset.toSource {
          root = unfilteredRoot;
          fileset = lib.fileset.unions [
            fileset
            fileset-extension
            ./flake.nix
            ./flake.lock
            ./Dockerfile
            ./README.md 
          ];
        };

        source = pkgs.runCommand "tucant-extension-source.zip" {} ''
          cd ${source-with-build-instructions}
          ${pkgs.zip}/bin/zip -r $out *
          ${pkgs.strip-nondeterminism}/bin/strip-nondeterminism --type zip $out
        '';

        source-unpacked = pkgs.runCommand "tucant-extension-source.zip" {} ''
          cp -r ${source-with-build-instructions} $out
        '';

        rustfmt = craneNightlyLib.buildPackage {
          src = pkgs.fetchFromGitHub {
            owner = "tucant";
            repo = "rustfmt";
            rev = "html-extractor-formatting";
            hash = "sha256-gj1Ac4TW1X4mj+AIqMB6X+WguNoJvgpgQXE62gQjRcM=";
          };
          doCheck = false;
        };

        yew-fmt = craneLib.buildPackage {
          src = pkgs.fetchFromGitHub {
            owner = "mohe2015";
            repo = "yew-fmt";
            rev = "patch-1";
            hash = "sha256-WECfuQ3mBzoRu8uzhf0v1mjT7N+iU+APWDj/u3H0FPU=";
          };
        };

        craneYewFmtLib = ((crane.mkLib pkgs).overrideToolchain rustNightlyToolchainFor).overrideScope (final: prev: {
          # We override the behavior of `mkCargoDerivation` by adding a wrapper which
          # will set a default value of `CARGO_PROFILE` when not set by the caller.
          # This change will automatically be propagated to any other functions built
          # on top of it (like `buildPackage`, `cargoBuild`, etc.)
          mkCargoDerivation = args: prev.mkCargoDerivation ({
            RUSTFMT = "${yew-fmt}/bin/yew-fmt";
          } // args);
        });
      in
      {
        checks = {
          # Build the crate as part of `nix flake check` for convenience
          inherit myServer myClient;

          # Run clippy (and deny all warnings) on the crate source,
          # again, reusing the dependency artifacts from above.
          #
          # Note that this is done as a separate derivation so that
          # we can block the CI if there are issues here, but not
          # prevent downstream consumers from building our crate by itself.
          my-app-clippy = craneLib.cargoClippy (commonArgs // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            # Here we don't care about serving the frontend
            CLIENT_DIST = "";
          });

          #my-app-fmt = craneNightlyLib.cargoFmt.override { rustfmt = rustfmt; } commonArgs;

          # uses both formatters
          my-app-fmt = craneYewFmtLib.cargoFmt.override { rustfmt = rustfmt; } commonArgs;
        };

        packages.default = myClient;
        packages.extension = extension;
        packages.extension-unpacked = extension-unpacked;
        packages.extension-source = source;
        packages.extension-source-unpacked = source-unpacked;
        packages.rustfmt = rustfmt;
        pkgs.yew-fmt = yew-fmt;

        apps.default = flake-utils.lib.mkApp {
          name = "server";
          drv = myServer;
        };

        packages.publish = let
        version = (lib.importJSON ./tucant-extension/manifest.json).version;
        in pkgs.writeShellScript "publish"
        ''
          set -ex
          mkdir -p out
          cd out
          # seems like chromium writes into the parent folder of the pack-extension argument
          chmod -R ug+rw tucant-extension-${version} || true
          rm -Rf tucant-extension-${version}
          cp -r ${extension-unpacked} tucant-extension-${version}
          ${pkgs.chromium}/bin/chromium --no-sandbox --pack-extension=tucant-extension-${version} --pack-extension-key=$CHROMIUM_EXTENSION_SIGNING_KEY
          chmod 644 tucant-extension-${version}.crx

          chmod -R ug+rw tucant-extension-${version}
          rm -Rf tucant-extension-${version}
          cp -r ${extension-unpacked} tucant-extension-${version}
          chmod -R ug+rw tucant-extension-${version}

          ${pkgs.web-ext}/bin/web-ext sign --channel unlisted --source-dir tucant-extension-${version} --upload-source-code ${source}
          chmod 644 web-ext-artifacts/tucant-${version}.xpi
          cp web-ext-artifacts/tucant-${version}.xpi tucant-extension-${version}.xpi
        '';

        devShells.default = craneNightlyLib.devShell {
          # Inherit inputs from checks.
          checks = self.checks.${system};

          RUSTFMT = "${yew-fmt}/bin/yew-fmt";

          shellHook = ''
            export CLIENT_DIST=$PWD/tucant-yew/dist;
          '';

          # Extra inputs can be added here; cargo and rustc are provided by default.
          packages = [
            trunk
            rustfmt
            yew-fmt
            pkgs.bashInteractive
            pkgs.diffoscope
          ];
        };
      });
}
