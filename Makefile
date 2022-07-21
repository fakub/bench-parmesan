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
concrete_4bit:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "concrete c4 all_light"
concrete_8bit:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "concrete c8 all_light"
concrete_16bit:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "concrete c16 all"
concrete_32bit:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "concrete c32 all"

# build-only
build:
	RUSTFLAGS="-C target-cpu=native" cargo build --release --features "all"

# run tests
test:
	RUSTFLAGS="-C target-cpu=native" cargo test --release
