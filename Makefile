.PHONY: server server-auth client test build

server:
	cargo run --bin s5d

server-auth:
	cargo run --bin s5d -- --auth admin:12345

client:
	cargo run --bin s5c

test:
	cargo test -p s5d && cargo test -p s5

build:
	cargo build --release --target x86_64-unknown-linux-musl

s: server
sa: server-auth
c: client
t: test
b: build