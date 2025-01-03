all: clean build run

run: clean build
	./target/main

clean:
	cargo clean
	rm -rf target

build:
	cargo build --release --lib
	mkdir -p target
	gcc -o target/main main.c -I. -L. ../../../target/release/libc.a