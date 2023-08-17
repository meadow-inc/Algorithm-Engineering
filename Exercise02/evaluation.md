# 1. General
* well done
* parallel merge and parallel re-arrange are missing

# 2. Paper
* two columns, please!

## Abstract & Introduction
* good
* "The parallel quick sort algorithm... but instead of using multiple threads, it uses a single thread" -- well, no. The point of parallel algorithms is that they use multiple threads!?

## Algorithm and Implementation
* "It requires a significant amount of memory to store the sub-arrays" -- not quite. Merge-Sort requires a buffer of the very same size as the input data.
* pseudo-code is a good idea, but far too detailed. Actually it's almost a 1-on-1 copy of the Java source code.
* two strategies are missing:
	* parallel merging in Merge-Sort
	* parallel re-arrangement in Quick-Sort

## Experimental Evaluation
* figures look quite good, but we want more words on the results !!
* Figure 5.2.1. (plot orientation is unusual, but well)
	* very nice speed-ups for both Merge-Sort and Quick-Sort up to 6 cores. 
	* with 7 cores running times grow again. Any ideas why that woul be the case?
* Figure 5.2.2. very plausible results. Can you make an educated guess at running time growth?   (should be like O(n log n)) 

## Conclusion
* oh, come on. Write more than one sentence !  (with lots of typos)


# 3. Code & Test Cases
* nice and detailed readme file
* for the test cases: why do I have to bother with input files? Better create input in-memory.
* give me some *useful* feedback during the tests !  
* have you thought about using JUnit for test cases ? (I mean, you are using it anyway)

* sequential Merge-Sort:
	* line 16,28: you create a temporary buffer for each call. Better: use just **two** buffers and copy back and forth (you can do that because calls operate on distinct ranges). Saves some allocations and copies.
	
* parallel Merge-Sort:
    *  using RecursiveAction as base class makes a lot of sense
    *  but: there's no use to split recursive calls alls the way down to the leaves of the call tree
    *  once all cores are busy with recursive calls, there's no benefit in creating ever more parallel tasks. At the deeper levels of recursion, it's better to fall back to the sequential implementation
    *  parallelisation of individual merge-steps is missing     

* parallel Quick-Sort:
	* same: it makes sense to fall back to sequential implementation once all cores are busy
	* parallelisation of re-arrangement is missing

# 4. Assessment
Pass

