# Example wasm32-wasi app

## Building & running

```shell
cargo build --target=wasm32-wasi
wasmedge run ./target/wasm32-wasi/debug/wasm-ctr.wasm # wasmedge version 0.13.5, also in nix dev shell
```

### Docker

```shell
docker buildx build --platform wasi/wasm32 -f docker/Dockerfile -t localhost/wasm-ctr:latest .
docker run --name wasm-ctr --rm -ti -p 127.0.0.1:8080:8080 localhost/wasm-ctr:latest
```

## Usage

```shell
curl -v http://127.0.0.1:8080/hash/sha3-256/random
```
