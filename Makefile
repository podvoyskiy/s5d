.PHONY: server server-auth server-xor client client-https client-xor client-proxy client-tun client-tun-permissions test test-server test-client test-lib build

server:
	cargo run --bin s5d

server-auth:
	cargo run --bin s5d -- --auth admin:12345

server-xor:
	cargo run --bin s5d -- --xor 0xAA

client:
	cargo run --bin s5d-client -- --target http://34.234.10.121/get?key=value

client-https:
	cargo run --bin s5d-client -- --target https://httpbin.org/post --data '{"key":"value"}'

client-xor:
	cargo run --bin s5d-client -- --xor 0xAA --target https://httpbin.org/post --data '{"key":"value"}'

client-proxy:
	cargo run --bin s5d-client -- --mode proxy

client-tun:
	cargo build --release --target x86_64-unknown-linux-musl --bin s5d-client
	target/x86_64-unknown-linux-musl/release/s5d-client --mode tun

client-tun-permissions:
	sudo setcap cap_net_admin=+ep target/x86_64-unknown-linux-musl/release/s5d-client

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
ch: client-https
cx: client-xor
cp: client-proxy
ct: client-tun
ctp: client-tun-permissions
t: test
ts: test-server
tc: test-client
tl: test-lib
b: build