all: clean build run

run: clean build
	./target/main

clean:
	cargo clean
	rm -rf target

build:
	cargo build --release --lib
	# Generate the C header file.
	cd ..; cbindgen  --crate c --output src/c.h --lang c; cd src
	mkdir -p target
	gcc -o target/main main.c -I. -L. ../../../target/release/libc.a