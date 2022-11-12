HELP_FILES := $(patsubst BQN%,lsp/src%,$(subst BQN/help/README.md,,$(wildcard BQN/help/*.md)))

all: target/debug/bqnlsp

PREFIX ?= output/
install: target/debug/bqnlsp
	mkdir -p ${PREFIX}/BQN/src
	cp target/debug/bqnlsp ${PREFIX}
	cp -r BQN/src/* ${PREFIX}/BQN/src
	# FIXME: howto do this properly??
	cp $(shell find target/debug -name libcbqn.so | head -n1) ${PREFIX}

clean:
	-rm -rf ${PREFIX}
	-rm -rf lsp/src/help/*.md

target/debug/bqnlsp: $(wildcard lsp/src/*.rs) ${HELP_FILES}
	@if [ ! -d BQN ]; then \
		echo "BQN directory not found"; \
		echo "Try running \`git submodule init --update --recursive\`"; \
		exit 1; \
	fi
	cargo build --manifest-path=lsp/Cargo.toml

${HELP_FILES}: BQN/src/c.bqn
	cargo run --release --bin genhelp ./BQN ./lsp/src/help

.PHONY: install

# Disable implicit rules
.SUFFIXES:
