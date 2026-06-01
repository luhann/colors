target_dir := join(home_dir(), "bin")

binary := "colorctl"

default:
    @just --list

# Clean cargo artefacts
clean:
  cargo clean

# Build everything
build:
    cargo build --release

# Install colorctl 
install: build
    @echo "Building and installing {{binary}}..."
    @mkdir -p {{target_dir}}
    @install -vDm755 "target/x86_64-unknown-linux-musl/release/{{binary}}" "{{target_dir}}/{{binary}}"; \

# Run all tests in release mode
test:
  cargo test --release
