{
  description = "BQN LSP implementation";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
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
      in
      rec {
        packages = {
          default = packages.lsp;
          genhelp = naersk'.buildPackage {
            pname = "bqnlsp-genhelp";
            root = ./.;
            buildInputs = [ pkgs.cbqn ];
            cargoBuildOptions = x: x ++ [ "-p" "genhelp" ];
            cargoTestOptions = x: x ++ [ "-p" "genhelp" ];
            RUSTFLAGS = "-L ${pkgs.cbqn}/lib";
          };
          lsp = naersk'.buildPackage {
            pname = "bqnlsp";
            version = "20230507-1";
            name = "${packages.lsp.pname}-${packages.lsp.version}";
            root = ./.;
            buildInputs = [
              bqn
              pkgs.cbqn
              packages.genhelp
            ];
            cargoBuildOptions = x: x ++ [ "-p" "bqnlsp" ];
            cargoTestOptions = x: x ++ [ "-p" "bqnlsp" ];
            RUSTFLAGS = "-L ${pkgs.cbqn}/lib";
            BQNLSP_BQN_PATH = "${bqn}/";

            overrideMain = x: x // {
              preBuild = ''
                ${packages.genhelp}/bin/genhelp ${bqn} ./lsp/src/help
              '';
            };
            meta = with nixpkgs.lib; {
              homepage = "https://git.sr.ht/~detegr/bqnlsp";
              description = "BQN Language Server";
              license = licenses.gpl3Plus;
              maintainers = with maintainers; [ detegr ];
              platforms = platforms.all;
            };
          };
        };

        # nix run
        apps = {
          default = apps.lsp;
          lsp = flake-utils.lib.mkApp {
            name = "bqnlsp";
            drv = packages.lsp;
          };
        };

        # nix develop
        devShells = {
          default = pkgs.mkShell {
            RUSTFLAGS = "-L ${pkgs.cbqn}/lib";
            inputsFrom = builtins.attrValues self.packages.${system};
            nativeBuildInputs = [
              pkgs.rust-bin.stable.latest.default
              pkgs.rust-analyzer
            ];
          };
        };
      }
    );
}
