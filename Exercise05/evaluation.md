# 1. General
* well done
* super-detailed results

# 2. Paper
## Abstract & Introduction
* 2.1. and 2.2 are very detailed, but contain some misconceptions:

> ... is modeled as a linear program where the variables represent
> the decision of whether to visit a particular city or
> not.

no, the variables correspond to **edges**. (it's correct in the formulas)

* you need constraints that forbid subtours (it's mentioned in 2.2)

> 2.2: such as visiting a city more than once or visiting cities in a circu
> lar fashion that does not return to the starting city

no, Constraint (2) makes sure that each vertex is visited once.

The subtour-elimination constraints forbid tours that are *too short*.

> However, there is no guarantee that the optimal solution will be found

Yes, there is. But it may require an exponential number of constraints (and is, thus, infeasible). Turns out that this worst-case is unlikely and only few 'lazy' constraints produce a solution in most cases.

## Algorithm and Implementation

* pseudo-code is far too detailed
* prose description is good
* 3.2 title should be  'Lazy' approach
* 3.3: the 'AE cycle' is already contained in the assigment  (like: full approach is too slow, improve it by using the 'lazy' approach)


## Experimental Evaluation

* you need not list all the running-times for all values of k. Just the largest k would suffice.
* Table 4,5,6: that's the interesting stuff: number of 'lazy' constraints and number of rounds. The 'relative' column is almost always close to 0, as the total number grows exponentially.
* so you ran the full approach with k <= 19 ? (which is fine)
* nice pictures

## Conclusion

* very detailed


# 3. Code & Test Cases
* installation worked ok (but took very long; what are you downloading ?)
* tests run fine, but I would have liked some bulk tests (with multiple files)

# 4. Assessment
Pass

