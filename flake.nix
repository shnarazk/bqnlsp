{
  description = "BQN LSP implementation";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = { self, nixpkgs, rust-overlay, naersk, flake-utils }:
    let
      pkgsForSystem = system: import nixpkgs {
        inherit system;
        overlays = [
          (import rust-overlay)
        ];
      };
    in
    flake-utils.lib.eachDefaultSystem
    (system:
      let pkgs = pkgsForSystem system;
          naersk' = pkgs.callPackage naersk { };
          bqn = pkgs.fetchFromGitHub {
            name = "BQN";
            owner = "mlochbaum";
            repo = "BQN";
            rev = "c76eeded5b0b39d06085f6f7a8f12456d4b01a30";
            sha256 = "sha256-I0TMERE0VK2NrbEvKmOfzQYK4UwaRbcNfA28XjmuFr0=";
          };
          bytecode = pkgs.fetchFromGitHub {
            name = "bqnlsp-cbqn-bytecode";
            owner = "dzaima";
            repo = "cbqnBytecode";
            rev = "master";
            sha256 = "sha256-IOhxcfGmpARiTdFMSpc+Rh8VXtasZdfP6vKJzULNxAg=";
          };
          cbqn-shared-object = if
            pkgs.stdenv.hostPlatform.isDarwin then "libcbqn.dylib" else if
            pkgs.stdenv.hostPlatform.isWindows then "cbqn.dll"
            else "libcbqn.so";
          libcbqn = pkgs.llvmPackages.stdenv.mkDerivation {
            name = "bqnlsp-libcbqn";
            src = pkgs.fetchFromGitHub {
              owner = "dzaima";
              repo = "CBQN";
              rev = "v0.2.0";
              sha256 = "sha256-M9GTsm65DySLcMk9QDEhImHnUvWtYGPwiG657wHg3KA=";
            };

            nativeBuildInputs = [
              pkgs.bash
              pkgs.coreutils
            ];

            buildInputs = [
              pkgs.libffi
            ];

            dontConfigure = true;

            preBuild = ''
              mkdir -p build/bytecodeLocal/gen
              cp -r ${bytecode}/gen/* build/bytecodeLocal/gen
              patchShebangs build/build
            '';

            makeFlags = [
              "shared-o3"
            ];

            installPhase = (if pkgs.stdenv.hostPlatform.isDarwin then ''
              install_name_tool -id $out/${cbqn-shared-object} ${cbqn-shared-object}
            '' else "") + ''
              mkdir $out
              mv ${cbqn-shared-object} $out
            '';
          };
      in
      rec {
        defaultPackage = packages.lsp;
        packages = {
          genhelp = naersk'.buildPackage {
            pname = "bqnlsp-genhelp";
            root = ./.;
            buildInputs = [ libcbqn ];
            cargoBuildOptions = x: x ++ [ "-p" "genhelp" ];
            cargoTestOptions = x: x ++ [ "-p" "genhelp" ];
            RUSTFLAGS = "-L ${libcbqn}";
          };
          lsp = naersk'.buildPackage {
            pname = "bqnlsp";
            root = ./.;
            buildInputs = [
              bqn
              libcbqn
              packages.genhelp
            ];
            cargoBuildOptions = x: x ++ [ "-p" "bqnlsp" ];
            cargoTestOptions = x: x ++ [ "-p" "bqnlsp" ];
            RUSTFLAGS = "-L ${libcbqn}";
            BQNLSP_BQN_PATH = "${bqn}/";

            overrideMain = x: x // {
              preBuild = ''
                ${packages.genhelp}/bin/genhelp ${bqn} ./lsp/src/help
              '';
            };
          };
        };

        # nix run
        defaultApp = apps.lsp;
        apps.lsp = flake-utils.lib.mkApp {
          name = "bqnlsp";
          drv = packages.lsp;
        };

        # nix develop
        devShell = pkgs.mkShell {
          RUSTFLAGS = "-L ${libcbqn}";
          inputsFrom = builtins.attrValues self.packages.${system};
          nativeBuildInputs = [
            pkgs.rust-bin.stable.latest.default
            pkgs.rust-analyzer
          ];
        };
      }
    );
}
