### SOCKS5 Proxy Server

**Linux only**

### Installation

1. Download binary:
    ```bash
    wget https://github.com/podvoyskiy/s5d/releases/latest/download/s5d
    ```

2. Make executable:
   ```sh
   chmod +x ./s5d
   ```

### Usage

```bash
./s5d                             # listen on 127.0.0.1:1080 (default)
./s5d --host 0.0.0.0 --port 9976  # listen on all interfaces
```

### Examples

```bash
# http GET via proxy
curl -x socks5://127.0.0.1:1080 http://httpbin.org/get

# https POST via proxy + dns resolving on proxy side
curl -x socks5h://127.0.0.1:1080 https://httpbin.org/post -X POST -d '{"key":"value"}'
```