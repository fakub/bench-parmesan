#!/usr/bin/gnuplot

reset

set colorsequence default

set style fill solid noborder

set term pngcairo dashed size 8000,600
set out 'plot-dstat.png'

#~ set multiplot layout 2,1

set boxwidth 0.08

set yrange [-5:150]
set xrange [26.0:130]

set ytics 0,25,125

# combine files: https://stackoverflow.com/questions/11549004/gnuplot-plot-data-from-two-files-in-one-x-coordinate-in-other-y
# custom xtic: https://stackoverflow.com/questions/50340829/gnuplot-using-xtic-from-data-but-plotting-only-every-10th

# skip first two lines
cpu_stats = "\"< paste raw-cpu-stats-dstat.log operations.log | awk '(NR>2){print;}'\""

# ------------------------------------------------------------------------------

#~ set tmargin at screen 0.48
#~ set bmargin at screen 0.88

unset xtics
set x2tics rotate by 45 left

set grid x2tics ytics dt 3 lw 1 lt -1

#~ plot \
    #~ @cpu_stats u ($130-.0375):45 w boxes t 'CPU 33 load', \
    #~ @cpu_stats u ($130-.0125):46 w boxes t 'CPU 34 load', \
    #~ @cpu_stats u ($130+.0125):51 w boxes t 'CPU 47 load', \
    #~ @cpu_stats u ($130+.0375):52 w boxes t 'CPU 48 load', \
    #~ @cpu_stats u 130:175 w l dt 3 lw 2 t 'CPU 33 freq.', \
    #~ @cpu_stats u 130:176 w l dt 3 lw 2 t 'CPU 34 freq.', \
    #~ @cpu_stats u 130:189 w l dt 3 lw 2 t 'CPU 47 freq.', \
    #~ @cpu_stats u 130:190 w l dt 3 lw 2 t 'CPU 48 freq.', \
    #~ @cpu_stats u 259:(-100.0):x2tic(260) notitle

# ------------------------------------------------------------------------------

set tmargin at screen 0.15
set bmargin at screen 0.75

set xtics 26,1,130 nomirror
# keep x2tics

set xlabel 'Time [s]'

plot \
    @cpu_stats u 130:(($0+$1+$2+$3+$4+$5+$6+$7+$8+$9+$10+$11+$12+$13+$14+$15+$16+$17+$18+$19+$20+$21+$22+$23+$24+$25+$26+$27+$28+$29+$30+$31+$32+$33+$34+$35+$36+$37+$38+$39+$40+$41+$42+$43+$44+$45+$46+$47+$48+$49+$50+$51+$52+$53+$54+$55+$56+$57+$58+$59+$60+$61+$62+$63+$64+$65+$66+$67+$68+$69+$70+$71+$72+$73+$74+$75+$76+$77+$78+$79+$80+$81+$82+$83+$84+$85+$86+$87+$88+$89+$90+$91+$92+$93+$94+$95+$96+$97+$98+$99+$100+$101+$102+$103+$104+$105+$106+$107+$108+$109+$110+$111+$112+$113+$114+$115+$116+$117+$118+$119+$120+$121+$122+$123+$124+$125+$126+$127) / 128) w boxes t 'CPU load (dstat)', \
    @cpu_stats u 130:(($131+$132+$133+$134+$135+$136+$137+$138+$139+$140+$141+$142+$143+$144+$145+$146+$147+$148+$149+$150+$151+$152+$153+$154+$155+$156+$157+$158+$159+$160+$161+$162+$163+$164+$165+$166+$167+$168+$169+$170+$171+$172+$173+$174+$175+$176+$177+$178+$179+$180+$181+$182+$183+$184+$185+$186+$187+$188+$189+$190+$191+$192+$193+$194+$195+$196+$197+$198+$199+$200+$201+$202+$203+$204+$205+$206+$207+$208+$209+$210+$211+$212+$213+$214+$215+$216+$217+$218+$219+$220+$221+$222+$223+$224+$225+$226+$227+$228+$229+$230+$231+$232+$233+$234+$235+$236+$237+$238+$239+$240+$241+$242+$243+$244+$245+$246+$247+$248+$249+$250+$251+$252+$253+$254+$255+$256+$257+$258) / 128) w l dt 3 lw 2 t 'Avg. CPU freq.', \
    100 w l dt 2 lw 1 lt -1 t '', \
    @cpu_stats u 259:(-100.0):x2tic(260) notitle
    # hack: -100.0 puts the value far away, how to make it invisible?

    #~ @cpu_stats u (column(0)-.375):($1) w boxes t 'CPU 0 load', \

# ------------------------------------------------------------------------------

#~ unset multiplot
