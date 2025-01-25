start-dev:
  cargo run --features bevy/dynamic_linking

start-rel:
  cargo run --features bevy/dynamic_linking --release

run-builded-release:
  cargo b --release
  cp -r ./assets/ ./target/release/assets/
  ./target/release/game
