# xyborder.cfg
set style line 101 lc rgb '#333333' lt 1 lw 1
set border 3 front ls 101
set tics nomirror out scale 0.75
set format '%g'

# grid style
set style line 102 lc rgb '#cccccc' lt 0 lw 1
set grid back ls 102

# output type and more styling
load "spectral.pal"
set terminal svg \
    size 800,450\
    dynamic \
    enhanced \
    font "Open Sans, 14" \
    mousing \
#    color \
    dashed \
    rounded \
    linewidth 2
set key right top
set output "response.svg"

# axes setup
set title "AC analysis plot"
set xlabel "Frequency (HZ)"
set ylabel "Amplitude (dB)"
set y2label "Phase (rad)"
set grid
set logscale x
#set xrange [1e+01:1e+9]
set mxtics 10
set grid mxtics
unset logscale y
#set yrange[-3.1415:3.1415]
set ytics nomirror
set y2tics
set autoscale y2
plot "vdb_data" using 1:2 with lines title "Magnitude" axes x1y1 linestyle 7,\
"vp_data" using 1:2 with lines title "Phase" axes x1y2 linestyle 6

show output

# curve fitting
# b = -0.13
# a = 0.00001
# f(x) = a*x**2 + b
# fit f(x) "vdb_data" using 1:2 via a,b
# 
# fit_func = sprintf("fit $= %.3e \\times x^2 %+-.3e$",a,b)
# set label 1 at 10000,-0.13951 fit_func


