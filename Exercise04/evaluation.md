# 1. General
* well done

# 2. Paper
## Abstract & Introduction
* you should have noted the computational complexity of TSP - it's NP-hard
* 2.2: instead of searching edge swaps by brute force, one could think about smarter strategies. Any ideas?

## Algorithm and Implementation
* pseudo-code would be nicer than describing the algorithm in prose
* 3.3: ok, you got your code running correctly. But could you derive ideas that improve the algorigthm? Like, finding smarter ways to find nearest-neighbor?


## Experimental Evaluation

* did you put a cap on the number of edge swaps? Or the running time?
* plots look nice, but:
* Figure1: on the y-scale you are mixing: number of nodes(?), running time and relative tour length. That's completely confusing.
* Better have some distinct plots. Or, even better, plots comparing one value among the three variants.
* Same with Figure2. The most interesting values (ratio) are hard to read.
* Graphics are really nice. Luxemburg being the most instructive case.

## Conclusion

> 2-Opt algorithm is able to escape local optima

Yes and no, I would say. It can escape tiny local optima, but in the whole, it still might get stuck in sub-optimal solution. After all, it's an heuristic, there's no guarantee to find the global optimum.

* combination of both heuristics is best. That's the main message. OK.

# 3. Code & Test Cases
* good readme; compilation and tests run OK
* I would liked to have more detailed output; run (some) specific cases
* tsp_algo.rs, line 25: you omitted sqrt(). That's smart, because the heuristics (nearest-neighbor, edge swap) still work. sqrt() is only needed to compute the total tour length.
  For the OPT-2 heuristic you will get different (but still reasonable) results.
* line 92: you seem to struggle with rounding errors close to 0.0.
  Why not just tolerate those cases?

# 4. Assessment
Pass

