cargo build --release --target wasm32-unknown-unknown -Zbuild-std=std,panic_abort
wasm-bindgen --target web --out-dir pkg target/wasm32-unknown-unknown/release/webos.wasm
mkdir -p _site
cp index.html _site/
cp -r pkg _site/pkg
