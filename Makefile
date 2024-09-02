build: build-userspace build-kernel
run: run-x86_64

build-kernel:
	cd kernel && cargo build --release

build-userspace:
	make -C user build

run-x86_64: export ARCH=x86_64
run-x86_64: build
	./scripts/runner.sh

clean:
	cd kernel && cargo clean
