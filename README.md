### SOCKS5 Proxy Server

**Linux only** (other OS not tested)

### Installation

#### Option 1: Cargo

***Requires:***
- Rust
- build-essential (Debian/Ubuntu: `sudo apt install build-essential`)

```sh
cargo install s5d
```

#### Option 2: Prebuilt binary (no dependencies)

```sh
wget https://github.com/podvoyskiy/s5d/releases/latest/download/s5d
chmod +x ./s5d
```

### Usage

```sh
s5d                             # listen on 127.0.0.1:1080 (default)
s5d --host 0.0.0.0 --port 9976  # listen on all interfaces
s5d --auth admin:12345          # with auth
```

> **Note:** If using prebuilt binary, replace `s5d` with `./s5d`

### Examples

```sh
curl -x socks5h://127.0.0.1:1080 https://httpbin.org/post -X POST -d '{"key":"value"}'
curl -x socks5://127.0.0.1:1080 http://httpbin.org/get
curl -x socks5://admin:12345@127.0.0.1:1080 http://httpbin.org/get
```

> **Note:** Use `socks5h://` for DNS resolving on the proxy side, `socks5://` for client-side DNS