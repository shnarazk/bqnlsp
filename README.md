# [BQN Language Server](https://sr.ht/~detegr/bqnlsp/)

## Building

### Nix

Nix is the preferred way to build the project. Install the [Nix package manager](https://nixos.org/download.html) and run:

```
nix --extra-experimental-features nix-command --extra-experimental-features flakes build 'sourcehut:~detegr/bqnlsp'
```

After the build, there will be a symlink to the Nix store called `result` which will contain the binary in `result/bin/bqnlsp`.

### build.bqn

- Update submodules of this repository with `git submodule update --init --recursive`.
- Clone [CBQN](https://github.com/dzaima/CBQN) and build it with `make shared-o3`.
- Run `build.bqn /path/to/CBQN` for automated build of the project where `/path/to/CBQN` contains the prebuilt CBQN shared object.

### Manually

- Update submodules of this repository with `git submodule update --init --recursive`.
- Clone [CBQN](https://github.com/dzaima/CBQN) and build it with `make shared-o3`.
- Set following environment variables:
  - `RUSTFLAGS="-L /path/to/CBQN"`
  - `LD_LIBRARY_PATH="/path/to/CBQN"` (use `DYLD_LIBRARY_PATH` on MacOS)
- Generate help files with: `cargo run --release --bin genhelp ./BQN ./lsp/src/help`
- Build bqnlsp with: `cargo build --release --bin bqnlsp`
- When installing the `bqnlsp` binary, distribute the CBQN shared object to the same directory and it'll load without messing with `LD_LIBRARY_PATH`.

## Contents

`genhelp` is a helper software that runs through BQN help files, runs the BQN code in them and places the evaluated expression results into the files.

`lsp` is the language server implementation.

`editors` contains rough templates for both nvim-lspconfig and vscode.

## How to use

Build the project with the aforementioned instructions.

See the documentation for [nvim-lspconfig](item/editors/neovim/nvim-lspconfig/README.md) or [vscode](item/editors/vscode/README.md) to see how to set up an editor to use it with.

## Demo

[![demo](https://asciinema.org/a/WTO2wmizmOjM0yOZbvbsSyZQx.svg)](https://asciinema.org/a/WTO2wmizmOjM0yOZbvbsSyZQx)
