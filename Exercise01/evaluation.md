# 1. General
* well done

# 2. Paper
## Introduction
* good
## Preliminaries
* "with an average time of O(n log n)" -- it is even an upper bound, i.e. it applies to all cases
* alright, but some chunks of pseudo-code would have been helpful, maybe

## Algorithm and Implementation
* good
* again, pseudo-code, or a diagram might be easier to read than prose.
* "finding out that recursion was sub optimal, as it forced us to close the Filereaders"
	 (but that's not the fault of recursion; only of your sub-optimal file handling)
	 good that you improved on it !

## Experimental Evaluation

* good methodology to get meaningful results
* more file sizes would have been nice
* why did't you try memory sizes > 200 M ?
  your largest test instance required just 2 mins; so it would have been interesting to run some bigger ones.

## Conclusion

* why was there little difference between in-memory and external-memory implementations?
* wouldn't you expect in-memory to be significantly faster?  
  (but most likely, instances were too small to notice the difference)
* what about the runtime complexity of EMM. It should also come close to O(n log n).
  Would your figures confirm that?


# 3. Code & Test Cases
* README easy to follow
  * please explain how to compile the sources
  * -Xmx100M causes OOM (too small, anyway)
  * but Xmx200M runs OK
* love your progress bars :-)
* ClassicalMergeSort.java
  * line 79,80: you create new buffers for each call (which is OK; GC cleans up the old ones)
  * but it would be more efficient to keep just **two** buffers. Merge from one to the other, then switch roles --> no more wasted memory.
* EMM.java
	* lots of comments, thank you
	* but try to use more expressive variable names 	
	* line 121: you loop over the file to skip contents; OK
	* but you could make use of the fact that all lines have fixed size; skip to the right location using  BufferedReader::skip()
	* line 111-276 is *huge* and hard to read. try at better structure, using subroutines.


# 4. Assessment
Pass

