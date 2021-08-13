#!/bin/bash
#PBS -l select=1:ncpus=56:mem=1gb:scratch_local=1gb:cluster=samson
# consider:
#PBS -l place=exclhost
#PBS -l walltime=00:30:00
#PBS -N parmesan-bench
#PBS -j oe
#PBS -m ae
#PBS -M fakubo@gmail.com

# describtion from 'man qsub':
# -N ... declares a name for the job. The name specified may be up to and including 15 characters in length. It
#        must consist of printable, non white space characters with the first character alphabetic.
# -q ... defines the destination of the job (queue)
# -l ... defines the resources that are required by the job
# -j oe ... standard error stream of the job will be merged with the standard output stream
# -m ae ...  mail is sent when the job aborts or terminates
# job array: $ qsub -J 2-7:2 script.sh

# initialize required modules
module add fftw-3.3
# possibly other for Rust
# Rust installed via classical way:
# $ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# after copying, fix with
# $ rustup default stable

# clean the SCRATCH when job finishes (and data are successfully copied out) or is killed
trap 'clean_scratch' TERM EXIT

# go to the right place
test -n "$SCRATCHDIR" || { echo >&2 "Variable SCRATCHDIR is not set!"; exit 1; }
cd $SCRATCHDIR

DATA_DIR="/storage/brno2/home/fakub/parallel-arithmetics-benchmark"

# copy keys
cp \
    $DATA_DIR/keys/secret-key__n-560.key \
    $DATA_DIR/keys/bootstrapping-keys__n-560_k-1_N-1024_gamma-10_l-2.key \
    $DATA_DIR/keys/key-switching-keys__n-560_k-1_N-1024_kappa-1_t-16.key \
    . || { echo >&2 "Error while copying input file(s)!"; exit 2; }

#~ cp -r \
    #~ folders
    #~ . || { echo >&2 "Error while copying input folder(s)!"; exit 3; }

# clone repos
git clone --depth=1 --branch negacyclic git@github.com:fakub/concrete.git
git clone --depth=1 --branch nn git@gitlab.fit.cvut.cz:klemsjak/parmesan.git
git clone --depth=1 git@gitlab.fit.cvut.cz:klemsjak/bench-parmesan.git

# compile (cascadelake processor type is at Samson)
cd bench-parmesan
RUSTFLAGS="-C target-cpu=cascadelake" cargo build --release
# save for future re-use
cp target/release/bench-parmesan $DATA_DIR || { export CLEAN_SCRATCH=false; echo >&2 "Result file(s) copying failed! Try to copy them manually."; exit 4; }
mv target/release/bench-parmesan ..
cd ..

# main command(s)
./bench-parmesan || { echo >&2 "Calculation ended up erroneously (with a code $?) !!"; exit 5; }

# copy output files (if any)
# cp output $DATA_DIR || { export CLEAN_SCRATCH=false; echo >&2 "Result file(s) copying failed! Try to copy them manually."; exit 6; }
