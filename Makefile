# run with default settings in Cargo.toml
run:
	RUSTFLAGS="-C target-cpu=native" cargo run --release

# bench only selected feature
add:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --no-default-features --features "add"
sgn:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --no-default-features --features "sgn"
max:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --no-default-features --features "max"
mul:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --no-default-features --features "mul"
scm:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --no-default-features --features "scm"
nn:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --no-default-features --features "nn"

# build-only
build:
	RUSTFLAGS="-C target-cpu=native" cargo build --release

# run tests
test:
	RUSTFLAGS="-C target-cpu=native" cargo test --release
