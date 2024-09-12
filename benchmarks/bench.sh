# show commands
set -x

# compile
clang -O2 -g -fno-omit-frame-pointer -o main.out clox-c/*.c

# dtrace the program
sudo dtrace  -x ustackframes=100 -x stackframes=100\
  -c './main.out benchmarks/zoo.lox'\
  -o out.stacks -n '
profile-4999
/execname == "main.out"/
{
  @[ustack()] = count();
}'

# generate the flamegraph 
./benchmarks/stackcollapse.pl benchmarks/out.stacks | ./benchmarks/flamegraph.pl --colors=java --title="benchmarks/zoo" --countname="samples" > benchmarks/profile.svg

open benchmarks/profile.svg
