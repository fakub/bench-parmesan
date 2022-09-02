# run with default settings in Cargo.toml
all_light_8bit:
	cargo update -p parmesan
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "8bit all_light"
all_32bit:
	cargo update -p parmesan
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "32bit all"

analyze_4bit:
	cargo update -p parmesan
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "4bit all_light seq_analyze"
analyze_8bit:
	cargo update -p parmesan
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "8bit all_light seq_analyze"
analyze_16bit:
	cargo update -p parmesan
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "16bit all seq_analyze"
analyze_32bit:
	cargo update -p parmesan
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "32bit all seq_analyze"

# bench only selected feature
pbs:
	cargo update -p parmesan
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "8bit pbs"
add:
	cargo update -p parmesan
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "8bit add"
scm:
	cargo update -p parmesan
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "8bit scm"
sgn:
	cargo update -p parmesan
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "8bit sgn"
round:
	cargo update -p parmesan
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "8bit round"
max:
	cargo update -p parmesan
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "8bit max"
nn:
	cargo update -p parmesan
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "8bit nn"

mul_light_4bit:
	cargo update -p parmesan
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "4bit mul_light"
mul_light_8bit:
	cargo update -p parmesan
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "8bit mul_light"
mul_16bit:
	cargo update -p parmesan
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "16bit mul"
mul_32bit:
	cargo update -p parmesan
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "32bit mul"
squ_light_4bit:
	cargo update -p parmesan
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "4bit squ_light"
squ_light_8bit:
	cargo update -p parmesan
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "8bit squ_light"
squ_16bit:
	cargo update -p parmesan
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "16bit squ"
squ_32bit:
	cargo update -p parmesan
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "32bit squ"

concrete_4bit:
	cargo update -p parmesan
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "4bit concrete all_light"
concrete_8bit:
	cargo update -p parmesan
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "8bit concrete all_light"
concrete_16bit:
	cargo update -p parmesan
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "16bit concrete all"
concrete_32bit:
	cargo update -p parmesan
	RUSTFLAGS="-C target-cpu=native" cargo run --release --features "32bit concrete all"

# build-only
build:
	cargo update -p parmesan
	RUSTFLAGS="-C target-cpu=native" cargo build --release --features "32bit concrete all"

# run tests
test:
	cargo update -p parmesan
	RUSTFLAGS="-C target-cpu=native" cargo test --release
