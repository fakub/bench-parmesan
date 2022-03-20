# run with default settings in Cargo.toml
default:
	RUSTFLAGS="-C target-cpu=native" cargo run --release

# bench only selected feature
pbs:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --no-default-features --features "pbs"
add:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --no-default-features --features "add"
sgn:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --no-default-features --features "sgn"
max:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --no-default-features --features "max"
mul:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --no-default-features --features "mul"
squ:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --no-default-features --features "squ"
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
