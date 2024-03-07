.PHONY: build						\
	unit-tests 						\
	build-kernel 					\
	build-servers 				\
	build-userspace				\
	build-book						\
	build-docs						\
	check-clippy					\
	check-format					\
	unit-tests-kernel 		\
	unit-tests-servers 		\
	unit-tests-userspace 	\
	integration-tests			\
	run-i686						 	\
	run-x86_64 						\
	run-aarch64 					\
	clean									\
	help

run: run-x86_64
build: build-kernel build-servers build-userspace
unit-tests: unit-tests-kernel unit-tests-servers unit-tests-userspace

build-kernel:
	cd kernel && cargo build --release

build-servers:
	

build-userspace:
	

build-book:
	cd book && mdbook build
	
build-docs:
	mkdir -p docs/kernel
	RUSTDOCFLAGS="-Zunstable-options --enable-index-page" && \
	cd kernel && cargo doc 				\
		--document-private-items 		\
		--all-features 							\
		--keep-going 								\
		--workspace 								\
		--no-deps 									\
		--release

	mv kernel/target/x86_64/doc/* docs/kernel/
	
check-clippy:
	cd kernel && cargo clippy --all-features -- -D warnings

check-format:
	cd kernel && cargo fmt --all -- --check


unit-tests-kernel:
	# cd kernel && cargo test --release --target=x86_64-unknown-linux-gnu -Z build-std

unit-tests-servers:
	

unit-tests-userspace:
	

integration-tests:
	# QEMU_FLAGS="-nographic" && run
	
run-i686: export ARCH=i686
run-i686: build
	./scripts/runner.sh

run-x86_64: export ARCH=x86_64
run-x86_64: build
	./scripts/runner.sh

run-aarch64: export ARCH=aarch64
run-aarch64: build
	./scripts/runner.sh

clean:
	cd kernel && cargo clean

help:
	@echo "WIP"
