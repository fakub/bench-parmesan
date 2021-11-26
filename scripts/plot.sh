#!/usr/bin/gnuplot

reset

set term pngcairo dashed size 8000,1100
set out 'plot.png'

set colorsequence default

set style fill solid noborder
set boxwidth 0.025

set multiplot layout 2,1

set yrange [-5:110]
set xrange [7.0:76.5]

set ytics 0,25,100

# combine files: https://stackoverflow.com/questions/11549004/gnuplot-plot-data-from-two-files-in-one-x-coordinate-in-other-y
# custom xtic: https://stackoverflow.com/questions/50340829/gnuplot-using-xtic-from-data-but-plotting-only-every-10th

# skip first two lines
cpu_stats = "\"< paste cpu-stats.log operations.log | awk '(NR>2){print;}'\""

# ------------------------------------------------------------------------------

set tmargin at screen 0.48
set bmargin at screen 0.88

unset xtics
set x2tics rotate by 45 left

set grid x2tics ytics dt 3 lw 1 lt -1

plot \
    @cpu_stats u ($6-.0375):1 w boxes t 'CPU 0 load', \
    @cpu_stats u ($6-.0125):2 w boxes t 'CPU 1 load', \
    @cpu_stats u ($6+.0125):3 w boxes t 'CPU 2 load', \
    @cpu_stats u ($6+.0375):4 w boxes t 'CPU 3 load', \
    @cpu_stats u 6: 7 w l dt 3 lw 2 t 'CPU 0 freq.', \
    @cpu_stats u 6: 8 w l dt 3 lw 2 t 'CPU 1 freq.', \
    @cpu_stats u 6: 9 w l dt 3 lw 2 t 'CPU 2 freq.', \
    @cpu_stats u 6:10 w l dt 3 lw 2 t 'CPU 3 freq.', \
    @cpu_stats u 11:(-100.0):x2tic(12) notitle
    # hack: -100.0 puts the value far away, how to make it invisible?

# ------------------------------------------------------------------------------

set tmargin at screen 0.08
set bmargin at screen 0.48

set xtics 0,1,75 nomirror
# keep x2tics

set xlabel 'Time [s]'

plot \
    @cpu_stats u 6:(($1+$2+$3+$4 )/4) w boxes t 'Total CPU load', \
    @cpu_stats u 6:(($7+$8+$9+$10)/4) w l dt 3 lw 2 t 'Avg. CPU freq.', \
    @cpu_stats u 11:(-100.0):x2tic("") notitle

    #~ @cpu_stats u (column(0)-.375):($1) w boxes t 'CPU 0 load', \

# ------------------------------------------------------------------------------

unset multiplot
