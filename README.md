# BQN Language Server

## Building

Run `make install` to build the server and the resources it needs to directory `output/`

## Contents

`genhelp` is a helper software that runs through BQN help files, runs the BQN code in them and places the evaluated expression results into the files.

`lsp` is the language server implementation.

`editors` contains rough templates for both nvim-lspconfig and vscode.
