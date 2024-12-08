default: run

## Builds the app for current os-arch
build:
	cargo build --release

## Runs the app
run:
	@CARGO_INCREMENTAL=1 cargo +nightly fmt && make lint && cargo run

## Run clippy
lint:
	@find . -type f | grep '\/src\/.*\.rs'| xargs touch && cargo clippy --all-targets --workspace

## Fix lint
lint-fix:
	@cargo fix

fmt-dep:
	@rustup toolchain install nightly-aarch64-apple-darwin
	@rustup component add --toolchain nightly-aarch64-apple-darwin rustfmt

## Run format
fmt: fmt-dep
	@cargo +nightly fmt

## Analyse for unsafe usage - `cargo install cargo-geiger`
analyse:
	@cargo geiger

clean:
	@cargo clean

## Install release binary to $HOME/.local/bin
install: build
	@mkdir -p $$HOME/.local/bin
	@cp ./target/release/kubectl-peek $$HOME/.local/bin
	@cargo clean
