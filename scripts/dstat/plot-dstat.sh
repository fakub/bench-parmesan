#!/usr/bin/gnuplot

reset

set colorsequence default

set style fill solid noborder

set term pngcairo dashed size 8000,2400
set out 'plot-dstat.png'

set multiplot layout 4,1

set boxwidth 0.08

set yrange [-0.2:32.5]
set xrange [26.0:130]

set ytics 0,1,32
set x2tics rotate by 45 left

# combine files: https://stackoverflow.com/questions/11549004/gnuplot-plot-data-from-two-files-in-one-x-coordinate-in-other-y
# custom xtic: https://stackoverflow.com/questions/50340829/gnuplot-using-xtic-from-data-but-plotting-only-every-10th

# skip first two lines
cpu_stats = "\"< paste raw-cpu-stats-dstat.log operations.log | awk '(NR>2){print;}'\""

set grid x2tics ytics dt 3 lw 1 lt -1

# ------------------------------------------------------------------------------

# 4.times{|j| 32.times{|i| puts "    @cpu_stats u 130:((#{'$' + (j*32..(j+1)*32-i-1).to_a.join('+$')})/ 100) w boxes notitle, \\" } }

set tmargin at screen 0.68
set bmargin at screen 0.88

plot \
    @cpu_stats u 130:(($97+$98+$99+$100+$101+$102+$103+$104+$105+$106+$107+$108+$109+$110+$111+$112+$113+$114+$115+$116+$117+$118+$119+$120+$121+$122+$123+$124+$125+$126+$127+$128)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($97+$98+$99+$100+$101+$102+$103+$104+$105+$106+$107+$108+$109+$110+$111+$112+$113+$114+$115+$116+$117+$118+$119+$120+$121+$122+$123+$124+$125+$126+$127)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($97+$98+$99+$100+$101+$102+$103+$104+$105+$106+$107+$108+$109+$110+$111+$112+$113+$114+$115+$116+$117+$118+$119+$120+$121+$122+$123+$124+$125+$126)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($97+$98+$99+$100+$101+$102+$103+$104+$105+$106+$107+$108+$109+$110+$111+$112+$113+$114+$115+$116+$117+$118+$119+$120+$121+$122+$123+$124+$125)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($97+$98+$99+$100+$101+$102+$103+$104+$105+$106+$107+$108+$109+$110+$111+$112+$113+$114+$115+$116+$117+$118+$119+$120+$121+$122+$123+$124)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($97+$98+$99+$100+$101+$102+$103+$104+$105+$106+$107+$108+$109+$110+$111+$112+$113+$114+$115+$116+$117+$118+$119+$120+$121+$122+$123)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($97+$98+$99+$100+$101+$102+$103+$104+$105+$106+$107+$108+$109+$110+$111+$112+$113+$114+$115+$116+$117+$118+$119+$120+$121+$122)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($97+$98+$99+$100+$101+$102+$103+$104+$105+$106+$107+$108+$109+$110+$111+$112+$113+$114+$115+$116+$117+$118+$119+$120+$121)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($97+$98+$99+$100+$101+$102+$103+$104+$105+$106+$107+$108+$109+$110+$111+$112+$113+$114+$115+$116+$117+$118+$119+$120)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($97+$98+$99+$100+$101+$102+$103+$104+$105+$106+$107+$108+$109+$110+$111+$112+$113+$114+$115+$116+$117+$118+$119)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($97+$98+$99+$100+$101+$102+$103+$104+$105+$106+$107+$108+$109+$110+$111+$112+$113+$114+$115+$116+$117+$118)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($97+$98+$99+$100+$101+$102+$103+$104+$105+$106+$107+$108+$109+$110+$111+$112+$113+$114+$115+$116+$117)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($97+$98+$99+$100+$101+$102+$103+$104+$105+$106+$107+$108+$109+$110+$111+$112+$113+$114+$115+$116)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($97+$98+$99+$100+$101+$102+$103+$104+$105+$106+$107+$108+$109+$110+$111+$112+$113+$114+$115)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($97+$98+$99+$100+$101+$102+$103+$104+$105+$106+$107+$108+$109+$110+$111+$112+$113+$114)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($97+$98+$99+$100+$101+$102+$103+$104+$105+$106+$107+$108+$109+$110+$111+$112+$113)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($97+$98+$99+$100+$101+$102+$103+$104+$105+$106+$107+$108+$109+$110+$111+$112)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($97+$98+$99+$100+$101+$102+$103+$104+$105+$106+$107+$108+$109+$110+$111)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($97+$98+$99+$100+$101+$102+$103+$104+$105+$106+$107+$108+$109+$110)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($97+$98+$99+$100+$101+$102+$103+$104+$105+$106+$107+$108+$109)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($97+$98+$99+$100+$101+$102+$103+$104+$105+$106+$107+$108)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($97+$98+$99+$100+$101+$102+$103+$104+$105+$106+$107)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($97+$98+$99+$100+$101+$102+$103+$104+$105+$106)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($97+$98+$99+$100+$101+$102+$103+$104+$105)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($97+$98+$99+$100+$101+$102+$103+$104)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($97+$98+$99+$100+$101+$102+$103)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($97+$98+$99+$100+$101+$102)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($97+$98+$99+$100+$101)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($97+$98+$99+$100)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($97+$98+$99)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($97+$98)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($97)/ 100) w boxes notitle, \
    @cpu_stats u 130:(-100):xtic("") notitle, \
    32 w l dt 2 lw 1 lt -1 t '', \
    @cpu_stats u 259:(-100.0):x2tic(260) notitle
    # hack: -100.0 puts the value far away, how to make it invisible?

