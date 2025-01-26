start-dev:
  cargo run --features bevy/dynamic_linking

start-rel:
  cargo run --features bevy/dynamic_linking --release

build-windows:
  CARGO_FEATURE_PURE=1 cargo build --target=x86_64-pc-windows-gnu --release
  cp -r ./assets/ ./target/release/assets/

build-macos:
  cargo build --release --target x86_64-apple-darwin
  cp -r ./assets/ ./target/release/assets/

run-builded-release: build-macos
  ./target/release/game

bundle-windows: build-windows
  cd target/release/ && zip -r ../../ggj-2025-windows.zip ./game ./assets/

bundle-macos: build-macos
  cd target/release/ && zip -r ../../ggj-2025-macos.zip ./game ./assets/

bundle: bundle-macos bundle-windows
