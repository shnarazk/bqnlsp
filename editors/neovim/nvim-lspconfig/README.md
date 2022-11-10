# BQN LSP for nvim-lspconfig

This is a way to add support for `bqnlsp` to `nvim-lspconfig`:

- Clone the `nvim-lspconfig` repository
- Copy the `bqnlsp.lua` file to `nvim-lspconfig/lua/lspconfig/server_configurations/bqnlsp.lua`
- Set up Neovim to use the local version of `nvim-lspconfig`. How to do this step depends on your plugin manager.
- Add `bqnlsp` to your LSP servers in your nvimrc.
