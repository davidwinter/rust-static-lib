run: build
	cd rust-app && cargo run

build: build.cpp-lib build.rust

build.cpp-lib:
	cd cpp-lib \
		&& cmake -B ./build \
		&& cmake --build ./build

build.cpp-test-app:
	cd cpp-test-app \
		&& cmake -B ./build \
		&& cmake --build ./build

build.rust:
	cd rust-app \
		&& cargo build
