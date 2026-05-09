.PHONY: server server-auth test build

server:
	cargo run --bin s5d

server-auth:
	cargo run --bin s5d -- --auth admin:12345

test:
	cargo test -p s5d

build:
	cargo build --release --target x86_64-unknown-linux-musl

s: server
sa: server-auth
t: test
b: build