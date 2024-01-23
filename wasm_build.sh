echo "building app"
cargo build --release --target wasm32-unknown-unknown
cd ..
echo "wasm bindgen"
wasm-bindgen --out-name wasm_bloons_randomizer --out-dir bloons_randomizer_web/bloons_randomizer/target --target web bloons_randomizer/target/wasm32-unknown-unknown/release/bloons_randomizer.wasm
echo "finished"