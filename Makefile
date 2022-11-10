-include helpfiles.mk

all: ${HELP_FILES} target/debug/bqnlsp

PREFIX ?= output/
install: target/debug/bqnlsp ${HELP_FILES}
	mkdir -p ${PREFIX}/BQN/src
	cp target/debug/bqnlsp ${PREFIX}
	cp -r help/ ${PREFIX}
	cp -r BQN/src/* ${PREFIX}/BQN/src
	# FIXME: howto do this properly??
	cp $(shell find target/debug -name libcbqn.so | head -n1) ${PREFIX}

clean:
	-rm -rf ${PREFIX}
	-rm -rf help
	-rm -rf BQN

BQN/src/c.bqn:
	git submodule update --init --recursive

target/debug/bqnlsp: $(wildcard lsp/src/*.rs)
	cargo build --manifest-path=lsp/Cargo.toml

${HELP_FILES}: BQN/src/c.bqn
	cargo run --release --bin genhelp ./BQN ./help

.PHONY: install
