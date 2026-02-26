.POSIX:
.SILENT:
.PHONY: \
	all \
	audit \
	build \
	cargo-check \
	clean \
	clean-archive \
	clean-cargo \
	clean-example \
	clean-ports \
	clippy \
	crit \
	doc \
	docker-build \
	docker-test \
	docker-push \
	install \
	lint \
	port \
	publish \
	rustfmt \
	test \
	uninstall
.IGNORE: \
	clean \
	clean-archive \
	clean-cargo \
	clean-example \
	clean-ports

VERSION=0.0.10
BANNER=chandler-$(VERSION)

all: install

audit:
	cargo audit

build: lint test
	cargo build --release

cargo-check:
	cargo check

clean: \
	clean-archive \
	clean-cargo \
	clean-example \
	clean-ports

clean-archive:
	rm .crit/bin/$(BANNER).tgz

clean-cargo:
	cargo clean

clean-example:
	rm -f example/Cargo.lock
	rm -rf example/target
	find example -iname '*.tgz' -print -delete

clean-ports:
	crit -c

clippy:
	cargo clippy

crit:
	crit -b $(BANNER)

doc:
	cargo doc

docker-build:
	tuggy -t n4jm4/chandler --load

docker-push:
	tuggy -t n4jm4/chandler -a n4jm4/chandler:$(VERSION) --push

docker-test:
	tuggy -t n4jm4/chandler:test --load
	tuggy -t n4jm4/chandler:test --push

install:
	cargo install --force --path .

lint: \
	cargo-check \
	clippy \
	doc \
	rustfmt

port:
	./port -C .crit/bin -a chandler $(BANNER)

publish:
	cargo publish

rustfmt:
	cargo fmt

test:
	cargo test

uninstall:
	cargo uninstall chandler
