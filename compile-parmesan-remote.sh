#!/bin/bash
#PBS -l select=1
#PBS -l ncpus=2
#PBS -l mem=4gb
#PBS -l scratch_local=10gb
#PBS -l walltime=00:10:00
#PBS -N parmesan-compilation

# initialize required modules
module fftw-3.3
#TODO rustc, cargo, ... tar?

# clean the SCRATCH when job finishes (and data are successfully copied out) or is killed
trap 'clean_scratch' TERM EXIT

# copy repositories (storage is shared via NFSv4)
DATAROOT="/storage/brno2/home/fakub/parallel-arithmetics-benchmark/bench-parmesan"

cp \
    $DATAROOT/bench-parmesan.tar.gz \
    $DATAROOT/parmesan.tar.gz \
    $SCRATCHDIR

# go for the computation
cd $SCRATCHDIR

# main command(s)
mkdir -p bench-parmesan
mkdir -p parmesan

tar -xzf bench-parmesan.tar.gz -C bench-parmesan
tar -xzf parmesan.tar.gz -C parmesan

cd bench-parmesan

RUSTFLAGS="-C target-cpu=native" cargo build --release

# copy output files (if needed)
cp target/release/bench-parmesan $DATADIR || export CLEAN_SCRATCH=false
