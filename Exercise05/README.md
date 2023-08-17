# Get started

## Dependencies

The [good_lp](https://crates.io/crates/good_lp) crate allows us to utilize the mixed integer linear programming solver, [Coin-or Cbc](https://www.coin-or.org/Cbc/), within Rust.

In order to use it, Coin-or Cbc **must be installed** as outlined in its [README](https://github.com/coin-or/Cbc/blob/master/README.md).\
**note:** a working C compiler is also required for Coin-or Cbc to function properly.

## Running 

### Compiling
before running the code you need to compile it:
```bash
cd 'Path_to_this_folder'
cargo build --release
```

### Full ILP-formulation
run:
```bash
./target/release/Exercise05 'tsp-file' 'k' 0
```
**k** is the number of cities computed by the code.\
you can use -1 for k to compute the TSP for the full Graph.\
you can find the country tsp files [here](https://www.math.uwaterloo.ca/tsp/world/countries.html).


### Lazy constrain ILP-formulation
run:
```bash
./target/release/Exercise05 'tsp-file' 'k' 1
```

# Experiments

to measure the total running time run:
```bash
time ./target/release/Exercise05 'Path_to_tsp_file' 'k' '0' > results.txt
```

than to get the total solve time (Sum of Time (CPU seconds)), rounds of ILP and subtour elimination constraints run:

```bash
grep "Time (CPU seconds):" results.txt | awk '{sum+=$NF} END {print "Sum of Time (CPU seconds):", sum}' && tail results.txt -n 10
```