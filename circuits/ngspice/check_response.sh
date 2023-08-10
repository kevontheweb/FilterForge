#!/bin/bash

ngspice lpf.cir
gnuplot plot.gp
convert response.svg response.png
