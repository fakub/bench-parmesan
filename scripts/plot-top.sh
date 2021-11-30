#!/usr/bin/gnuplot

reset

set colorsequence default

set style fill solid noborder

set term pngcairo dashed size 8000,500
set out 'plot-top.png'

set boxwidth 0.02

set yrange [-5:420]
set xrange [83:153]

set xtics 0,1,500 nomirror
set x2tics rotate by 45 left

set ytics 0,100,400

set grid x2tics ytics dt 3 lw 1 lt -1

set xlabel 'Time [s]'

set tmargin at screen 0.15
set bmargin at screen 0.75

#~ set style data histogram
set style histogram rowstacked
# w boxes

plot \
    'cpu-load-top.log' u ($1-0.03):2 w boxes t 'CPU 1 load (top)', \
                    '' u ($1-0.01):3 w boxes t 'CPU 2 load (top)', \
                    '' u ($1+0.01):4 w boxes t 'CPU 3 load (top)', \
                    '' u ($1+0.03):5 w boxes t 'CPU 4 load (top)', \
    'operations.log' u 1:(-100.0):x2tic(2) notitle
    # hack: -100.0 puts the value far away, how to make it invisible?

#~ plot \
    #~ 'cpu-load-top.log' u 2:($11/4) w boxes t 'CPU load (top)', \
    #~ 100 w l dt 2 lw 1 lt -1 t '', \
    #~ 'operations.log' u 1:(-100.0):x2tic(2) notitle