# ------------------------------------------------------------------------------

set tmargin at screen 0.48
set bmargin at screen 0.68

plot \
    @cpu_stats u 130:(($65+$66+$67+$68+$69+$70+$71+$72+$73+$74+$75+$76+$77+$78+$79+$80+$81+$82+$83+$84+$85+$86+$87+$88+$89+$90+$91+$92+$93+$94+$95+$96)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($65+$66+$67+$68+$69+$70+$71+$72+$73+$74+$75+$76+$77+$78+$79+$80+$81+$82+$83+$84+$85+$86+$87+$88+$89+$90+$91+$92+$93+$94+$95)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($65+$66+$67+$68+$69+$70+$71+$72+$73+$74+$75+$76+$77+$78+$79+$80+$81+$82+$83+$84+$85+$86+$87+$88+$89+$90+$91+$92+$93+$94)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($65+$66+$67+$68+$69+$70+$71+$72+$73+$74+$75+$76+$77+$78+$79+$80+$81+$82+$83+$84+$85+$86+$87+$88+$89+$90+$91+$92+$93)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($65+$66+$67+$68+$69+$70+$71+$72+$73+$74+$75+$76+$77+$78+$79+$80+$81+$82+$83+$84+$85+$86+$87+$88+$89+$90+$91+$92)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($65+$66+$67+$68+$69+$70+$71+$72+$73+$74+$75+$76+$77+$78+$79+$80+$81+$82+$83+$84+$85+$86+$87+$88+$89+$90+$91)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($65+$66+$67+$68+$69+$70+$71+$72+$73+$74+$75+$76+$77+$78+$79+$80+$81+$82+$83+$84+$85+$86+$87+$88+$89+$90)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($65+$66+$67+$68+$69+$70+$71+$72+$73+$74+$75+$76+$77+$78+$79+$80+$81+$82+$83+$84+$85+$86+$87+$88+$89)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($65+$66+$67+$68+$69+$70+$71+$72+$73+$74+$75+$76+$77+$78+$79+$80+$81+$82+$83+$84+$85+$86+$87+$88)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($65+$66+$67+$68+$69+$70+$71+$72+$73+$74+$75+$76+$77+$78+$79+$80+$81+$82+$83+$84+$85+$86+$87)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($65+$66+$67+$68+$69+$70+$71+$72+$73+$74+$75+$76+$77+$78+$79+$80+$81+$82+$83+$84+$85+$86)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($65+$66+$67+$68+$69+$70+$71+$72+$73+$74+$75+$76+$77+$78+$79+$80+$81+$82+$83+$84+$85)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($65+$66+$67+$68+$69+$70+$71+$72+$73+$74+$75+$76+$77+$78+$79+$80+$81+$82+$83+$84)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($65+$66+$67+$68+$69+$70+$71+$72+$73+$74+$75+$76+$77+$78+$79+$80+$81+$82+$83)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($65+$66+$67+$68+$69+$70+$71+$72+$73+$74+$75+$76+$77+$78+$79+$80+$81+$82)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($65+$66+$67+$68+$69+$70+$71+$72+$73+$74+$75+$76+$77+$78+$79+$80+$81)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($65+$66+$67+$68+$69+$70+$71+$72+$73+$74+$75+$76+$77+$78+$79+$80)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($65+$66+$67+$68+$69+$70+$71+$72+$73+$74+$75+$76+$77+$78+$79)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($65+$66+$67+$68+$69+$70+$71+$72+$73+$74+$75+$76+$77+$78)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($65+$66+$67+$68+$69+$70+$71+$72+$73+$74+$75+$76+$77)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($65+$66+$67+$68+$69+$70+$71+$72+$73+$74+$75+$76)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($65+$66+$67+$68+$69+$70+$71+$72+$73+$74+$75)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($65+$66+$67+$68+$69+$70+$71+$72+$73+$74)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($65+$66+$67+$68+$69+$70+$71+$72+$73)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($65+$66+$67+$68+$69+$70+$71+$72)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($65+$66+$67+$68+$69+$70+$71)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($65+$66+$67+$68+$69+$70)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($65+$66+$67+$68+$69)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($65+$66+$67+$68)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($65+$66+$67)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($65+$66)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($65)/ 100) w boxes notitle, \
    @cpu_stats u 130:(-100):xtic("") notitle, \
    32 w l dt 2 lw 1 lt -1 t '', \
    @cpu_stats u 259:(-100.0):x2tic("") notitle

