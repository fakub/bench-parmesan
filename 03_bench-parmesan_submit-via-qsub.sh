#!/bin/bash
#PBS -l select=1
#PBS -l ncpus=2
#PBS -l mem=4gb
#PBS -l scratch_local=10gb
#PBS -l walltime=00:01:00
#PBS -N parmesan-benchmark

# initialize required modules (if needed)
# module xyz


# clean the SCRATCH when job finishes (and data are successfully copied out) or is killed
trap 'clean_scratch' TERM EXIT

# copy keys and binary (storage is shared via NFSv4)
DATAROOT="/storage/brno2/home/fakub/parallel-arithmetics-benchmark"

cp \
    $DATAROOT/keys/bootstrapping-keys__n-560_k-1_N-1024_gamma-10_l-2.key \
    $DATAROOT/keys/key-switching-keys__n-560_k-1_N-1024_kappa-1_t-16.key \
    $DATAROOT/bench-parmesan \
    $SCRATCHDIR

# go for the computation
cd $SCRATCHDIR

# main command(s)
./bench-parmesan

# copy output files (if needed)
# cp output $DATADIR || export CLEAN_SCRATCH=false
