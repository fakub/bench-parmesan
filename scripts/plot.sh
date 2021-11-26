#!/usr/bin/gnuplot

reset

set term pngcairo dashed size 8000,1100
set out 'plot.png'

set colorsequence default

set style fill solid noborder
set boxwidth 0.025

set multiplot layout 2,1

set yrange [-5:110]
set xrange [0.0:75.0]

set xtics 0,1,75 nomirror
set x2tics ( \
    'Load keys' 02.016, \
    'PBS (10x)' 07.367, \
    '1st level add' 08.340, \
    '1st level subtr' 09.178, \
    '2nd level add' 10.005, \
    'sgn 1' 10.905, \
    'sgn 2' 12.116, \
    'max 1' 13.478, \
    'max 2' 17.957, \
    'max 3' 22.202, \
    '4w mul' 26.814, \
    '8w mul' 28.807, \
    '4w squ' 37.066, \
    '8w squ' 38.951, \
    'sc mul 161' 45.884, \
    'sc mul 239' 48.760, \
    'sc mul 231' 51.475, \
    'sc mul 170' 55.369, \
    'sc mul 219' 59.154, \
    'nn eval' 63.065, \
    'END' 71.736, \
) rotate by 45 left

# skip first two lines
cpu_stats = "\"< awk '(NR>2){print;}' cpu-stats.log\""

# ------------------------------------------------------------------------------

plot \
    @cpu_stats u ($6-.0375):1 w boxes t 'CPU 0 load', \
    @cpu_stats u ($6-.0125):2 w boxes t 'CPU 1 load', \
    @cpu_stats u ($6+.0125):3 w boxes t 'CPU 2 load', \
    @cpu_stats u ($6+.0375):4 w boxes t 'CPU 3 load', \
    @cpu_stats u 6: 7 w l dt 3 lw 2 t 'CPU 0 freq.', \
    @cpu_stats u 6: 8 w l dt 3 lw 2 t 'CPU 1 freq.', \
    @cpu_stats u 6: 9 w l dt 3 lw 2 t 'CPU 2 freq.', \
    @cpu_stats u 6:10 w l dt 3 lw 2 t 'CPU 3 freq.', \
    0   w l dt 3 lw 1 lt -1 t '', \
    100 w l dt 3 lw 1 lt -1 t ''

# ------------------------------------------------------------------------------

set ytics 0,25,100

plot \
    @cpu_stats u 6:(($1+$2+$3+$4 )/4) w boxes t 'Total CPU load', \
    @cpu_stats u 6:(($7+$8+$9+$10)/4) w l dt 3 lw 2 t 'Avg. CPU freq.', \
    0   w l dt 3 lw 1 lt -1 t '', \
    25  w l dt 3 lw 1 lt -1 t '', \
    50  w l dt 3 lw 1 lt -1 t '', \
    75  w l dt 3 lw 1 lt -1 t '', \
    100 w l dt 3 lw 1 lt -1 t ''

    #~ @cpu_stats u (column(0)-.375):($1) w boxes t 'CPU 0 load', \