# ------------------------------------------------------------------------------

set tmargin at screen 0.28
set bmargin at screen 0.48

plot \
    @cpu_stats u 130:(($33+$34+$35+$36+$37+$38+$39+$40+$41+$42+$43+$44+$45+$46+$47+$48+$49+$50+$51+$52+$53+$54+$55+$56+$57+$58+$59+$60+$61+$62+$63+$64)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($33+$34+$35+$36+$37+$38+$39+$40+$41+$42+$43+$44+$45+$46+$47+$48+$49+$50+$51+$52+$53+$54+$55+$56+$57+$58+$59+$60+$61+$62+$63)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($33+$34+$35+$36+$37+$38+$39+$40+$41+$42+$43+$44+$45+$46+$47+$48+$49+$50+$51+$52+$53+$54+$55+$56+$57+$58+$59+$60+$61+$62)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($33+$34+$35+$36+$37+$38+$39+$40+$41+$42+$43+$44+$45+$46+$47+$48+$49+$50+$51+$52+$53+$54+$55+$56+$57+$58+$59+$60+$61)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($33+$34+$35+$36+$37+$38+$39+$40+$41+$42+$43+$44+$45+$46+$47+$48+$49+$50+$51+$52+$53+$54+$55+$56+$57+$58+$59+$60)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($33+$34+$35+$36+$37+$38+$39+$40+$41+$42+$43+$44+$45+$46+$47+$48+$49+$50+$51+$52+$53+$54+$55+$56+$57+$58+$59)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($33+$34+$35+$36+$37+$38+$39+$40+$41+$42+$43+$44+$45+$46+$47+$48+$49+$50+$51+$52+$53+$54+$55+$56+$57+$58)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($33+$34+$35+$36+$37+$38+$39+$40+$41+$42+$43+$44+$45+$46+$47+$48+$49+$50+$51+$52+$53+$54+$55+$56+$57)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($33+$34+$35+$36+$37+$38+$39+$40+$41+$42+$43+$44+$45+$46+$47+$48+$49+$50+$51+$52+$53+$54+$55+$56)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($33+$34+$35+$36+$37+$38+$39+$40+$41+$42+$43+$44+$45+$46+$47+$48+$49+$50+$51+$52+$53+$54+$55)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($33+$34+$35+$36+$37+$38+$39+$40+$41+$42+$43+$44+$45+$46+$47+$48+$49+$50+$51+$52+$53+$54)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($33+$34+$35+$36+$37+$38+$39+$40+$41+$42+$43+$44+$45+$46+$47+$48+$49+$50+$51+$52+$53)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($33+$34+$35+$36+$37+$38+$39+$40+$41+$42+$43+$44+$45+$46+$47+$48+$49+$50+$51+$52)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($33+$34+$35+$36+$37+$38+$39+$40+$41+$42+$43+$44+$45+$46+$47+$48+$49+$50+$51)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($33+$34+$35+$36+$37+$38+$39+$40+$41+$42+$43+$44+$45+$46+$47+$48+$49+$50)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($33+$34+$35+$36+$37+$38+$39+$40+$41+$42+$43+$44+$45+$46+$47+$48+$49)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($33+$34+$35+$36+$37+$38+$39+$40+$41+$42+$43+$44+$45+$46+$47+$48)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($33+$34+$35+$36+$37+$38+$39+$40+$41+$42+$43+$44+$45+$46+$47)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($33+$34+$35+$36+$37+$38+$39+$40+$41+$42+$43+$44+$45+$46)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($33+$34+$35+$36+$37+$38+$39+$40+$41+$42+$43+$44+$45)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($33+$34+$35+$36+$37+$38+$39+$40+$41+$42+$43+$44)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($33+$34+$35+$36+$37+$38+$39+$40+$41+$42+$43)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($33+$34+$35+$36+$37+$38+$39+$40+$41+$42)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($33+$34+$35+$36+$37+$38+$39+$40+$41)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($33+$34+$35+$36+$37+$38+$39+$40)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($33+$34+$35+$36+$37+$38+$39)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($33+$34+$35+$36+$37+$38)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($33+$34+$35+$36+$37)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($33+$34+$35+$36)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($33+$34+$35)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($33+$34)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($33)/ 100) w boxes notitle, \
    @cpu_stats u 130:(-100):xtic("") notitle, \
    32 w l dt 2 lw 1 lt -1 t '', \
    @cpu_stats u 259:(-100.0):x2tic("") notitle

