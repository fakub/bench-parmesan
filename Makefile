# run with default settings in Cargo.toml
light:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "all_light"
all:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "all"

# bench only selected feature
pbs:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "pbs"
add:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "add"
sgn:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "sgn"
round:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "round"
max:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "max"
mul:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "mul"
squ:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "squ"
scm:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "scm"
nn:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "nn"
concrete_4_light:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --no-default-features --features "c4 all_light"
concrete_8_light:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --no-default-features --features "c8 all_light"
concrete_16:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --no-default-features --features "c16 all_light"
concrete_32:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --no-default-features --features "c32 all_light"

# build-only
build:
	RUSTFLAGS="-C target-cpu=native" cargo build --release --features "all"

# run tests
test:
	RUSTFLAGS="-C target-cpu=native" cargo test --release
