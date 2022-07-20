#!/bin/bash
#
# with hyperthreading (only possible with place=exclhost; ncpus means CPU cores, this makes 128 threads, however, it is slower):
# #PBS -l select=1:ncpus=64:hyperthreading=True:mem=1gb:scratch_local=1gb:cluster=halmir
# #PBS -l place=exclhost
#
# no hyperthreading:
#PBS -l select=1:ncpus=64:mem=1gb:scratch_local=1gb:cluster=halmir
#
#PBS -l walltime=00:15:00
#
#   Name        CPU's                           Queue                           Threads                     Rust CPU family         Clock
#
#   samson      4x Intel Xeon Platinum 8280     cerit-pbs.cerit-sc.cz           4x56 threads (224)          cascadelake             2.70 - 4.00 GHz
#   eltu        4x Intel Xeon Platinum 8260     elixir-pbs.elixir-czech.cz      4x48 threads (192)          cascadelake             2.40 - 3.90 GHz
#   elwe        2x AMD EPYC 7532                elixir-pbs.elixir-czech.cz      2x64 threads (128)          znver2                  2.40 - 3.30 GHz
#   kirke       2x AMD EPYC 7532                meta-pbs.metacentrum.cz         dtto
#   TODO        they seem to have the same number of processors, which is..?
#   halmir      1x AMD EPYC 7543                meta-pbs.metacentrum.cz         64 threads                  znver2                  2.80 - 3.70 GHz
#
#PBS -N parmesan-bench_halmir
#PBS -j oe
#PBS -m ae
#PBS -M fakubo@gmail.com

# describtion from 'man qsub' (also see https://wiki.metacentrum.cz/wiki/About_scheduling_system):
# -N ... declares a name for the job. The name specified may be up to and including 15 characters in length. It
#        must consist of printable, non white space characters with the first character alphabetic.
# -q ... defines the destination of the job (queue)
# -l ... defines the resources that are required by the job
# -j oe ... standard error stream of the job will be merged with the standard output stream
# -m ae ...  mail is sent when the job aborts or terminates
# job array: $ qsub -J 2-7:2 script.sh


# ------------------------------------------------------------------------------
#
#   Setup Variables
#

# declare which binary is to be executed:
# ALL_LOG .. compiled with "log_ops" feature (measurement & log at every call of measure_duration! -- which is at every operation)
BINARY_LOG="bench-parmesan_ALL_LOG_znver2-AMD"
# ALL_BEN .. compiled without any measurement feature (measurement & log only at simple_duration! -- which is not inside the lib)
BINARY_BEN="bench-parmesan_ALL_BEN_znver2-AMD"
    # for Halmir, Kirke, Elwe and other AMD-based:
    #   bench-parmesan_ALL_BEN_znver2-AMD
    #   bench-parmesan_PBS_znver2-AMD
    #   bench-parmesan_ADD_znver2-AMD
    #   bench-parmesan_SGN_znver2-AMD
    #   bench-parmesan_MAX_znver2-AMD
    #   bench-parmesan_MUL_znver2-AMD
    #   bench-parmesan_SCM_znver2-AMD
    #   bench-parmesan_NN_znver2-AMD
        # for Samson, Eltu and other Intel-based:
        #   bench-parmesan_ALL_BEN_cascadelake-XEON
        #   bench-parmesan_PBS_cascadelake-XEON
        #   bench-parmesan_ADD_cascadelake-XEON
        #   bench-parmesan_SGN_cascadelake-XEON
        #   bench-parmesan_MAX_cascadelake-XEON
        #   bench-parmesan_MUL_cascadelake-XEON
        #   bench-parmesan_SCM_cascadelake-XEON
        #   bench-parmesan_NN_cascadelake-XEON

CLUSTER_NAME="halmir"   # elwe   samson   eltu   halmir

MEASURE_METHOD="dstat"   # dstat   top

MEASURE_SCRIPT="measure-$MEASURE_METHOD.sh"
CPU_STATS_LOG="raw-cpu-stats-$MEASURE_METHOD.log"

# ------------------------------------------------------------------------------


# initialize required modules (if any)
module add fftw/fftw-3.3.8-intel-19.0.4-532p634
# module add fftw/fftw-3.3.8-intel-20.0.0-au2vxr2   # does not compile with this one

# clean the SCRATCH when job finishes (and data are successfully copied out) or is killed
trap 'clean_scratch' TERM EXIT

# go to the right place
test -n "$SCRATCHDIR" || { echo >&2 "Variable SCRATCHDIR is not set!"; exit 1; }
cd $SCRATCHDIR

# copy files: keys, pre-compiled binary, measurement scripts
DATA_DIR="/storage/brno2/home/fakub/parallel-arithmetics-benchmark"
rm -rf keys
mkdir -p keys
cp \
    $DATA_DIR/keys/SK__n-473_N-1024_gamma-19_l-1_kappa-3_t-5.key \
    $DATA_DIR/keys/BK__n-473_N-1024_gamma-19_l-1_kappa-3_t-5.key \
    $DATA_DIR/keys/KSK__n-473_N-1024_gamma-19_l-1_kappa-3_t-5.key \
    keys/ || { echo >&2 "Error while copying input file(s)!"; exit 2; }
cp \
    $DATA_DIR/bin/$BINARY_LOG \
    $DATA_DIR/bin/$BINARY_BEN \
    $DATA_DIR/dstat-with-short-intervals/dstat \
    $DATA_DIR/dstat-with-short-intervals/measure-dstat.sh \
    $DATA_DIR/dstat-with-short-intervals/measure-top.sh \
    . || { echo >&2 "Error while copying input file(s)!"; exit 2; }

cp -r \
    $DATA_DIR/dstat-with-short-intervals/plugins \
    . || { echo >&2 "Error while copying input folder(s)!"; exit 3; }

# add exec rights
chmod a+x $MEASURE_SCRIPT

# --------------------------------------
# run main command(s):

# processor load measurements (CPU log goes to raw-cpu-stats-dstat.log, besides operations.log)
echo -e "\n>>> Running main command: CPU load & detailed measurements\n"
./$MEASURE_SCRIPT ./$BINARY_LOG
mv operations.log operations-$MEASURE_METHOD.log

# benchmark without extra measurements (log goes to operations.log)
echo -e "\n>>> Running main command: benchmarking maximum performance\n"
./$BINARY_BEN || { echo >&2 "Calculation ended up erroneously (with a code $?) !!"; exit 5; }
mv operations.log operations-bench.log
# --------------------------------------

# copy output log files
ts=$(date +"%y-%m-%d_%H-%M")
logpath=$DATA_DIR/logs/$CLUSTER_NAME/$ts
mkdir -p $logpath


cp \
    $CPU_STATS_LOG \
    operations-$MEASURE_METHOD.log \
    operations-bench.log \
    $logpath || { echo >&2 "Error while copying result file(s)!"; exit 6; }
    #~ $DATA_DIR || { export CLEAN_SCRATCH=false; echo >&2 "Error while copying result file(s)! Try to copy them manually."; exit 6; }
