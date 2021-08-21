#!/bin/bash
#PBS -l select=1:ncpus=1:mem=200mb:scratch_local=200mb:cluster=elwe
#PBS -l walltime=00:01:00
#PBS -N processor-info
#PBS -j oe
#PBS -m ae
#PBS -M fakubo@gmail.com

# clean the SCRATCH when job finishes (and data are successfully copied out) or is killed
trap 'clean_scratch' TERM EXIT

# copy rust binaries
FAKUB_HOME="/storage/brno2/home/fakub"
echo ">>> Scratch dir: $SCRATCHDIR"
cp -r \
    $FAKUB_HOME/.cargo/bin/ \
    $SCRATCHDIR
export PATH="$SCRATCHDIR/bin:$PATH"   # export PATH="$HOME/.cargo/bin:$PATH"
echo ">>> PATH: $PATH"

# go for the computation
cd $SCRATCHDIR

# version of GLIBC
#~ echo ">>> ldd --version"
#~ ldd --version

echo ">>> which rustc"
which rustc

echo ">>> rustup --version"
rustup --version

echo ">>> rustup default stable"
rustup default stable

echo
echo ">>> rustc --print target-cpus"
echo "--------------------------------------------------------------------------------"

rustc --print target-cpus


echo
echo ">>> cat /proc/cpuinfo"
echo "--------------------------------------------------------------------------------"

cat /proc/cpuinfo


#~ echo
#~ echo ">>> lshw"
#~ echo "--------------------------------------------------------------------------------"

#~ lshw
