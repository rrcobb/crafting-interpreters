set -x
# compile w/ profiler
clang -g -fno-omit-frame-pointer -o main.out clox-c/*.c -L/usr/local/Cellar/gperftools/2.15/lib -lprofiler

CPUPROFILE_FREQUENCY=1000 CPUPROFILE=profile.out ./main.out benchmarks/zoo.lox

pprof --text ./main.out profile.out > benchmarks/report.txt 2>/dev/null

echo "report written to benchmarks/report.txt"
