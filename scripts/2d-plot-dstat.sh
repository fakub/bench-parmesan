#!/usr/bin/gnuplot

reset

set term pngcairo dashed size 2500,1100
set out '2d-plot-dstat.png'

unset key

set xrange [-10:1050]
set yrange [-5:133]                     # [-2:6]

set multiplot layout 2,1

# ------------------------------------------------------------------------------

set xlabel 'Time [s]'

set tmargin at screen 0.08
set bmargin at screen 0.53

plot 'cpu-load-dstat.log' matrix with image notitle   # every ::3::5

# ------------------------------------------------------------------------------

unset xtics
unset xlabel

set tmargin at screen 0.53
set bmargin at screen 0.98

plot 'cpu-freq-dstat.log' matrix with image notitle   # every ::3::5

# ------------------------------------------------------------------------------

unset multiplot
