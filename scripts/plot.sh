#!/usr/bin/gnuplot

reset

set term pngcairo dashed size 2400,1100
set out 'plot.png'

set colorsequence default

set multiplot layout 2,1

set x2tics ('$2^{-13}$' 0.00012207, '$2^{-11}$' 0.000488281, '$2^{-9}$' 0.001953125, '$2^{-7}$' 0.0078125, '$2^{-5}$' 0.03125, '$2^{-3}$' 0.125, '$\nicefrac{1}{2}$' 0.5, '1' 1.0)

set yrange [-5:110]
set xrange [3.7:15.0]

set style fill solid noborder
set boxwidth 0.025

# skip first two lines
cpu_stats = "\"< awk '(NR>2){print;}' cpu-stats.log\""

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

set ytics 0,25,100

plot \
    @cpu_stats u 6:(($1+$2+$3+$4 )/4) w boxes t 'Total CPU load', \
    @cpu_stats u 6:(($7+$8+$9+$10)/4) w l dt 3 lw 2 t 'Avg. CPU freq.', \
    0   w l dt 3 lw 1 lt -1 t '', \
    25  w l dt 3 lw 1 lt -1 t '', \
    50  w l dt 3 lw 1 lt -1 t '', \
    75  w l dt 3 lw 1 lt -1 t '', \
    100 w l dt 3 lw 1 lt -1 t ''

    # @cpu_stats u (column(0)-.375):($1) w boxes t 'CPU 0 load', \
