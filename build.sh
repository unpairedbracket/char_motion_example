cargo build --config profile.web.inherits='"dev"' --no-default-features --profile web-release --target wasm32-unknown-unknown --lib
wasm-bindgen --no-typescript --out-name char_motion_example --out-dir ./target/wasm32-unknown-unknown/web-release --target web ./target/wasm32-unknown-unknown/web-release/char_motion_example.wasm
wasm-opt --strip-debug -Os -o ./target/wasm32-unknown-unknown/web-release/char_motion_example_bg.wasm ./target/wasm32-unknown-unknown/web-release/char_motion_example_bg.wasm
mkdir -p ./web/build
rm ./web/build/*
cp ./target/wasm32-unknown-unknown/web-release/char_motion_example_bg.wasm ./web/build/
cp ./target/wasm32-unknown-unknown/web-release/char_motion_example.js ./web/build/
