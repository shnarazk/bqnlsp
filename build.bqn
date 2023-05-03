#!/usr/bin/env bqn

Fail ← {•Out¨𝕩 ⋄ •Exit 1}

# TODO: Detect the OS and act accordingly
(Fail∘⋈∘"Usage: build.bqn path/to/libcbqn.(so|dylib|dll)")⍟{𝕩<1} ≠•args

libcbqn ← ⊑•args
env ← "env"‿"-S"‿("RUSTFLAGS=""-L"∾libcbqn∾""" LD_LIBRARY_PATH="""∾libcbqn∾"""")

(Fail∘"BQN directory not found."‿"Try running `git submodule update --init --recursive` first.")⍟(¬•file.Exists) "BQN/src"

•Out "Building genhelp"
code‿out‿err ← •SH env∾"cargo"‿"build"‿"--release"‿"--bin"‿"genhelp"
(Fail∘⋈∘"Invalid path to libcbqn.so")⍟(+´"cannot find -lcbqn"⍷err) @
(Fail∘"Failed to build genhelp:"‿err)⍟{𝕩≠0} code

•Out "Generating help pages"
code‿out‿err ↩ •SH env∾"cargo"‿"run"‿"--bin"‿"genhelp"‿"./BQN"‿"./lsp/src/help"

(Fail∘"Failed to generate help pages:"‿err)⍟{𝕩≠0} code

•Out "Building bqnlsp"
code‿out‿err ↩ •SH env∾"cargo"‿"build"‿"--release"‿"--bin"‿"bqnlsp"
(Fail∘⋈∘"Invalid path to libcbqn.so")⍟(+´"cannot find -lcbqn"⍷err) @
(Fail∘"Failed to build bqnlsp:"‿err)⍟{𝕩≠0} code

•Out "Built: target/release/bqnlsp"
