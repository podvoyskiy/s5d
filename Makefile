.PHONY: server server-auth client client-cli test test-server test-client test-lib build

server:
	cargo run --bin s5d

server-auth:
	cargo run --bin s5d -- --auth admin:12345

client:
	cargo run --bin s5d-client

client-cli:
	cargo run --bin s5d-client -- --target httpbin.org

test: test-server test-client test-lib

test-server:
	cargo test -p s5d

test-client:
	cargo test -p s5d-client

test-lib:
	cargo test -p s5d-lib

build:
	cargo build --release --target x86_64-unknown-linux-musl

s: server
sa: server-auth
c: client
cc: client-cli
t: test
ts: test-server
tc: test-client
tl: test-lib
b: build