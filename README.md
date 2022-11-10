# BQN Language Server

## Building

Run `make install` to build the server and the resources it needs to directory `output/`

## Contents

`genhelp` is a helper software that runs through BQN help files, runs the BQN code in them and places the evaluated expression results into the files.

`lsp` is the language server implementation.

`editors` contains rough templates for both nvim-lspconfig and vscode.

## How to use

Build the project with `make install`.

Add the `bqnlsp` executable in `$PATH`. The executable is `output/bqnlsp`, created by running `make install`.

See the documentation for [nvim-lspconfig](item/editors/neovim/nvim-lspconfig/README.md) or [vscode](item/editors/vscode/README.md) to see how to set up an editor to use it with.

## Demo

[![demo](https://asciinema.org/a/WTO2wmizmOjM0yOZbvbsSyZQx.svg)](https://asciinema.org/a/WTO2wmizmOjM0yOZbvbsSyZQx)
