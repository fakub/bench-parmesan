# run with default settings in Cargo.toml
all_light_8bit:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "8bit all_light"
all_32bit:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "32bit all"

# bench only selected feature
pbs:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "8bit pbs"
add:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "8bit add"
sgn:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "8bit sgn"
round:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "8bit round"
max:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "8bit max"
mul_light:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "8bit mul_light"
squ_light:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "8bit squ_light"
scm:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "8bit scm"
nn:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "8bit nn"
concrete_4bit:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "4bit concrete all_light"
concrete_8bit:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "8bit concrete all_light"
concrete_16bit:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "16bit concrete all"
concrete_32bit:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "32bit concrete all"

# build-only
build:
	RUSTFLAGS="-C target-cpu=native" cargo build --release --features "32bit concrete all"

# run tests
test:
	RUSTFLAGS="-C target-cpu=native" cargo test --release