# ------------------------------------------------------------------------------

set tmargin at screen 0.08
set bmargin at screen 0.28

set xlabel 'Time [s]'

plot \
    @cpu_stats u 130:(($1+$2+$3+$4+$5+$6+$7+$8+$9+$10+$11+$12+$13+$14+$15+$16+$17+$18+$19+$20+$21+$22+$23+$24+$25+$26+$27+$28+$29+$30+$31+$32)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($1+$2+$3+$4+$5+$6+$7+$8+$9+$10+$11+$12+$13+$14+$15+$16+$17+$18+$19+$20+$21+$22+$23+$24+$25+$26+$27+$28+$29+$30+$31)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($1+$2+$3+$4+$5+$6+$7+$8+$9+$10+$11+$12+$13+$14+$15+$16+$17+$18+$19+$20+$21+$22+$23+$24+$25+$26+$27+$28+$29+$30)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($1+$2+$3+$4+$5+$6+$7+$8+$9+$10+$11+$12+$13+$14+$15+$16+$17+$18+$19+$20+$21+$22+$23+$24+$25+$26+$27+$28+$29)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($1+$2+$3+$4+$5+$6+$7+$8+$9+$10+$11+$12+$13+$14+$15+$16+$17+$18+$19+$20+$21+$22+$23+$24+$25+$26+$27+$28)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($1+$2+$3+$4+$5+$6+$7+$8+$9+$10+$11+$12+$13+$14+$15+$16+$17+$18+$19+$20+$21+$22+$23+$24+$25+$26+$27)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($1+$2+$3+$4+$5+$6+$7+$8+$9+$10+$11+$12+$13+$14+$15+$16+$17+$18+$19+$20+$21+$22+$23+$24+$25+$26)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($1+$2+$3+$4+$5+$6+$7+$8+$9+$10+$11+$12+$13+$14+$15+$16+$17+$18+$19+$20+$21+$22+$23+$24+$25)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($1+$2+$3+$4+$5+$6+$7+$8+$9+$10+$11+$12+$13+$14+$15+$16+$17+$18+$19+$20+$21+$22+$23+$24)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($1+$2+$3+$4+$5+$6+$7+$8+$9+$10+$11+$12+$13+$14+$15+$16+$17+$18+$19+$20+$21+$22+$23)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($1+$2+$3+$4+$5+$6+$7+$8+$9+$10+$11+$12+$13+$14+$15+$16+$17+$18+$19+$20+$21+$22)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($1+$2+$3+$4+$5+$6+$7+$8+$9+$10+$11+$12+$13+$14+$15+$16+$17+$18+$19+$20+$21)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($1+$2+$3+$4+$5+$6+$7+$8+$9+$10+$11+$12+$13+$14+$15+$16+$17+$18+$19+$20)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($1+$2+$3+$4+$5+$6+$7+$8+$9+$10+$11+$12+$13+$14+$15+$16+$17+$18+$19)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($1+$2+$3+$4+$5+$6+$7+$8+$9+$10+$11+$12+$13+$14+$15+$16+$17+$18)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($1+$2+$3+$4+$5+$6+$7+$8+$9+$10+$11+$12+$13+$14+$15+$16+$17)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($1+$2+$3+$4+$5+$6+$7+$8+$9+$10+$11+$12+$13+$14+$15+$16)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($1+$2+$3+$4+$5+$6+$7+$8+$9+$10+$11+$12+$13+$14+$15)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($1+$2+$3+$4+$5+$6+$7+$8+$9+$10+$11+$12+$13+$14)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($1+$2+$3+$4+$5+$6+$7+$8+$9+$10+$11+$12+$13)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($1+$2+$3+$4+$5+$6+$7+$8+$9+$10+$11+$12)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($1+$2+$3+$4+$5+$6+$7+$8+$9+$10+$11)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($1+$2+$3+$4+$5+$6+$7+$8+$9+$10)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($1+$2+$3+$4+$5+$6+$7+$8+$9)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($1+$2+$3+$4+$5+$6+$7+$8)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($1+$2+$3+$4+$5+$6+$7)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($1+$2+$3+$4+$5+$6)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($1+$2+$3+$4+$5)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($1+$2+$3+$4)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($1+$2+$3)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($1+$2)/ 100) w boxes notitle, \
    @cpu_stats u 130:(($1)/ 100) w boxes notitle, \
    @cpu_stats u 130:(-100):xtic("") notitle, \
    32 w l dt 2 lw 1 lt -1 t '', \
    @cpu_stats u 259:(-100.0):x2tic("") notitle

# ------------------------------------------------------------------------------

    #~ @cpu_stats u 130:(($131+$132+$133+$134+$135+$136+$137+$138+$139+$140+$141+$142+$143+$144+$145+$146+$147+$148+$149+$150+$151+$152+$153+$154+$155+$156+$157+$158+$159+$160+$161+$162+$163+$164+$165+$166+$167+$168+$169+$170+$171+$172+$173+$174+$175+$176+$177+$178+$179+$180+$181+$182+$183+$184+$185+$186+$187+$188+$189+$190+$191+$192+$193+$194+$195+$196+$197+$198+$199+$200+$201+$202+$203+$204+$205+$206+$207+$208+$209+$210+$211+$212+$213+$214+$215+$216+$217+$218+$219+$220+$221+$222+$223+$224+$225+$226+$227+$228+$229+$230+$231+$232+$233+$234+$235+$236+$237+$238+$239+$240+$241+$242+$243+$244+$245+$246+$247+$248+$249+$250+$251+$252+$253+$254+$255+$256+$257+$258) / 128) w l dt 3 lw 2 t 'Avg. CPU freq.', \

unset multiplot
