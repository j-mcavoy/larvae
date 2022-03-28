set dotenv-load

watch target:
    watchexec -e rs -r -w . just run {{target}}

serve:
    npx browser-sync start -c bs-config.js

test:
    cargo test
    cd crates/larvae_core && cargo test
run wasm: (build "--target wasm32-wasi")
    wagi -c modules.toml --env TEMPLATE_PATH="/templates" --log-dir ./logs
run-native:
    cd {{justfile_directory()}}; export TEMPLATE_PATH=$(pwd)/templates; cargo run

build target:
    cd {{justfile_directory()}}; cargo build --target {{target}}

push sample:
    wasm-to-oci push target/wasm32-wasi/debug/{{sample}}.wasm rustlinzwasm.azurecr.io/wagi-{{sample}}-oci:latest
