
#   Benchmarking Parmesan

... implements a bunch of scripts for experimental evaluation of Parmesan's arithmetic operations.


## Select Operations

There are currently the following Rust features, that allow to select what operations will be executed:

  * "pbs": runs programmable bootstrapping `PBS_N`-times,
  * "add": addition,
  * "sgn": signum,
  * "round": rounding,
  * "max": maximum,
  * "mul": multiplication (goes with a light-weight variant "mul_light" for 4- and 8-bit multiplication),
  * "squ": squaring (light-weight variant "squ_light"),
  * "scm": scalar multiplication,
  * "nn": neural network evaluation.


## Setup Benchmark Log Verbosity

There are two levels of measurements & logging that can be set up by the following Rust features:

  * "measure": each `measure_duration!` macro within Parmesan's code measures the timing of respective block (including nested occurences),
  * "log_ops": logs the measured timing into the `operations.log` file.

### For Analysis

Compile with the "log_ops" feature: most of the nested operations will be measured & logged into the `operations.log` file.

### For Best Performance

Compile without any feature: just overall operations' timing will be written into the `operations.log` file (everything called by the `simple_duration!` macro), without affecting any nested call.


## Running on a Cluster

Scripts for the management of benchmarks on a cluster are provided in the root.
These scripts are prepared for PBS (Portable Batch System) queuing system, however, they need to be customized.

All results are written into a `YY-MM-DD_hh-mm` folder on the cluster, with the file `operations.log` renamed to either `operations-dstat.log`, or `operations-bench.log` (both can be present).


## Plotting Results

After copying the log folder from the cluster to a local folder, copy here also the data processing & plotting scripts from the `scripts` folder: `preproc-cpu-stats.rb` etc.
The pre-processing script extracts lots of data and writes into respective log files.
Then edit the x-range [s] in the `plot-dstat.sh` script according to the range in the `operations-dstat.log` file: 2nd & 3rd column stands for mm:ss.
E.g., for `operations-dstat.log`
```
0   49 54.706   49 56.679   "Load PrivKeySet"
...
0   50 21.303   50 21.303   "Neural Network evaluation over i64"
```
the x-range in `plot-dstat.sh` shall be set (possibly with some extra room) as
```bash
set xrange [2990:3030]
```
Running `plot-dstat.sh` then creates an overview of operations and respective processor load in time, with major operations highlighted.

FIXME: if `operations-dstat.log` is longer than `raw-cpu-stats-dstat.log`, merging them in `plot-dstat.sh` makes shit. Can be fixed manually by appending zero-filled lines to `cpu-load-ord-dstat.log`.


## Dev Questions

* Is the fork/branch of `dstat` the ideal way to log the processor load?
* How to set the highest CPU clock? Btw does this make sense wrt CPU temperature?
    * `-l select=cpu_flag=<some-turbo-flag>` ?
    * maybe [only one core](https://en.wikichip.org/wiki/intel/xeon_platinum/8280)?
* Advanced: 2 or 3 [UPI links](https://en.wikichip.org/wiki/intel/microarchitectures/cascade_lake) in configurations with Intel Xeon?
* C.f. other settings on [wiki](https://wiki.metacentrum.cz/wiki/About_scheduling_system).


## License

Parmesan is licensed under AGPLv3.
