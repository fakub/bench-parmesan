#!/usr/bin/gnuplot

reset

set term pngcairo dashed size 2500,1100
set out '2d-plot.png'

unset key

set multiplot layout 2,1

set xrange [-10:1050]
set yrange [-5:133]

set xlabel 'Time [s]'

# ------------------------------------------------------------------------------

set tmargin at screen 0.08
set bmargin at screen 0.53

plot 'cpu-load.log' matrix with image notitle   # every ::3::5

# ------------------------------------------------------------------------------

unset xtics

set tmargin at screen 0.53
set bmargin at screen 0.98

plot 'cpu-freq.log' matrix with image notitle   # every ::3::5

# ------------------------------------------------------------------------------

unset multiplot
