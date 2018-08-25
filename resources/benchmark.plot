#!/usr/bin/env gnuplot -c

if (strlen(ARG1) == 0) print "Usage: " . ARG0 . " <repo_name> <repo.csv>"; exit

set terminal svg
set datafile separator comma
set title "Performance on the " . ARG1 . " Repository w/o Cloc (Lower is better)"
unset key
set xlabel 'Program'
set ylabel 'Mean time (milliseconds)'
# Adjust depending on graphs.
set yrange [0:380]
set style fill solid
set style data histogram
set xtics center
plot ARG2 using 2:xtic(1) title columnheader
