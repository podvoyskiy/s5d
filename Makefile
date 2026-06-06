.PHONY: server server-auth server-xor client-get client-post client-https client-xor test test-server test-client test-lib build

server:
	cargo run --bin s5d

server-auth:
	cargo run --bin s5d -- --auth admin:12345

server-xor:
	cargo run --bin s5d -- --xor 0xAA

client-get:
	cargo run --bin s5d-client -- --target http://34.234.10.121/get?key=value

client-post:
	cargo run --bin s5d-client -- --target http://httpbin.org/post --data '{"key":"value"}'

client-https:
	cargo run --bin s5d-client -- --target https://httpbin.org/post --data '{"key":"value"}'

client-xor:
	cargo run --bin s5d-client -- --xor 0xAA --target https://httpbin.org/post --data '{"key":"value"}'

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
sx: server-xor
c: client
cg: client-get
cp: client-post
ch: client-https
cx: client-xor
t: test
ts: test-server
tc: test-client
tl: test-lib
b: build