#!/bin/bash

#
#   Intial Steps
#
# Install Rust:
# $ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# In case the bin folder is copied, it needs to be "fixed" with
# $ rustup default stable
#
# Clone repos:
# $ git clone --depth=1 --branch negacyclic https://github.com/fakub/concrete.git
# $ git clone --depth=1                     https://github.com/fakub/parmesan.git
# $ git clone --depth=1                     https://gitlab.fit.cvut.cz/klemsjak/bench-parmesan.git
# $ git clone --depth=1 --branch 100ms      https://gitlab.fit.cvut.cz/klemsjak/dstat-with-short-intervals.git
# $ mv concrete concrete-lib

#
#   Update all repos
#
cd concrete-lib
git checkout negacyclic
git pull

cd ../parmesan
git checkout master
git pull

cd ../bench-parmesan
git checkout master
git pull

cd ../dstat-with-short-intervals
git checkout 100ms
git pull

cd ..

#
#   Add FFTW module
#
module add fftw/fftw-3.3.8-intel-19.0.4-532p634
#~ module add fftw/fftw-3.3.8-intel-20.0.0-au2vxr2

#
#   Compile
#
rm -r bin
mkdir bin
cd bench-parmesan

# for Intel Xeon (and copy)
#~ export RUSTFLAGS="-C target-cpu=cascadelake"

#~ cargo build --release
#~ mv target/release/bench-parmesan ../bin/bench-parmesan_ALL_cascadelake-XEON
#~ cargo build --release --no-default-features --features "pbs"
#~ mv target/release/bench-parmesan ../bin/bench-parmesan_PBS_cascadelake-XEON
#~ cargo build --release --no-default-features --features "add"
#~ mv target/release/bench-parmesan ../bin/bench-parmesan_ADD_cascadelake-XEON
#~ cargo build --release --no-default-features --features "sgn"
#~ mv target/release/bench-parmesan ../bin/bench-parmesan_SGN_cascadelake-XEON
#~ cargo build --release --no-default-features --features "max"
#~ mv target/release/bench-parmesan ../bin/bench-parmesan_MAX_cascadelake-XEON
#~ cargo build --release --no-default-features --features "mul"
#~ mv target/release/bench-parmesan ../bin/bench-parmesan_MUL_cascadelake-XEON
#~ cargo build --release --no-default-features --features "squ"
#~ mv target/release/bench-parmesan ../bin/bench-parmesan_SQU_cascadelake-XEON
#~ cargo build --release --no-default-features --features "scm"
#~ mv target/release/bench-parmesan ../bin/bench-parmesan_SCM_cascadelake-XEON
#~ cargo build --release --no-default-features --features "nn"
#~ mv target/release/bench-parmesan ../bin/bench-parmesan_NN_cascadelake-XEON

# for AMD EPYC (and copy)
export RUSTFLAGS="-C target-cpu=znver2"

cargo build --release --no-default-features --features "all"
mv target/release/bench-parmesan ../bin/bench-parmesan_ALL_BEN_znver2-AMD
cargo build --release --no-default-features --features "all log_ops"
mv target/release/bench-parmesan ../bin/bench-parmesan_ALL_LOG_znver2-AMD
#~ cargo build --release --no-default-features --features "pbs"
#~ mv target/release/bench-parmesan ../bin/bench-parmesan_PBS_znver2-AMD
#~ cargo build --release --no-default-features --features "add"
#~ mv target/release/bench-parmesan ../bin/bench-parmesan_ADD_znver2-AMD
#~ cargo build --release --no-default-features --features "sgn"
#~ mv target/release/bench-parmesan ../bin/bench-parmesan_SGN_znver2-AMD
#~ cargo build --release --no-default-features --features "max"
#~ mv target/release/bench-parmesan ../bin/bench-parmesan_MAX_znver2-AMD
#~ cargo build --release --no-default-features --features "mul"
#~ mv target/release/bench-parmesan ../bin/bench-parmesan_MUL_znver2-AMD
#~ cargo build --release --no-default-features --features "squ"
#~ mv target/release/bench-parmesan ../bin/bench-parmesan_SQU_znver2-AMD
#~ cargo build --release --no-default-features --features "scm"
#~ mv target/release/bench-parmesan ../bin/bench-parmesan_SCM_znver2-AMD
#~ cargo build --release --no-default-features --features "nn"
#~ mv target/release/bench-parmesan ../bin/bench-parmesan_NN_znver2-AMD
