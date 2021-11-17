#!/bin/bash
#PBS -l select=1:ncpus=56:mem=1gb:scratch_local=1gb:cluster=samson
#
#   Name        CPU's                           Queue                           Threads                     Rust CPU family         Clock
#
#   samson      4x Intel Xeon Platinum 8280     cerit-pbs.cerit-sc.cz           4x56 threads (224)          cascadelake             2.70 - 4.00 GHz
#   eltu        4x Intel Xeon Platinum 8260     elixir-pbs.elixir-czech.cz      4x48 threads (192)          cascadelake             2.40 - 3.90 GHz
#   elwe        2x AMD EPYC 7532                elixir-pbs.elixir-czech.cz      2x64 threads (128)          znver2                  2.40 - 3.30 GHz
#   kirke_x     2x AMD EPYC 7532                meta-pbs.metacentrum.cz         dtto
#
# consider: #PBS -l place=exclhost
#
#PBS -l walltime=00:10:00
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

# initialize required modules (if any)
module add fftw/fftw-3.3.8-intel-19.0.4-532p634

# clean the SCRATCH when job finishes (and data are successfully copied out) or is killed
trap 'clean_scratch' TERM EXIT

# go to the right place
test -n "$SCRATCHDIR" || { echo >&2 "Variable SCRATCHDIR is not set!"; exit 1; }
cd $SCRATCHDIR

DATA_DIR="/storage/brno2/home/fakub/parallel-arithmetics-benchmark"

# copy keys & pre-compiled binary:   bench-parmesan_znver2-AMD   or   bench-parmesan_cascadelake-XEON
cp \
    $DATA_DIR/keys/secret-key__n-560.key \
    $DATA_DIR/keys/bootstrapping-keys__n-560_k-1_N-1024_gamma-10_l-2.key \
    $DATA_DIR/keys/key-switching-keys__n-560_k-1_N-1024_kappa-1_t-16.key \
    $DATA_DIR/bench-parmesan_cascadelake-XEON \
    . || { echo >&2 "Error while copying input file(s)!"; exit 2; }

#~ cp -r \
    #~ folders
    #~ . || { echo >&2 "Error while copying input folder(s)!"; exit 3; }

# run main command(s)
#TODO dstat --cpu-use -t > cpu-use_XEON.log &
./bench-parmesan_cascadelake-XEON || { echo >&2 "Calculation ended up erroneously (with a code $?) !!"; exit 5; }

# copy output files (if any)
# cp output $DATA_DIR || { export CLEAN_SCRATCH=false; echo >&2 "Result file(s) copying failed! Try to copy them manually."; exit 6; }
