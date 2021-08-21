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
# $ git clone --depth=1                     https://gitlab.fit.cvut.cz/klemsjak/parmesan.git
# $ git clone --depth=1                     https://gitlab.fit.cvut.cz/klemsjak/bench-parmesan.git
# $ mv concrete concrete-lib

#
#   Update all repos
#
cd concrete-lib
git pull
cd ../parmesan
git pull
cd ../bench-parmesan
git pull
cd ..

#
#   Add FFTW module
#
module add fftw/fftw-3.3.8-intel-19.0.4-532p634

#
#   Compile
#
cd bench-parmesan
# for Intel Xeon (and copy)
RUSTFLAGS="-C target-cpu=cascadelake" cargo build --release
mv target/release/bench-parmesan ../bench-parmesan_cascadelake-XEON
# for AMD EPYC (and copy)
RUSTFLAGS="-C target-cpu=znver2" cargo build --release
mv target/release/bench-parmesan ../bench-parmesan_znver2-AMD
# go back
cd ..
