set -e && RUSTFLAGS='-C link-arg=-s' cargo build -p certy-cert --target wasm32-unknown-unknown --release
set -e && RUSTFLAGS='-C link-arg=-s' cargo build -p certy-cecareer --target wasm32-unknown-unknown --release

cp target/wasm32-unknown-unknown/release/*.wasm ./res/