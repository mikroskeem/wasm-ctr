FROM scratch

COPY ./target/wasm32-wasi/debug/wasm-ctr.wasm /wasm-ctr.wasm

EXPOSE 8080
ENTRYPOINT ["/wasm-ctr.wasm"]
